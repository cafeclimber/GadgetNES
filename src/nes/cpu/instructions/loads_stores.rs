//! Instructions which load from / store to memory.
#![allow(non_snake_case)]

use nes::cpu::Cpu;
use nes::memory::Memory;
use super::AddressingMode;

/// Load and store instructions
impl Cpu {
    /// LoaD Accumulator.
    pub fn LDA(&mut self, mem: &mut Memory, addr_mode: AddressingMode) {
        let val = self.fetch_byte(mem, addr_mode);
        self.a = val;
        self.set_zn_flags(val);
    }

    /// LoaD X register.
    pub fn LDX(&mut self, mem: &mut Memory, addr_mode: AddressingMode) {
        let val = self.fetch_byte(mem, addr_mode);
        self.x = val;
        self.set_zn_flags(val);
    }

    /// LoaD Y register.
    pub fn LDY(&mut self, mem: &mut Memory, addr_mode: AddressingMode) {
        let val = self.fetch_byte(mem, addr_mode);
        self.y = val;
        self.set_zn_flags(val);
    }

    /// STore Accumulator.
    pub fn STA(&mut self, mem: &mut Memory, addr_mode: AddressingMode) {
        let val = self.a;
        self.set_byte(mem, addr_mode, val);
    }

    /// STore X register.
    pub fn STX(&mut self, mem: &mut Memory, addr_mode: AddressingMode) {
        let val = self.x;
        self.set_byte(mem, addr_mode, val);
    }

    /// STore Y register.
    pub fn STY(&mut self, mem: &mut Memory, addr_mode: AddressingMode) { 
        let val = self.y;
        self.set_byte(mem, addr_mode, val);
    }
}
