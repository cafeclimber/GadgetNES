//! For instructions related to setting and clearing of flags
//! All instructions reutrn the number of cycles taken
#![allow(non_snake_case)]

use nes::cpu::{Cpu, StatusFlag};

impl Cpu {
    pub fn CLC(&mut self) {
        self.set_flag(StatusFlag::Carry, false);
    }

    pub fn SEC(&mut self) {
        self.set_flag(StatusFlag::Carry, true);
    }

    pub fn CLI(&mut self) {
        self.set_flag(StatusFlag::IntDisable, false);
    }

    pub fn SEI(&mut self) {
        self.set_flag(StatusFlag::IntDisable, true);
    }

    pub fn CLV(&mut self) {
        self.set_flag(StatusFlag::Overflow, false);
    }

    pub fn CLD(&mut self) {
        self.set_flag(StatusFlag::Decimal, false);
    }

    pub fn SED(&mut self) {
        self.set_flag(StatusFlag::Decimal, true);
    }
}
