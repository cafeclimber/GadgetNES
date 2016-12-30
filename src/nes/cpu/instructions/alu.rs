//! For alu related instructions
//! All instructions reutrn the number of cycles taken
#![allow(non_snake_case)]

use nes::cpu::{Cpu, Register, StatusFlag};
use nes::memory::Memory;
use super::AddressingMode;

// PRETTIFYME: Abstract the similiar bits
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
    
    pub fn BIT(&mut self, mem: &Memory, addr_mode: AddressingMode) {
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

    pub fn ORA(&mut self, mem: &Memory, addr_mode: AddressingMode) {
        let val = self.fetch_byte(mem, addr_mode);
        self.a = self.a | val;
        self.alu_check_flags(Register::A);
    }

    pub fn AND(&mut self, mem: &Memory, addr_mode: AddressingMode) {
        let val = self.fetch_byte(mem, addr_mode);
        self.a = self.a & val;
        self.alu_check_flags(Register::A);
    }

    pub fn EOR(&mut self, mem: &Memory, addr_mode: AddressingMode) {
        let val = self.fetch_byte(mem, addr_mode);
        self.a = self.a ^ val;
        self.alu_check_flags(Register::A);
    }

    pub fn ADC(&mut self, mem: &Memory, addr_mode: AddressingMode) {
        let val = self.fetch_byte(mem, addr_mode);
        let sum = val.wrapping_add(self.a + {
            if self.check_flag(StatusFlag::Carry, true) {
                1
            } else {
                0
            }});

        if sum < val {
            self.set_flag(StatusFlag::Carry, true);
        } else {
            self.set_flag(StatusFlag::Carry, false);
        }

        if sum & (1 << 7) != 0 {
            self.set_flag(StatusFlag::Negative, true);
        } else {
            self.set_flag(StatusFlag::Negative, false);
        }

        if sum == 0 {
            self.set_flag(StatusFlag::Zero, true);
        } else {
            self.set_flag(StatusFlag::Zero, false);
        }

        if self.a & (1 << 7) == 0 &&
            val & (1 << 7) == 0 // both positive
        {
            if sum & (1 << 7) != 0 {
                self.set_flag(StatusFlag::Overflow, true);
            } else {
                self.set_flag(StatusFlag::Overflow, false);
            }
        } else if self.a & (1 << 7) != 0 &&
                   val & (1 << 7) != 0 // both negative
        {
            if sum & (1 << 7) == 0 {
                self.set_flag(StatusFlag::Overflow, true);
            } else {
                self.set_flag(StatusFlag::Overflow, false);
            }
        } else {
            self.set_flag(StatusFlag::Overflow, false);
        }

        self.a = sum;
    }

    pub fn CMP(&mut self, mem: &Memory, addr_mode: AddressingMode) {
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

    pub fn CPY(&mut self, mem: &Memory, addr_mode: AddressingMode) {
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

    pub fn CPX(&mut self, mem: &Memory, addr_mode: AddressingMode) {
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

    // PRETTIFYME: Jesus....
    // TODO: Check this carefully!
    pub fn SBC(&mut self, mem: &Memory, addr_mode: AddressingMode) {
        let val = self.fetch_byte(mem, addr_mode);
        let diff = self.a.wrapping_sub(val) - (1 - {
            if self.check_flag(StatusFlag::Carry, true) {
                1
            } else {
                0
            }
        });

        if val > self.a {
            self.set_flag(StatusFlag::Carry, false);
        } else {
            self.set_flag(StatusFlag::Carry, true);
        }

        if diff & (1 << 7) != 0 {
            self.set_flag(StatusFlag::Negative, true);
        } else {
            self.set_flag(StatusFlag::Negative, false);
        }

        if diff == 0 {
            self.set_flag(StatusFlag::Zero, true);
        } else {
            self.set_flag(StatusFlag::Zero, false);
        }

        if self.a & (1 << 7) == 0 &&
            val & (1 << 7) != 0 // first positive second negative 
        {
            if diff & (1 << 7) != 0 { // result negative
                self.set_flag(StatusFlag::Overflow, true);
            } else {
                self.set_flag(StatusFlag::Overflow, false);
            }
        } else if self.a & (1 << 7) != 0 &&
                   val & (1 << 7) == 0 // first negative second positive
        {
            if diff & (1 << 7) == 0 { // result positive
                self.set_flag(StatusFlag::Overflow, true);
            } else {
                self.set_flag(StatusFlag::Overflow, false);
            }
        } else {
            self.set_flag(StatusFlag::Overflow, false);
        }

        self.a = diff;
    }

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
