//! For register related instructions (increments / decrements of registers)
//! All instructions reutrn the number of cycles taken
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]

use nes::cpu::{Cpu, StatusFlag, Register};

impl Cpu {
    fn check_reg_flags(&mut self, reg: Register) {
        let check_reg = match reg {
            Register::A => self.a,
            Register::X => self.x,
            Register::Y => self.y,
        };
        
        if check_reg == 0 {
            self.set_flag(StatusFlag::Zero);
        } else {
            self.unset_flag(StatusFlag::Zero);
        }
        if check_reg & (1 << 7) != 0 {
            self.set_flag(StatusFlag::Negative);
        } else {
            self.unset_flag(StatusFlag::Negative);
        }
    }

    pub fn DEY(&mut self) {
        print!(" DEY         ");
        print!("                   ");
        self.y = self.y.wrapping_sub(1);
        self.check_reg_flags(Register::Y);
    }

    pub fn DEX(&mut self) {
        print!(" DEX         ");
        print!("                   ");
        self.x = self.x.wrapping_sub(1);
        self.check_reg_flags(Register::X);
    }

    pub fn INX(&mut self) {
        print!(" INX         ");
        print!("                   ");
        self.x = self.x.wrapping_add(1);
        self.check_reg_flags(Register::X);
    }

    pub fn INY(&mut self) {
        print!(" INY         ");
        print!("                   ");
        self.y = self.y.wrapping_add(1);
        self.check_reg_flags(Register::Y);
    }

    pub fn TAX(&mut self) {
        print!(" TAX         ");
        print!("                   ");
        self.x = self.a;
        self.check_reg_flags(Register::X);
    }

    pub fn TXA(&mut self) {
        print!(" TXA         ");
        print!("                   ");
        self.a = self.x;
        self.check_reg_flags(Register::A);
    }

    pub fn TAY(&mut self) {
        print!(" TAY         ");
        print!("                   ");
        self.y = self.a;
        self.check_reg_flags(Register::Y);
    }

    pub fn TYA(&mut self) {
        print!(" TYA         ");
        print!("                   ");
        self.a = self.y;
        self.check_reg_flags(Register::A);
    }
}
