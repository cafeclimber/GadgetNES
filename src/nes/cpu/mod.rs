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
        let addr_low = (self.pc & 0b1111_1111) as u8;
        let addr_high = ((self.pc & 0b1111_1111_0000_0000) >> 8) as u8;
        self.push_stack(mem, addr_high);
        self.push_stack(mem, addr_low);
        let vector = match interrupt {
            Interrupt::BRK => {
                self.set_flag(StatusFlag::Break, true);
                let flags = self.p;
                self.push_stack(mem, flags);
                self.set_flag(StatusFlag::Break, false); 
                BRK_IRQ_VECTOR
            },
            Interrupt::IRQ => {
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
        println!("\n!!!!!!!!!!!!!!!!!!  Asserting {:?} interrupt with vector: {:#04X} !!!!!!!!!!!!!!!!!!!\n",
                 interrupt,
                 vector);
        self.pc = vector;
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
                      mem: &Memory,
                      addr_mode: AddressingMode)
                      -> u8
    {
        match addr_mode {
            AddressingMode::Accumulator => self.a,
            AddressingMode::Immediate => read_byte(mem, self.pc + 1),
            _ => read_byte(mem, self.get_addr(mem, addr_mode))
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
                      mem: &Memory,
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


#[cfg(feature="debug")]
// Could probably split up but don't really care, it's just printing for now...
// Probably a much better way to do this than readdressing memory, but....whateva
fn debug_print(cpu: &Cpu,
               op_code: u8,
               instr: Instruction,
               mem: &Memory,
               addr_mode: AddressingMode)
{
    print!("{:04X}  {:02X}", cpu.pc, op_code);

    use self::instructions::AddressingMode::*;
    match addr_mode {
        Implied => {
            print!("        {:?}                            ",
                   instr);
        },
        Accumulator => {
            print!("        {:?} A                          ",
                   instr);
        },
        ZeroPage => {
            print!(" {0:02X}     {1:?} ${0:02X} = {2:02X}                   ",
                   read_byte(mem, cpu.pc + 1),
                   instr,
                   read_byte(mem, read_byte(mem, cpu.pc + 1) as u16));
        },
        Relative => {
            let offset = read_byte(mem, cpu.pc + 1) as i8;
            let branch_target = (((cpu.pc + 2) as i32) + (offset as i32)) as u16;
            print!(" {0:02X}     {1:?} ${2:04X}                      ",
                   read_byte(mem, cpu.pc + 1),
                   instr,
                   branch_target);
        },
        Absolute => {
            if instr == Instruction::JMP  || instr == Instruction::JSR {
                print!(" {0:02X} {1:02X}  {2:?} ${1:02X}{0:02X}                      ",
                    read_byte(mem, cpu.pc + 1),
                    read_byte(mem, cpu.pc + 2),
                    instr);
            } else {
                print!(" {0:02X} {1:02X}  {2:?} ${1:02X}{0:02X} = {3:02X}                 ",
                       read_byte(mem, cpu.pc + 1),
                       read_byte(mem, cpu.pc + 2),
                       instr,
                       read_byte(mem, read_word(mem, cpu.pc + 1)));
            }
        },
        Indirect => {
            let addr = read_word(mem, cpu.pc + 1);
            let val = {
                if addr & 0xFF == 0xFF {
                    (read_byte(mem, addr) as u16) |
                    // keep upper byte and make low byte 0
                    (read_byte(mem, addr & 0xFF00) as u16) << 8
                } else {
                    (read_byte(mem, addr) as u16) |
                    (read_byte(mem, addr + 1) as u16) << 8
                }
            };
            print!(" {0:02X} {1:02X}  {2:?} (${3:04X}) = {4:04X}             ",
                   read_byte(mem, cpu.pc + 1),
                   read_byte(mem, cpu.pc + 2),
                   instr,
                   read_word(mem, cpu.pc + 1),
                   val);
        },
        Immediate => {
            print!(" {0:02X}     {1:?} #${0:02X}                       ",
                   read_byte(mem, cpu.pc + 1),
                   instr,
            );
        },
        IndexedIndirect => {
            let operand = read_byte(mem, cpu.pc + 1);
            let index = operand.wrapping_add(cpu.x);
            // Deals with zero-page wrapping
            let addr = {
                (read_byte(mem, index as u16) as u16) |
                (read_byte(mem, index.wrapping_add(1) as u16) as u16) << 8
            };
            let val = read_byte(mem, addr);
            print!(" {0:02X}     {1:?} (${0:02X},X) @ {2:02X} = {3:04X} = {4:02X}   ",
                   operand,
                   instr,
                   index,
                   addr,
                   val);
        },
        IndirectIndexed => {
            let operand = read_byte(mem, cpu.pc + 1);
            // Deals with zero-page wrapping
            let addr = {
                (read_byte(mem, operand as u16) as u16) |
                (read_byte(mem, operand.wrapping_add(1) as u16) as u16) << 8
            };
            let indexed_addr = addr.wrapping_add(cpu.y as u16);
            let val = read_byte(mem, indexed_addr);
            print!(" {0:02X}     {1:?} (${0:02X}),Y = {2:04X} @ {3:04X} = {4:02X} ",
                   operand,
                   instr,
                   addr,
                   indexed_addr,
                   val);
        },
        ZeroPageIndexedX => {
            let addr = read_byte(mem, cpu.pc + 1);
            let indexed_addr = addr.wrapping_add(cpu.x);
            let val = read_byte(mem, indexed_addr as u16);
            print!(" {0:02X}     {1:?} ${2:02X},X @ {3:02X} = {4:02X}            ",
                   read_byte(mem, cpu.pc + 1),
                   instr,
                   addr,
                   indexed_addr,
                   val);
        },
        ZeroPageIndexedY => {
            let addr = read_byte(mem, cpu.pc + 1);
            let indexed_addr = addr.wrapping_add(cpu.y);
            let val = read_byte(mem, indexed_addr as u16);
            print!(" {0:02X}     {1:?} ${2:02X},Y @ {3:02X} = {4:02X}            ",
                   read_byte(mem, cpu.pc + 1),
                   instr,
                   addr,
                   indexed_addr,
                   val);
        },
        AbsoluteIndexedX => {
            let addr = read_word(mem, cpu.pc + 1) as u16;
            let indexed_addr = addr.wrapping_add(cpu.x as u16);
            let val = read_byte(mem, indexed_addr);
            print!(" {0:02X} {1:02X}  {2:?} ${3:04X},X @ {4:04X} = {5:02X}        ",
                   read_byte(mem, cpu.pc + 1),
                   read_byte(mem, cpu.pc + 2),
                   instr,
                   addr,
                   indexed_addr,
                   val);
        },
        AbsoluteIndexedY => {
            let addr = read_word(mem, cpu.pc + 1) as u16;
            let indexed_addr = addr.wrapping_add(cpu.y as u16);
            let val = read_byte(mem, indexed_addr);
            print!(" {0:02X} {1:02X}  {2:?} ${3:04X},Y @ {4:04X} = {5:02X}        ",
                   read_byte(mem, cpu.pc + 1),
                   read_byte(mem, cpu.pc + 2),
                   instr,
                   addr,
                   indexed_addr,
                   val);
        },
    }
    print!("{:?}\n", cpu);
}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X} CYC:{:5?}",
               self.a,
               self.x,
               self.y,
               self.p,
               self.sp,
               self.cycle)
    }
}
