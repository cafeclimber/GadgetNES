//! Instructions which interact with the stack.
#![allow(non_snake_case)]

use nes::cpu::{Cpu, StatusFlag};
use nes::memory::Memory;

/// Stack related instructions
impl Cpu {
    /// PusH Processor status.
    pub fn PHP(&mut self, mem: &mut Memory) {
        let val = self.p | (1 << 4); // By spec. Bit 4 is always set in this op
        self.push_stack(mem, val);
    }

    /// PuLl Processor status.
    pub fn PLP(&mut self, mem: &mut Memory) {
        self.p = self.pop_stack(mem) & 0b1110_1111; // Bit 4 is ignored
        self.p = self.p | (1 << 5); // Bit 5 must always be 1
    }

    /// PusH Accumulator.
    pub fn PHA(&mut self, mem: &mut Memory) {
        let val = self.a;
        self.push_stack(mem, val);
    }

    /// PuLl Accumulator.
    pub fn PLA(&mut self, mem: &mut Memory) {
        self.a = self.pop_stack(mem);

        if self.a == 0 {
            self.set_flag(StatusFlag::Zero, true);
        } else {
            self.set_flag(StatusFlag::Zero, false);
        }

        if self.a & (1 << 7) != 0 {
            self.set_flag(StatusFlag::Negative, true);
        } else {
            self.set_flag(StatusFlag::Negative, false);
        }
    }

    /// Transfer X register to Stack pointer.
    pub fn TXS(&mut self) {
        self.sp = self.x;
    }

    /// Transfer Stack pointer to X register.
    pub fn TSX(&mut self) {
        self.x = self.sp;

        if self.x == 0 {
            self.set_flag(StatusFlag::Zero, true);
        } else {
            self.set_flag(StatusFlag::Zero, false);
        }

        if self.x & (1 << 7) != 0 {
            self.set_flag(StatusFlag::Negative, true);
        } else {
            self.set_flag(StatusFlag::Negative, false);
        }
    }
}
