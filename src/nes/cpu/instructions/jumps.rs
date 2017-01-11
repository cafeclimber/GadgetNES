//! For jump related instructions
#![allow(non_snake_case)]

use nes::cpu::Cpu;
use nes::cpu::Interrupt::BRK;
use nes::cpu::instructions::AddressingMode;
use nes::cpu::memory_map::{read_byte, read_word};
use nes::memory::Memory;

/// Jump and return instructions.
impl Cpu {
    /// BReak.
    pub fn BRK(&mut self, mem: &mut Memory) {
        self.interrupt(mem, BRK);
    }
    
    /// Jump to SubRoutine.
    pub fn JSR(&mut self, mem: &mut Memory) {
        let jump_target = read_word(mem, self.pc + 1);

        let addr_low = (self.pc + 2 & 0b1111_1111) as u8;
        let addr_high = ((self.pc + 2 & 0b1111_1111_0000_0000) >> 8) as u8;

        self.push_stack(mem, addr_high);
        self.push_stack(mem, addr_low);

        self.pc = jump_target;
    }

    /// JuMP to address.
    pub fn JMP(&mut self, mem: &mut Memory, addr_mode: AddressingMode) {
        let jump_target = match addr_mode {
            AddressingMode::Absolute => {
                let val = read_word(mem, self.pc + 1);
                val
            },
            // Has to deal with jumps at end of a page
            // Does not go to next addr, wraps @ page boundary
            // e.g. JMP($30FF) will get low byte from $30FF
            // and high byte from $3000 instead of $3100
            AddressingMode::Indirect => {
                let addr = read_word(mem, self.pc + 1);
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
                val
            },
            _ => panic!("Unsupported JMP: {:?}", addr_mode),
        };
        self.pc = jump_target;
    }


    /// ReTurn from Interrupt.
    pub fn RTI(&mut self, mem: &mut Memory) {
        self.p = self.pop_stack(mem);
        self.p = self.p | (1 << 5); // Bit 5 always on

        let addr_low = self.pop_stack(mem);
        let addr_high = self.pop_stack(mem);
        let ret_addr = (addr_high as u16) << 8 | (addr_low as u16);

        self.pc = ret_addr;
    }

    /// ReTurn from Subroutine.
    pub fn RTS(&mut self, mem: &mut Memory) {
        let addr_low = self.pop_stack(mem);
        let addr_high = self.pop_stack(mem);
        
        let ret_addr = (addr_high as u16) << 8 | (addr_low as u16);
        
        self.pc = ret_addr + 1;
    }
}
