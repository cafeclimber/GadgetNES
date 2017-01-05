//! This module provides an interface for the 6502 as used in the NES
use std::fmt;

pub mod instructions;
mod memory_map;

use nes::memory::Memory;
use self::instructions::{Instruction, decode, execute, AddressingMode};
use self::memory_map::{read_byte, write_byte, read_word};

const NMI_VECTOR: u16 = 0xFFFA;
const BRK_IRQ_VECTOR: u16 = 0xFFFE;

/// A *nearly* cycle-accurate representation of
/// the 6502 processor as used in the NES (ignores
/// branch cycles and page boundary crossings)
///
/// The CPU also contains a memory_map struct which provides an interface
/// For RAM, I/O, etc.
pub struct Cpu {
    pc: u16,
    sp: u8,
    p: u8,
    x: u8,
    y: u8,
    a: u8,

    pub cycle: u32,
}

// Registers used for flag checking. May change
enum Register {
    X,
    Y,
    A,
}

#[derive(Debug)]
pub enum Interrupt {
    BRK,
    IRQ,
    NMI,
}

/// Each of the flags in the status register.
enum StatusFlag {
    Carry      = 1 << 0,
    Zero       = 1 << 1,
    IntDisable = 1 << 2,
    Decimal    = 1 << 3,
    // Doesn't techincally exist but used when flags are pushed to the stack
    Break      = 1 << 4,
    Overflow   = 1 << 6,
    Negative   = 1 << 7,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            pc: 0x8000,
            sp: 0xFD, // Top of stack starts at end of Page 1 of RAM
            p: 0x24,
            x: 0,
            y: 0,
            a: 0,

            cycle: 0,
        }
    }

    /// Pushes `val` to stack.
    fn push_stack(&mut self, mem: &mut Memory, val: u8) {
        mem.write_ram_byte((self.sp as u16) + 0x100, val);
        self.sp -= 1;
    }

    /// Pops a value from the stack.
    fn pop_stack(&mut self, mem: &mut Memory) -> u8 {
        self.sp += 1;
        mem.read_ram_byte((self.sp as u16) + 0x100)
    }

    /// Sets `flag` to `set`. 
    fn set_flag(&mut self, flag: StatusFlag, set: bool) {
        match set {
            true => self.p |= flag as u8,
            false => self.p &= !(flag as u8),
        }
    }

    /// Checks if `flag` matches `is_set`
    ///
    /// #Examples
    /// ``` rust
    /// self.check_flag(StatusFlag::Carry, true) // checks if Carry flag is set
    /// self.check_flag(StatusFlag::Zero, false) // checks if Zero flag is not set
    /// ```
    fn check_flag(&self, flag: StatusFlag, is_set: bool) -> bool {
        match is_set {
            true => self.p & flag as u8 != 0,
            false => self.p & flag as u8 == 0,
        }
        
    }

    pub fn interrupt(&mut self, mem: &mut Memory, interrupt: Interrupt) {
        let vector = match interrupt {
            Interrupt::BRK => {
                // Check if IntDisable flag is set
                if self.check_flag(StatusFlag::IntDisable, true) { return; }
                self.set_flag(StatusFlag::Break, true);
                let flags = self.p;
                self.push_stack(mem, flags);
                self.set_flag(StatusFlag::Break, false); 
                BRK_IRQ_VECTOR
            },
            Interrupt::IRQ => {
                // Check if IntDisable flag is set
                if self.check_flag(StatusFlag::IntDisable, true) { return; }
                self.set_flag(StatusFlag::Break, false);
                let flags = self.p;
                self.push_stack(mem, flags);
                BRK_IRQ_VECTOR
            },
            Interrupt::NMI => {
                self.set_flag(StatusFlag::Break, false);
                let flags = self.p;
                self.push_stack(mem, flags);
                NMI_VECTOR
            },
        };
        let addr_low = (self.pc & 0b1111_1111) as u8;
        let addr_high = ((self.pc & 0b1111_1111_0000_0000) >> 8) as u8;
        self.push_stack(mem, addr_high);
        self.push_stack(mem, addr_low);
        #[cfg(feature="debug")]
        println!("\n!!!!!!!!!!!!!!!!!!!!!  Asserting {:?} interrupt with addr: {:#04X} !!!!!!!!!!!!!!!!!!!!!\n",
                 interrupt,
                 read_word(mem, vector));
        self.pc = read_word(mem, vector);
    }

    /// This is the primary operation of the CPU. It represents the
    /// execution of one instruction. Essentially, this function
    /// fetches the next instruction, decodes, then executes it.
    pub fn step(&mut self, mem: &mut Memory) {
        let op_code = read_byte(mem, self.pc);
        let (inst, addr_mode) = decode(self, op_code);

        #[cfg(feature="debug")]
        debug_print(&self, op_code, inst, mem, addr_mode);

        execute(self, mem, (inst, addr_mode));


        if inst != Instruction::JMP &&
           inst != Instruction::JSR &&
           inst != Instruction::RTS &&
           inst != Instruction::RTI
        {
            self.bump_pc(addr_mode); // Increment pc depending on addressing mode
        }
    }

    /// Increments PC depending on the addressing mode. 
    fn bump_pc(&mut self, addr_mode: AddressingMode) {
        let bump: u16 = match addr_mode {
            // Jumps handled above, branches set own PC, so don't change
            AddressingMode::Indirect => 0,
            AddressingMode::Relative => 0, 

            AddressingMode::Accumulator => 1,
            AddressingMode::Implied     => 1,

            AddressingMode::Immediate        => 2,
            AddressingMode::IndexedIndirect  => 2,
            AddressingMode::IndirectIndexed  => 2,
            AddressingMode::ZeroPage         => 2,
            AddressingMode::ZeroPageIndexedX => 2,
            AddressingMode::ZeroPageIndexedY => 2,

            AddressingMode::Absolute         => 3,
            AddressingMode::AbsoluteIndexedX => 3,
            AddressingMode::AbsoluteIndexedY => 3,
        };
        self.pc += bump;
    }

    /// Returns a byte from mapped memory.
    pub fn fetch_byte(&self,
                      mem: &mut Memory,
                      addr_mode: AddressingMode)
                      -> u8
    {
        match addr_mode {
            AddressingMode::Accumulator => self.a,
            AddressingMode::Immediate => read_byte(mem, self.pc + 1),
            _ => {
                let addr = self.get_addr(mem, addr_mode);
                read_byte(mem, addr)
            }
        }
    }

    /// Sets a byte in mapped memory. 
    /// 
    /// #Panics
    /// Panics if there is an attempt to make write with immediate mode.
    /// This should not be possible as no instructions write in this mode.
    pub fn set_byte(&mut self,
                    mem: &mut Memory,
                    addr_mode: AddressingMode,
                    val: u8)
    {
        match addr_mode {
            AddressingMode::Accumulator => self.a = val,
            AddressingMode::Immediate => {
                panic!("Immediate writes not supported");
            },
            _ => {
                let addr = self.get_addr(mem, addr_mode);
                write_byte(mem, addr, val);
            }
        }
    }

    /// Returns an addressing depending on the addressing mode passed to it.
    ///
    /// #Panics
    /// Panics on those modes which do not actually interact with memory, or
    /// those modes where this interaction is handled by the individual function.
    /// These are Implied, Indexed, and Relative modes.
    pub fn get_addr(&self,
                      mem: &mut Memory,
                      addr_mode: AddressingMode)
                      -> u16
    {
        match addr_mode {
            AddressingMode::ZeroPage => {
                read_byte(mem, self.pc + 1) as u16
            },
            AddressingMode::Absolute => {
                read_word(mem, self.pc + 1)
            },
            AddressingMode::IndexedIndirect => {
                let operand = read_byte(mem, self.pc + 1);
                let index = operand.wrapping_add(self.x);
                // Deals with zero-page wrapping
                (read_byte(mem, index as u16) as u16) |
                (read_byte(mem, index.wrapping_add(1) as u16) as u16) << 8
            },
            AddressingMode::IndirectIndexed => {
                let operand = read_byte(mem, self.pc + 1);
                // Deals with zero-page wrapping
                let addr = {
                    (read_byte(mem, operand as u16) as u16) |
                    (read_byte(mem, operand.wrapping_add(1) as u16) as u16) << 8
                };
                addr.wrapping_add(self.y as u16)
            },
            AddressingMode::ZeroPageIndexedX => {
                let addr = read_byte(mem, self.pc + 1);
                addr.wrapping_add(self.x) as u16
            },
            AddressingMode::ZeroPageIndexedY => {
                let addr = read_byte(mem, self.pc + 1);
                addr.wrapping_add(self.y) as u16
            },
            AddressingMode::AbsoluteIndexedX => {
                let addr = read_word(mem, self.pc + 1) as u16;
                addr.wrapping_add(self.x as u16)
            },
            AddressingMode::AbsoluteIndexedY => {
                let addr = read_word(mem, self.pc + 1) as u16;
                addr.wrapping_add(self.y as u16)
            },
            // Implied, Relative, Indexed
            _ => {
                panic!("Attemped to get_addr via unsupported mode: {:?}, {:?}",
                self.pc, addr_mode)
            }
        }
    }
}


// TODO: Fix this so as to not screw with PPU read/write privileges
#[cfg(feature="debug")]
// Could probably split up but don't really care, it's just printing for now...
// Probably a much better way to do this than readdressing memory, but....whateva
fn debug_print(cpu: &Cpu,
               op_code: u8,
               instr: Instruction,
               mem: &mut Memory,
               addr_mode: AddressingMode)
{
    print!("{:04X}  {:02X} {:?}",
           cpu.pc,
           op_code,
           instr);

    match addr_mode {
        AddressingMode::Implied => print!("                                   "),
        AddressingMode::Accumulator => print!(" A                                 "),
        AddressingMode::Relative => print!(" BRANCH                            "),
        AddressingMode::Absolute => print!(" ${:04X}                             ",
                                           cpu.get_addr(mem, addr_mode)),
        AddressingMode::AbsoluteIndexedX => print!(" ${:04X},X                     ",
                                                   cpu.get_addr(mem, addr_mode)),
        AddressingMode::AbsoluteIndexedY => print!(" ${:04X},Y                      ",
                                                   cpu.get_addr(mem, addr_mode)),
        AddressingMode::Immediate => print!(" #{:02X}                               ",
                                            cpu.fetch_byte(mem, addr_mode)),
        AddressingMode::Indirect => print!(" ($ADDR)"),
        AddressingMode::IndexedIndirect => print!(" (${:04X},X)                 ",
                                                  cpu.get_addr(mem, addr_mode)),
        AddressingMode::IndirectIndexed => print!(" (${:04X}),Y                         ",
                                                  cpu.get_addr(mem, addr_mode)),
        AddressingMode::ZeroPage => print!(" ${:02X}                               ",
                                           cpu.get_addr(mem, addr_mode)),
        AddressingMode::ZeroPageIndexedX => print!(" ${:02X},X                      ",
                                                   cpu.get_addr(mem, addr_mode)),
        AddressingMode::ZeroPageIndexedY => print!(" ${:02X},Y                        ",
                                                   cpu.get_addr(mem, addr_mode)),
    }

    println!("{:?}", cpu);
}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X}    CYC:{:5?}",
               self.a,
               self.x,
               self.y,
               self.p,
               self.sp,
               self.cycle)
    }
}
