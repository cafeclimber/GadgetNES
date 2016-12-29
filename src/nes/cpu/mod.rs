//! This module provides an interface for the 6502 as used in the NES
use std::fmt;

pub mod instructions;
mod memory_map;

use nes::memory::Memory;
use self::instructions::{Instruction, decode, execute, AddressingMode};
use self::memory_map::{read_byte, write_byte, read_word};

/// The 6502 Processor
///
/// Contains 8 registers:
/// pc: Program Counter
/// sp: Stack Pointer
/// sr: Status Flags
/// x: Index
/// y: Index
/// a: Accumulator
///
/// Also contains a field which keeps track of the number of cycles.
/// This is useful for a number of reasons including accuracy and for rendering
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
}

// Registers used for flag checking. May change
enum Register {
    X,
    Y,
    A,
}

// Used for status flags
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
    pub fn init() -> Cpu {
        Cpu {
            pc: 0x8000,
            sp: 0xFD, // Top of stack starts at end of Page 1 of RAM
            p: 0x24,
            x: 0,
            y: 0,
            a: 0,
        }
    }

    // TODO: Check for overflows of stack page?
    fn push_stack(&mut self, mem: &mut Memory, val: u8) {
        mem.write_ram_byte((self.sp as u16) + 0x100, val);
        self.sp -= 1;
    }

    fn pop_stack(&mut self, mem: &mut Memory) -> u8 {
        self.sp += 1;
        let val = mem.read_ram_byte((self.sp as u16) + 0x100);
        val
    }

    fn set_flag(&mut self, flag: StatusFlag) {
        self.p |= flag as u8;
    }

    fn unset_flag(&mut self, flag: StatusFlag) {
        self.p &= !(flag as u8);
    }

    fn check_flag(&self, flag: StatusFlag) -> bool {
        self.p & flag as u8 != 0
    }

    fn check_no_flag(&self, flag: StatusFlag) -> bool {
        self.p & flag as u8 == 0
    }


    // fetches, decodes, and executes instruction printing state AFTER
    // running instruction
    pub fn step(&mut self, mem: &mut Memory) {
        print!("{:04X}", self.pc);

        let op_code = read_byte(mem, self.pc);
        print!("  {:02X}       ", op_code);

        let (inst, addr_mode) = decode(op_code);
        execute(self, mem, (inst, addr_mode));

        if inst != Instruction::JMP  &&
           inst != Instruction::JSR &&
           inst != Instruction::RTS &&
           inst != Instruction::RTI
        {
            self.bump_pc(addr_mode); // Increment pc depending on addressing mode
        }

        print!("{:?}\n", self);
    }

    pub fn bump_pc(&mut self, addr_mode: AddressingMode) {
        let bump: u16 = match addr_mode {
            // Only for jumps and branches which set pc
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

    // TODO: Better name?
    pub fn fetch_byte(&self,
                      mem: &Memory,
                      addr_mode: AddressingMode)
                      -> u8
    {
        match addr_mode {
            AddressingMode::Accumulator => {
                self.a
            }
            AddressingMode::Immediate => {
                print!(" #${:02X}    ", read_byte(mem, self.pc + 1));
                print!("                   ");
                read_byte(mem, self.pc + 1)
            },
            AddressingMode::ZeroPage => {
                print!(" ${:02X}     ", read_byte(mem, self.pc + 1));
                print!("                   ");
                let addr = read_byte(mem, self.pc + 1) as u16;
                read_byte(mem, addr)
            },
            AddressingMode::Absolute => {
                print!(" ${:04X}   ", read_word(mem, self.pc + 1));
                print!("                   ");
                let addr = read_word(mem, self.pc + 1);
                read_byte(mem, addr)
            },
            AddressingMode::IndexedIndirect => {
                print!(" (${:02X},X) @", read_byte(mem, self.pc + 1));
                let operand = read_byte(mem, self.pc + 1);
                let index = operand.wrapping_add(self.x);
                // Deals with zero-page wrapping
                let addr = {
                    (read_byte(mem, index as u16) as u16) |
                    (read_byte(mem, index.wrapping_add(1) as u16) as u16) << 8
                };
                let val = read_byte(mem, addr);
                print!(" {:02X} = {:04X} = {:02X}   ", index, addr, val);
                val
            },
            AddressingMode::IndirectIndexed => {
                print!(" (${:02X}),Y", read_byte(mem, self.pc + 1));
                let operand = read_byte(mem, self.pc + 1);
                // Deals with zero-page wrapping
                let mut addr = {
                    (read_byte(mem, operand as u16) as u16) |
                    (read_byte(mem, operand.wrapping_add(1) as u16) as u16) << 8
                };
                print!(" = {:04X}", addr);
                addr = addr.wrapping_add(self.y as u16);
                let val = read_byte(mem, addr);
                print!(" @ {:04X} = {:02X} ", addr, val);
                val
            },
            AddressingMode::ZeroPageIndexedX => {
                print!(" ${:02X},X", read_byte(mem, self.pc + 1));
                let mut addr = read_byte(mem, self.pc + 1);
                addr = addr.wrapping_add(self.x);
                print!(" @ {:02X} = {:02X}            ", addr,
                       read_byte(mem, addr as u16));
                read_byte(mem, addr as u16)
            },
            AddressingMode::ZeroPageIndexedY => {
                print!(" ${:02X},Y", read_byte(mem, self.pc + 1));
                let mut addr = read_byte(mem, self.pc + 1);
                addr = addr.wrapping_add(self.y);
                print!(" @ {:02X} = {:02X}            ", addr,
                       read_byte(mem, addr as u16));
                read_byte(mem, addr as u16)
            },
            AddressingMode::AbsoluteIndexedX => {
                print!(" ${:04X},X", read_word(mem, self.pc + 1));
                let mut addr = read_word(mem, self.pc + 1) as u16;
                addr = addr.wrapping_add(self.x as u16);
                let val = read_byte(mem, addr);
                print!(" @ {:04X} = {:02X}        ", addr, val);
                val
            },
            AddressingMode::AbsoluteIndexedY => {
                print!(" ${:04X},Y", read_word(mem, self.pc + 1));
                let mut addr = read_word(mem, self.pc + 1) as u16;
                addr = addr.wrapping_add(self.y as u16);
                let val = read_byte(mem, addr);
                print!(" @ {:04X} = {:02X}        ", addr, val);
                val
            },
            // Implied, Relative, Indexed
            _ => {
                panic!("Attemped to read via unsupported mode: {:?}, {:?}",
                self.pc, addr_mode)
            }
        }
    }

    // TODO: Better name?
    pub fn set_byte(&mut self,
                    mem: &mut Memory,
                    addr_mode: AddressingMode,
                    val: u8)
    {
        match addr_mode {
            AddressingMode::Accumulator => {
                print!(" A       ");
                print!("                   ");
                self.a = val;
            }
            AddressingMode::Immediate => {
                panic!("Writes for immediate mode not yet implemented");
            },
            AddressingMode::ZeroPage => {
                print!(" ${:02X}     ", read_byte(mem, self.pc + 1));
                print!("                   ");
                let addr = read_byte(mem, self.pc + 1) as u16;
                write_byte(mem, addr, val);
            },
            AddressingMode::Absolute => {
                print!(" ${:04X}   ", read_word(mem, self.pc + 1));
                print!("                   ");
                let addr = read_word(mem, self.pc + 1);
                write_byte(mem, addr, val);
            },
            AddressingMode::IndexedIndirect => {
                print!(" (${:02X}, X) @", read_byte(mem, self.pc + 1));
                let operand = read_byte(mem, self.pc + 1);
                let index = operand.wrapping_add(self.x);
                // Deals with zero-page wrapping
                let addr = {
                    (read_byte(mem, index as u16) as u16) |
                    (read_byte(mem, index.wrapping_add(1) as u16) as u16) << 8
                };
                print!(" {:02X} = {:04X} = {:02X}  ", index,
                       addr,
                       read_byte(mem, addr));
                write_byte(mem, addr, val);
            },
            AddressingMode::IndirectIndexed => {
                print!(" (${:02X}),Y", read_byte(mem, self.pc + 1));
                let operand = read_byte(mem, self.pc + 1);
                // Deals with zero-page wrapping
                let mut addr = {
                    (read_byte(mem, operand as u16) as u16) |
                    (read_byte(mem, operand.wrapping_add(1) as u16) as u16) << 8
                };
                print!(" = {:04X}", addr);
                addr = addr.wrapping_add(self.y as u16);
                print!(" @ {:04X} = {:02X} ", addr, val);
                write_byte(mem, addr, val);
            },
            AddressingMode::ZeroPageIndexedX => {
                print!(" ${:02X},X", read_byte(mem, self.pc + 1));
                let mut addr = read_byte(mem, self.pc + 1);
                addr = addr.wrapping_add(self.x);
                print!(" @ {:02X} = {:02X}            ", addr,
                       read_byte(mem, addr as u16));
                write_byte(mem, addr as u16, val);
            },
            AddressingMode::ZeroPageIndexedY => {
                print!(" ${:02X}, Y", read_byte(mem, self.pc + 1));
                let mut addr = read_byte(mem, self.pc + 1);
                addr = addr.wrapping_add(self.y);
                print!(" @ {:02X} = {:02X}      ", addr,
                       read_byte(mem, addr as u16));
                write_byte(mem, addr as u16, val);
            },
            AddressingMode::AbsoluteIndexedX => {
                print!(" ${:04X}, X", read_word(mem, self.pc + 1));
                let mut addr = read_word(mem, self.pc + 1) as u16;
                addr = addr.wrapping_add(self.x as u16);
                print!(" @ {:04X} = {:02X}       ", addr,
                       read_byte(mem, addr));
                write_byte(mem, addr, val);
            },
            AddressingMode::AbsoluteIndexedY => {
                print!(" ${:04X}, Y", read_word(mem, self.pc + 1));
                let mut addr = read_word(mem, self.pc + 1) as u16;
                addr = addr.wrapping_add(self.y as u16);
                print!(" @ {:04X} = {:02X}       ", addr,
                       read_byte(mem, addr));
                write_byte(mem, addr, val);
            },
            // Implied, Accumulator, Relative, Indexed, Immediate
            _ => {
                panic!("Attemped to write via unsupported mode: {:?}, {:?}",
                self.pc, addr_mode)
            }
        }
    }
}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X}",
               self.a,
               self.x,
               self.y,
               self.p,
               self.sp)
    }
}
