//! Unofficial instructions.
#![allow(non_snake_case)]

use nes::cpu::{Cpu, StatusFlag};
use nes::memory::Memory;
use super::AddressingMode;

/// Unofficial instructions.
impl Cpu {
    pub fn NOP_u(&self) {
    }

    pub fn LAX_u(&mut self, mem: &mut Memory, addr_mode: AddressingMode) {
        let val = self.fetch_byte(mem, addr_mode);
        self.a = val;
        self.x = val;

        // Since both are the same, only have to use one to check flags
        let a = self.a; 
        self.set_zn_flags(a);
    }

    pub fn SAX_u(&mut self, mem: &mut Memory, addr_mode: AddressingMode) {
        let val = self.a & self.x;
        self.set_byte(mem, addr_mode, val);
    }

    pub fn DCP_u(&mut self, mem: &mut Memory, addr_mode: AddressingMode) {
        // Equivalent to DEV value then CMP value
        // Flags are weird which is why INC isn't called directly
        let val = self.fetch_byte(mem, addr_mode);
        let result = val.wrapping_sub(1);
        self.set_byte(mem, addr_mode, result);

        let a = self.a;
        self.set_flag(StatusFlag::Zero, a == result);
        let val = a.wrapping_sub(result);
        self.set_flag(StatusFlag::Negative, val & (1 << 7) != 0);
    }

    pub fn ISC_u(&mut self, mem: &mut Memory, addr_mode: AddressingMode) {
        // Equivalent to INC value then SBC value
        // Flags are weird which is why INC isn't called directly
        let val = self.fetch_byte(mem, addr_mode);
        let result = val.wrapping_add(1);
        self.set_byte(mem, addr_mode, result);

        self.SBC(mem, addr_mode);
    }

    pub fn SLO_u(&mut self, mem: &mut Memory, addr_mode: AddressingMode) {
        // Equivalent to ASL value, then ORA value
        self.ASL(mem, addr_mode);
        self.ORA(mem, addr_mode);
    }

    pub fn RLA_u(&mut self, mem: &mut Memory, addr_mode: AddressingMode) {
        // Equivalent to ROL value then AND value
        self.ROL(mem, addr_mode);
        self.AND(mem, addr_mode);
    }

    pub fn SRE_u(&mut self, mem: &mut Memory, addr_mode: AddressingMode) {
        self.LSR(mem, addr_mode);
        self.EOR(mem, addr_mode);
    }

    pub fn RRA_u(&mut self, mem: &mut Memory, addr_mode: AddressingMode) {
        self.ROR(mem, addr_mode);
        self.ADC(mem, addr_mode);
    }
}
