//! Register related increments and decrements.
#![allow(non_snake_case)]

use nes::cpu::Cpu;

/// Increment / decrement instructions for registers
impl Cpu {
    /// DEcrement X register.
    pub fn DEX(&mut self) {
        self.x = self.x.wrapping_sub(1);
        let x = self.x;
        self.set_zn_flags(x);
    }

    /// DEcrement Y register.
    pub fn DEY(&mut self) {
        self.y = self.y.wrapping_sub(1);
        let y = self.y;
        self.set_zn_flags(y);
    }

    /// INcrement X register.
    pub fn INX(&mut self) {
        self.x = self.x.wrapping_add(1);
        let x = self.x;
        self.set_zn_flags(x);
    }

    /// INcrement Y register.
    pub fn INY(&mut self) {
        self.y = self.y.wrapping_add(1);
        let y = self.y;
        self.set_zn_flags(y);
    }

    /// Transfer Accumulator to X register.
    pub fn TAX(&mut self) {
        self.x = self.a;
        let x = self.x;
        self.set_zn_flags(x);
    }

    /// Transfer X register to Accumulator .
    pub fn TXA(&mut self) {
        self.a = self.x;
        let a = self.a;
        self.set_zn_flags(a);
    }

    /// Transfer Accumulator to Y register.
    pub fn TAY(&mut self) {
        self.y = self.a;
        let y = self.y;
        self.set_zn_flags(y);
    }

    /// Transfer Y register to Accumulator .
    pub fn TYA(&mut self) {
        self.a = self.y;
        let a = self.a;
        self.set_zn_flags(a);
    }
}
