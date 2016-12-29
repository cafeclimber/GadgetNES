//! For stack related instructions
//! All instructions reutrn the number of cycles taken
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]

use nes::cpu::{Cpu, StatusFlag};
use nes::memory::Memory;

impl Cpu {
    pub fn PHP(&mut self, mem: &mut Memory) {
        print!(" PHP         ");
        print!("                   ");
        let val = self.p | (1 << 4); // By spec. Bit 4 is always set in this op
        self.push_stack(mem, val);
    }

    pub fn PLP(&mut self, mem: &mut Memory) {
        print!(" PLP         ");
        print!("                   ");
        self.p = self.pop_stack(mem) & 0b1110_1111; // Bit 4 is ignored
        self.p = self.p | (1 << 5); // Bit 5 must always be 1
    }

    pub fn PHA(&mut self, mem: &mut Memory) {
        print!(" PHA         ");
        print!("                   ");
        let val = self.a;
        self.push_stack(mem, val);
    }

    pub fn PLA(&mut self, mem: &mut Memory) {
        print!(" PLA         ");
        print!("                   ");
        self.a = self.pop_stack(mem);

        if self.a == 0 {
            self.set_flag(StatusFlag::Zero);
        } else {
            self.unset_flag(StatusFlag::Zero);
        }

        if self.a & (1 << 7) != 0 {
            self.set_flag(StatusFlag::Negative);
        } else {
            self.unset_flag(StatusFlag::Negative);
        }
    }

    pub fn TXS(&mut self) {
        print!(" TXS         ");
        print!("                   ");
        self.sp = self.x;
    }

    pub fn TSX(&mut self) {
        print!(" TSX         ");
        print!("                   ");
        self.x = self.sp;

        if self.a == 0 {
            self.set_flag(StatusFlag::Zero);
        } else {
            self.unset_flag(StatusFlag::Zero);
        }

        if self.a & (1 << 7) != 0 {
            self.set_flag(StatusFlag::Negative);
        } else {
            self.unset_flag(StatusFlag::Negative);
        }
    }
}
