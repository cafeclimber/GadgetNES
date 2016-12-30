//! For memory related instructions (loading and storing)
//! All instructions reutrn the number of cycles taken
#![allow(non_snake_case)]

use nes::cpu::{Cpu, Register, StatusFlag};
use nes::memory::Memory;
use super::AddressingMode;

impl Cpu {
    fn load_check_flags(&mut self, reg: Register) {
        let check_reg = match reg {
            Register::A => self.a,
            Register::X => self.x,
            Register::Y => self.y,
        };
        
        if check_reg == 0 {
            self.set_flag(StatusFlag::Zero, true);
        } else {
            self.set_flag(StatusFlag::Zero, false);
        }
        if check_reg & (1 << 7) != 0 {
            self.set_flag(StatusFlag::Negative, true);
        } else {
            self.set_flag(StatusFlag::Negative, false);
        }
    }

    pub fn LDA(&mut self, mem: &Memory, addr_mode: AddressingMode) {
        let val = self.fetch_byte(mem, addr_mode);
        self.a = val;
        self.load_check_flags(Register::A)
    }

    pub fn LDX(&mut self, mem: &Memory, addr_mode: AddressingMode) {
        let val = self.fetch_byte(mem, addr_mode);
        self.x = val;
        self.load_check_flags(Register::X)
    }

    pub fn LDY(&mut self, mem: &Memory, addr_mode: AddressingMode) {
        let val = self.fetch_byte(mem, addr_mode);
        self.y = val;
        self.load_check_flags(Register::Y)
    }

    pub fn STA(&mut self, mem: &mut Memory, addr_mode: AddressingMode) {
        let val = self.a;
        self.set_byte(mem, addr_mode, val);
    }

    pub fn STX(&mut self, mem: &mut Memory, addr_mode: AddressingMode) {
        let val = self.x;
        self.set_byte(mem, addr_mode, val);
    }

    pub fn STY(&mut self, mem: &mut Memory, addr_mode: AddressingMode) { 
        let val = self.y;
        self.set_byte(mem, addr_mode, val);
    }
}