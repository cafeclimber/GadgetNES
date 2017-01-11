//! Arithmetic instructions
#![allow(non_snake_case)]

use nes::cpu::{Cpu, Register, StatusFlag};
use nes::memory::Memory;
use super::AddressingMode;

// PRETTIFYME: Abstract the similiar bits
/// ALU Instructions
impl Cpu {
    // Checks zero and negative flag. Individual functions are responsible
    // for others
    fn alu_check_flags(&mut self, reg: Register) {
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
    
    /// BIt Test.
    pub fn BIT(&mut self, mem: &mut Memory, addr_mode: AddressingMode) {
        let val = self.fetch_byte(mem, addr_mode);
        let result = val & self.a;

        if result == 0 {
            self.set_flag(StatusFlag::Zero, true);
        } else {
            self.set_flag(StatusFlag::Zero, false);
        }

        if val & (1 << 6) != 0 {
            self.set_flag(StatusFlag::Overflow, true);
        } else {
            self.set_flag(StatusFlag::Overflow, false);
        }

        if val & (1 << 7) != 0 {
            self.set_flag(StatusFlag::Negative, true);
        } else {
            self.set_flag(StatusFlag::Negative, false);
        }

    }

    /// OR with Accumulator.
    pub fn ORA(&mut self, mem: &mut Memory, addr_mode: AddressingMode) {
        let val = self.fetch_byte(mem, addr_mode);
        self.a = self.a | val;
        self.alu_check_flags(Register::A);
    }

    /// AND with accumulator.
    pub fn AND(&mut self, mem: &mut Memory, addr_mode: AddressingMode) {
        let val = self.fetch_byte(mem, addr_mode);
        self.a = self.a & val;
        self.alu_check_flags(Register::A);
    }

    /// Exclusive OR with accumulator.
    pub fn EOR(&mut self, mem: &mut Memory, addr_mode: AddressingMode) {
        let val = self.fetch_byte(mem, addr_mode);
        self.a = self.a ^ val;
        self.alu_check_flags(Register::A);
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
        self.alu_check_flags(Register::A);
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
        self.alu_check_flags(Register::A);
        self.set_flag(StatusFlag::Carry, set_carry);
        self.set_flag(StatusFlag::Overflow, set_overflow);
    }


    /// CoMPare accumulator.
    pub fn CMP(&mut self, mem: &mut Memory, addr_mode: AddressingMode) {
        let mut val = self.fetch_byte(mem, addr_mode);

        if self.a >= val {
            self.set_flag(StatusFlag::Carry, true);
        } else {
            self.set_flag(StatusFlag::Carry, false);
        }

        if self.a == val {
            self.set_flag(StatusFlag::Zero, true);
        } else {
            self.set_flag(StatusFlag::Zero, false);
        }

        val = (self.a).wrapping_sub(val);

        if val & (1 << 7) != 0 {
            self.set_flag(StatusFlag::Negative, true);
        } else {
            self.set_flag(StatusFlag::Negative, false);
        }
    }

    /// ComPare Y register.
    pub fn CPY(&mut self, mem: &mut Memory, addr_mode: AddressingMode) {
        let mut val = self.fetch_byte(mem, addr_mode);

        if self.y >= val {
            self.set_flag(StatusFlag::Carry, true);
        } else {
            self.set_flag(StatusFlag::Carry, false);
        }

        if self.y == val {
            self.set_flag(StatusFlag::Zero, true);
        } else {
            self.set_flag(StatusFlag::Zero, false);
        }

        val = (self.y).wrapping_sub(val);

        if val & (1 << 7) != 0 {
            self.set_flag(StatusFlag::Negative, true);
        } else {
            self.set_flag(StatusFlag::Negative, false);
        }
    }

    /// ComPare X register.
    pub fn CPX(&mut self, mem: &mut Memory, addr_mode: AddressingMode) {
        let mut val = self.fetch_byte(mem, addr_mode);

        if self.x >= val {
            self.set_flag(StatusFlag::Carry, true);
        } else {
            self.set_flag(StatusFlag::Carry, false);
        }

        if self.x == val {
            self.set_flag(StatusFlag::Zero, true);
        } else {
            self.set_flag(StatusFlag::Zero, false);
        }

        val = (self.x).wrapping_sub(val);

        if val & (1 << 7) != 0 {
            self.set_flag(StatusFlag::Negative, true);
        } else {
            self.set_flag(StatusFlag::Negative, false);
        }
    }

    /// Arithmetic Shift Left. 
    pub fn ASL(&mut self, mem: &mut Memory, addr_mode: AddressingMode) {
        let mut val = self.fetch_byte(mem, addr_mode);

        // Carry set to bit 7
        if val & (1 << 7) != 0 {
            self.set_flag(StatusFlag::Carry, true);
        } else {
            self.set_flag(StatusFlag::Carry, false);
        }

        val = val << 1;

        if val == 0 {
            self.set_flag(StatusFlag::Zero, true);
        } else {
            self.set_flag(StatusFlag::Zero, false);
        }

        if val & (1 << 7) != 0 {
            self.set_flag(StatusFlag::Negative, true);
        } else {
            self.set_flag(StatusFlag::Negative, false);
        }

        self.set_byte(mem, addr_mode, val);
    }

    /// Logical Shift Right.
    pub fn LSR(&mut self, mem: &mut Memory, addr_mode: AddressingMode) {
        let mut val = self.fetch_byte(mem, addr_mode);

        // Carry set to bit 0
        if val & (1 << 0) != 0 {
            self.set_flag(StatusFlag::Carry, true);
        } else {
            self.set_flag(StatusFlag::Carry, false);
        }

        val = val >> 1;

        if val == 0 {
            self.set_flag(StatusFlag::Zero, true);
        } else {
            self.set_flag(StatusFlag::Zero, false);
        }

        if val & (1 << 7) != 0 {
            self.set_flag(StatusFlag::Negative, true);
        } else {
            self.set_flag(StatusFlag::Negative, false);
        }

        self.set_byte(mem, addr_mode, val);
    }

    /// ROtate Left.
    pub fn ROL(&mut self, mem: &mut Memory, addr_mode: AddressingMode) {
        let mut val = self.fetch_byte(mem, addr_mode);
        let old_carry = self.check_flag(StatusFlag::Carry, true);

        // Carry set to bit 0
        if val & (1 << 7) != 0 {
            self.set_flag(StatusFlag::Carry, true);
        } else {
            self.set_flag(StatusFlag::Carry, false);
        }

        val = val << 1;

        if old_carry {
            val = val | (1 << 0);
        } else {
            val = val & !(1 << 0);
        }

        if val == 0 {
            self.set_flag(StatusFlag::Zero, true);
        } else {
            self.set_flag(StatusFlag::Zero, false);
        }

        if val & (1 << 7) != 0 {
            self.set_flag(StatusFlag::Negative, true);
        } else {
            self.set_flag(StatusFlag::Negative, false);
        }


        self.set_byte(mem, addr_mode, val);
    }

    /// ROtate Right
    pub fn ROR(&mut self, mem: &mut Memory, addr_mode: AddressingMode) {
        let mut val = self.fetch_byte(mem, addr_mode);
        let old_carry = self.check_flag(StatusFlag::Carry, true);

        // Carry set to bit 0
        if val & (1 << 0) != 0 {
            self.set_flag(StatusFlag::Carry, true);
        } else {
            self.set_flag(StatusFlag::Carry, false);
        }

        val = val >> 1;

        if old_carry {
            val = val | (1 << 7);
        } else {
            val = val & !(1 << 7);
        }

        if val == 0 {
            self.set_flag(StatusFlag::Zero, true);
        } else {
            self.set_flag(StatusFlag::Zero, false);
        }

        if val & (1 << 7) != 0 {
            self.set_flag(StatusFlag::Negative, true);
        } else {
            self.set_flag(StatusFlag::Negative, false);
        }


        self.set_byte(mem, addr_mode, val);
    }
}
