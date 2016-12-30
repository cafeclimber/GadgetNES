//! For register related instructions (increments / decrements of registers)
//! All instructions reutrn the number of cycles taken
#![allow(non_snake_case)]

use nes::cpu::{Cpu, StatusFlag, Register};

impl Cpu {
    fn check_reg_flags(&mut self, reg: Register) {
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

    pub fn DEY(&mut self) {
        self.y = self.y.wrapping_sub(1);
        self.check_reg_flags(Register::Y);
    }

    pub fn DEX(&mut self) {
        self.x = self.x.wrapping_sub(1);
        self.check_reg_flags(Register::X);
    }

    pub fn INX(&mut self) {
        self.x = self.x.wrapping_add(1);
        self.check_reg_flags(Register::X);
    }

    pub fn INY(&mut self) {
        self.y = self.y.wrapping_add(1);
        self.check_reg_flags(Register::Y);
    }

    pub fn TAX(&mut self) {
        self.x = self.a;
        self.check_reg_flags(Register::X);
    }

    pub fn TXA(&mut self) {
        self.a = self.x;
        self.check_reg_flags(Register::A);
    }

    pub fn TAY(&mut self) {
        self.y = self.a;
        self.check_reg_flags(Register::Y);
    }

    pub fn TYA(&mut self) {
        self.a = self.y;
        self.check_reg_flags(Register::A);
    }
}
