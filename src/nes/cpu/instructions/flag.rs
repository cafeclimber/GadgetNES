//! For set/clear flag instructions.
#![allow(non_snake_case)]

use nes::cpu::{Cpu, StatusFlag};

/// Flag instructions
impl Cpu {
    /// CLear Carry.
    pub fn CLC(&mut self) {
        self.set_flag(StatusFlag::Carry, false);
    }

    /// SEt Carry.
    pub fn SEC(&mut self) {
        self.set_flag(StatusFlag::Carry, true);
    }

    /// CLear Interrupt.
    pub fn CLI(&mut self) {
        self.set_flag(StatusFlag::IntDisable, false);
    }

    /// SEt Interrupt.
    pub fn SEI(&mut self) {
        self.set_flag(StatusFlag::IntDisable, true);
    }

    /// CLear oVerflow.
    pub fn CLV(&mut self) {
        self.set_flag(StatusFlag::Overflow, false);
    }

    /// CLear Decimal.
    pub fn CLD(&mut self) {
        self.set_flag(StatusFlag::Decimal, false);
    }

    /// SEt Decimal.
    pub fn SED(&mut self) {
        self.set_flag(StatusFlag::Decimal, true);
    }
}
