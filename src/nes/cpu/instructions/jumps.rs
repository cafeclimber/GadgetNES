//! For jump related instructions
//! All instructions reutrn the number of cycles taken
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]

use nes::cpu::{Cpu, AddressingMode, StatusFlag};
use nes::cpu::memory_map::{read_byte, read_word};
use nes::memory::Memory;

impl Cpu {
    pub fn BRK(&mut self, mem: &mut Memory) {
        self.set_flag(StatusFlag::Break);
        let flags = self.p;
        self.push_stack(mem, flags);
        self.unset_flag(StatusFlag::Break); // B flag is only on in the stack
        let addr_low = (self.pc & 0b1111_1111) as u8;
        let addr_high = ((self.pc & 0b1111_1111_0000_0000) >> 8) as u8;
        self.push_stack(mem, addr_high);
        self.push_stack(mem, addr_low);
        self.pc = read_word(mem, 0xFFFE); // IRQ / BRK Vector
    }
    
    pub fn JSR(&mut self, mem: &mut Memory) {
        let jump_target = read_word(mem, self.pc + 1);
        print!(" JSR ${:X}   ", jump_target);
        print!("                   ");
        let addr_low = (self.pc + 2 & 0b1111_1111) as u8;
        let addr_high = ((self.pc + 2 & 0b1111_1111_0000_0000) >> 8) as u8;
        self.push_stack(mem, addr_high);
        self.push_stack(mem, addr_low);
        self.pc = jump_target;
    }

    pub fn JMP(&mut self, mem: &mut Memory, addr_mode: AddressingMode) {
        let jump_target = match addr_mode {
            AddressingMode::Absolute => {
                let val = read_word(mem, self.pc + 1);
                print!(" JMP ${:X}   ", val);
                print!("                   ");
                val
            },
            // Has to deal with jumps at end of a page
            // Does not go to next addr, wraps @ page boundary
            // e.g. JMP($30FF) will get low byte from $30FF
            // and high byte from $3000 instead of $3100
            // PRETTIFYME: Probably a better way to implement this
            AddressingMode::Indirect => {
                let addr = {
                    (read_byte(mem, self.pc + 1) as u16) |
                    (read_byte(mem, self.pc + 2) as u16) << 8
                };
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
                print!(" JMP (${:04X}) = {:04X} ", addr, val);
                print!("            ");
                val
            },
            _ => panic!("Unsupported JMP: {:?}", addr_mode),
        };
        self.pc = jump_target;
    }


    pub fn RTI(&mut self, mem: &mut Memory) {
        print!(" RTI         ");
        print!("                   ");
        self.p = self.pop_stack(mem);
        self.p = self.p | (1 << 5); // Bit 5 always on
        let addr_low = self.pop_stack(mem);
        let addr_high = self.pop_stack(mem);
        let ret_addr = (addr_high as u16) << 8 | (addr_low as u16);
        self.pc = ret_addr;
    }

    pub fn RTS(&mut self, mem: &mut Memory) {
        print!(" RTS         ");
        print!("                   ");
        let addr_low = self.pop_stack(mem);
        let addr_high = self.pop_stack(mem);
        let ret_addr = (addr_high as u16) << 8 | (addr_low as u16);
        self.pc = ret_addr + 1;
    }
}
