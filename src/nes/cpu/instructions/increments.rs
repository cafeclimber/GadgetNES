//! For increment related instructions
//! All instructions reutrn the number of cycles taken
#![allow(non_snake_case)]

use nes::cpu::{Cpu, StatusFlag};
use nes::memory::Memory;
use super::AddressingMode;

impl Cpu {
    fn check_inc_flags(&mut self, val: u8) {
        if val == 0 {
            self.set_flag(StatusFlag::Zero, true);
        } else {
            self.set_flag(StatusFlag::Zero, false);
        }

        if val & (1 << 7) != 0 {
            self.set_flag(StatusFlag::Negative, true);
        } else {
            self.set_flag(StatusFlag::Negative, false);
        }
    }

    pub fn DEC(&mut self, mem: &mut Memory, addr_mode: AddressingMode) {
        let val = self.fetch_byte(mem, addr_mode);
        let result = val.wrapping_sub(1);

        self.check_inc_flags(result);
        
        self.set_byte(mem, addr_mode, result);
    }

    pub fn INC(&mut self, mem: &mut Memory, addr_mode: AddressingMode) {
        let val = self.fetch_byte(mem, addr_mode);
        let result = val.wrapping_add(1);

        self.check_inc_flags(result);
        
        self.set_byte(mem, addr_mode, val.wrapping_add(1));
    }
}
