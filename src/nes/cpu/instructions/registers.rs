//! Register related increments and decrements.
#![allow(non_snake_case)]

use nes::cpu::{Cpu, StatusFlag, Register};

/// Increment / decrement instructions for registers
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

    /// DEcrement Y register.
    pub fn DEY(&mut self) {
        self.y = self.y.wrapping_sub(1);
        self.check_reg_flags(Register::Y);
    }

    /// DEcrement X register.
    pub fn DEX(&mut self) {
        self.x = self.x.wrapping_sub(1);
        self.check_reg_flags(Register::X);
    }

    /// INcrement X register.
    pub fn INX(&mut self) {
        self.x = self.x.wrapping_add(1);
        self.check_reg_flags(Register::X);
    }

    /// INcrement Y register.
    pub fn INY(&mut self) {
        self.y = self.y.wrapping_add(1);
        self.check_reg_flags(Register::Y);
    }

    /// Transfer Accumulator to X register.
    pub fn TAX(&mut self) {
        self.x = self.a;
        self.check_reg_flags(Register::X);
    }

    /// Transfer X register to Accumulator .
    pub fn TXA(&mut self) {
        self.a = self.x;
        self.check_reg_flags(Register::A);
    }

    /// Transfer Accumulator to Y register.
    pub fn TAY(&mut self) {
        self.y = self.a;
        self.check_reg_flags(Register::Y);
    }

    /// Transfer Y register to Accumulator .
    pub fn TYA(&mut self) {
        self.a = self.y;
        self.check_reg_flags(Register::A);
    }
}
