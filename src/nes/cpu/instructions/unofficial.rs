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

    pub fn ALR_u(&mut self, mem: &mut Memory, addr_mode: AddressingMode) {
        self.AND(mem, addr_mode);
        self.LSR(mem, AddressingMode::Implied);
    }

    pub fn ANC_u(&mut self, mem: &mut Memory, addr_mode: AddressingMode) {
        self.AND(mem, addr_mode);
        let n = self.a & (1 << 7) != 0;
        self.set_flag(StatusFlag::Carry, n);
    }

    pub fn ARR_u(&mut self, mem: &mut Memory, addr_mode: AddressingMode) {
        let val= self.fetch_byte(mem, addr_mode);
        self.a = self.a & val;
        self.a = self.a << 1;
        let a = self.a;
        self.set_zn_flags(a);
        self.set_flag(StatusFlag::Carry, a & (1 << 6) != 0);
        self.set_flag(StatusFlag::Overflow,
                      (a & (1 << 6) >> 6) ^ (a & (1 << 5) >> 5) != 0);
    }

    pub fn AXS_u(&mut self, mem: &mut Memory, addr_mode: AddressingMode) {
        let val = self.fetch_byte(mem, addr_mode);
        let a = self.a;
        let x = self.x;
        let result = (a & x).wrapping_sub(val);
        self.set_zn_flags(result);
        self.set_flag(StatusFlag::Carry, (a & x) < val);
    }

    pub fn AXA_u(&mut self, mem: &mut Memory, addr_mode: AddressingMode) {
        let a = self.a;
        let x = self.x;
        let val = (x & a) & 7;
        self.set_byte(mem, addr_mode, val);
    }

    pub fn XAS_u(&mut self, mem: &mut Memory, addr_mode: AddressingMode) {
        let a = self.a;
        let x = self.x;
        self.sp = x & a;
        let sp = self.sp;
        let addr = (((self.get_addr(mem, addr_mode)) & 0xF0) >> 8) as u8;
        let result = sp & (addr.wrapping_add(1));
        self.set_byte(mem, addr_mode, result);
    }

    pub fn SYA_u(&mut self, mem: &mut Memory, addr_mode: AddressingMode) {
        let y = self.y;
        let addr = (((self.get_addr(mem, addr_mode)) & 0xF0) >> 8) as u8;
        let result = y & (addr.wrapping_add(1));
        self.set_byte(mem, addr_mode, result);
    }

    pub fn SXA_u(&mut self, mem: &mut Memory, addr_mode: AddressingMode) {
        let x = self.x;
        let addr = (((self.get_addr(mem, addr_mode)) & 0xF0) >> 8) as u8;
        let result = x & (addr.wrapping_add(1));
        self.set_byte(mem, addr_mode, result);
    }

    pub fn ATX_u(&mut self, mem: &mut Memory, addr_mode: AddressingMode) {
        let byte = self.fetch_byte(mem, addr_mode);
        self.a = self.a & byte;
        self.x = self.a;
        let x = self.x;
        self.set_zn_flags(x);
    }

    pub fn LAR_u(&mut self, mem: &mut Memory, addr_mode: AddressingMode) {
        let val = self.fetch_byte(mem, addr_mode);
        self.sp = self.sp & val;
        self.x = self.sp;
        self.a = self.sp;
        let a = self.a;
        self.set_zn_flags(a);
    }
}
