//! For instructions related to setting and clearing of flags
//! All instructions reutrn the number of cycles taken
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]

use nes::cpu::{Cpu, StatusFlag};

impl Cpu {
    pub fn CLC(&mut self) {
        print!(" CLC         ");
        print!("                   ");
        self.unset_flag(StatusFlag::Carry);
    }

    pub fn SEC(&mut self) {
        print!(" SEC         ");
        print!("                   ");
        self.set_flag(StatusFlag::Carry);
    }

    pub fn CLI(&mut self) {
        print!(" CLI         ");
        print!("                   ");
        self.set_flag(StatusFlag::Carry);
    }

    pub fn SEI(&mut self) {
        print!(" SEI         ");
        print!("                   ");
        self.set_flag(StatusFlag::IntDisable);
    }

    pub fn CLV(&mut self) {
        print!(" CLV         ");
        print!("                   ");
        self.unset_flag(StatusFlag::Overflow);
    }

    pub fn CLD(&mut self) {
        print!(" CLD         ");
        print!("                   ");
        self.unset_flag(StatusFlag::Decimal);
    }

    pub fn SED(&mut self) {
        print!(" SED         ");
        print!("                   ");
        self.set_flag(StatusFlag::Decimal);
    }
}
