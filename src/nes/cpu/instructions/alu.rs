//! Arithmetic instructions
#![allow(non_snake_case)]

use nes::cpu::{Cpu, Register, StatusFlag};
use nes::memory::Memory;
use super::AddressingMode;

// PRETTIFYME: Abstract the similiar bits
/// ALU Instructions
impl Cpu {
    /// BIt Test.
    pub fn BIT(&mut self, mem: &mut Memory, addr_mode: AddressingMode) {
        let val = self.fetch_byte(mem, addr_mode);
        let result = val & self.a;

        self.set_flag(StatusFlag::Zero, result == 0);
        self.set_flag(StatusFlag::Overflow, val & (1 << 6) != 0);
        self.set_flag(StatusFlag::Negative, val & (1 << 7) != 0);
    }

    /// OR with Accumulator.
    pub fn ORA(&mut self, mem: &mut Memory, addr_mode: AddressingMode) {
        let val = self.fetch_byte(mem, addr_mode);
        self.a = self.a | val;
        let a = self.a;
        self.set_zn_flags(a);
    }

    /// AND with accumulator.
    pub fn AND(&mut self, mem: &mut Memory, addr_mode: AddressingMode) {
        let val = self.fetch_byte(mem, addr_mode);
        self.a = self.a & val;
        let a = self.a;
        self.set_zn_flags(a);
    }

    /// Exclusive OR with accumulator.
    pub fn EOR(&mut self, mem: &mut Memory, addr_mode: AddressingMode) {
        let val = self.fetch_byte(mem, addr_mode);
        self.a = self.a ^ val;
        let a = self.a;
        self.set_zn_flags(a);
    }

    /// ADd with Carry
    pub fn ADC(&mut self, mem: &mut Memory, addr_mode: AddressingMode) {
        // See StackOverflow question 29193303
        let val = self.fetch_byte(mem, addr_mode) as u16;
        let a = self.a as u16;
        let sum = val + a + (self.p & 0b1) as u16;
        let set_carry = sum > 0xFF;
        let set_overflow = (!(a ^ val) & (a ^ sum) & 0x80) != 0; 

        let sum = (val as u8).wrapping_add(self.a).wrapping_add(self.p & 0b1);

        self.a = sum;
        let a = self.a;
        self.set_zn_flags(a);
        self.set_flag(StatusFlag::Carry, set_carry);
        self.set_flag(StatusFlag::Overflow, set_overflow);
    }

    // PRETTIFYME: Jesus....
    // TODO: Check this carefully!
    /// SuBtract with Carry
    pub fn SBC(&mut self, mem: &mut Memory, addr_mode: AddressingMode) {
        // See StackOverflow question 29193303
        let val = !(self.fetch_byte(mem, addr_mode)) as u16;
        let a = self.a as u16;
        let diff = val + a + (self.p & 0b1) as u16;
        let set_carry = diff > 0xFF;
        let set_overflow = (!(a ^ val) & (a ^ diff) & 0x80) != 0; 

        let diff = (val as u8).wrapping_add(self.a).wrapping_add(self.p & 0b1);

        self.a = diff;
        let a = self.a;
        self.set_zn_flags(a);
        self.set_flag(StatusFlag::Carry, set_carry);
        self.set_flag(StatusFlag::Overflow, set_overflow);
    }


    /// CoMPare accumulator.
    pub fn CMP(&mut self, mem: &mut Memory, addr_mode: AddressingMode) {
        let mut val = self.fetch_byte(mem, addr_mode);

        let a = self.a;
        self.set_flag(StatusFlag::Carry, a >= val);
        self.set_flag(StatusFlag::Zero, a == val);

        val = a.wrapping_sub(val);
        self.set_flag(StatusFlag::Negative, val & (1 << 7) != 0);
    }

    /// ComPare Y register.
    pub fn CPY(&mut self, mem: &mut Memory, addr_mode: AddressingMode) {
        let mut val = self.fetch_byte(mem, addr_mode);

        let y = self.y;
        self.set_flag(StatusFlag::Carry, y >= val);
        self.set_flag(StatusFlag::Zero, y == val);

        val = (self.y).wrapping_sub(val);
        self.set_flag(StatusFlag::Negative, val & (1 << 7) != 0);
    }

    /// ComPare X register.
    pub fn CPX(&mut self, mem: &mut Memory, addr_mode: AddressingMode) {
        let mut val = self.fetch_byte(mem, addr_mode);

        let x = self.x;
        self.set_flag(StatusFlag::Carry, x >= val);
        self.set_flag(StatusFlag::Zero, x == val);

        val = (self.x).wrapping_sub(val);
        self.set_flag(StatusFlag::Negative, val & (1 << 7) != 0);
    }

    /// Arithmetic Shift Left. 
    pub fn ASL(&mut self, mem: &mut Memory, addr_mode: AddressingMode) {
        let mut val = self.fetch_byte(mem, addr_mode);

        // Carry set to bit 7
        self.set_flag(StatusFlag::Carry, val & (1 << 7) != 0);

        val = val << 1;

        self.set_flag(StatusFlag::Zero, val == 0);
        self.set_flag(StatusFlag::Negative, val & (1 << 7) != 0);

        self.set_byte(mem, addr_mode, val);
    }

    /// Logical Shift Right.
    pub fn LSR(&mut self, mem: &mut Memory, addr_mode: AddressingMode) {
        let mut val = self.fetch_byte(mem, addr_mode);

        // Carry set to bit 0
        self.set_flag(StatusFlag::Carry, val & (1 << 0) != 0);

        val = val >> 1;

        self.set_flag(StatusFlag::Zero, val == 0);
        self.set_flag(StatusFlag::Negative, val & (1 << 7) != 0);

        self.set_byte(mem, addr_mode, val);
    }

    /// ROtate Left.
    pub fn ROL(&mut self, mem: &mut Memory, addr_mode: AddressingMode) {
        let mut val = self.fetch_byte(mem, addr_mode);
        let old_carry = self.check_flag(StatusFlag::Carry, true);

        // Carry set to bit 0
        self.set_flag(StatusFlag::Carry, val & (1 << 7) != 0);

        val = val << 1;

        if old_carry {
            val = val | (1 << 0);
        } else {
            val = val & !(1 << 0);
        }

        self.set_flag(StatusFlag::Zero, val == 0);
        self.set_flag(StatusFlag::Negative, val & (1 << 7) != 0);

        self.set_byte(mem, addr_mode, val);
    }

    /// ROtate Right
    pub fn ROR(&mut self, mem: &mut Memory, addr_mode: AddressingMode) {
        let mut val = self.fetch_byte(mem, addr_mode);
        let old_carry = self.check_flag(StatusFlag::Carry, true);

        // Carry set to bit 0
        self.set_flag(StatusFlag::Carry, val & (1 << 0) != 0);

        val = val >> 1;

        if old_carry {
            val = val | (1 << 7);
        } else {
            val = val & !(1 << 7);
        }

        self.set_flag(StatusFlag::Zero, val == 0);
        self.set_flag(StatusFlag::Negative, val & (1 << 7) != 0);

        self.set_byte(mem, addr_mode, val);
    }
}
