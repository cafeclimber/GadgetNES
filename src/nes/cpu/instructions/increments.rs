//! Instructions to increment / decrement memory values.
#![allow(non_snake_case)]

use nes::cpu::{Cpu, StatusFlag};
use nes::memory::Memory;
use super::AddressingMode;

/// Instructions to increment / decrement memory values.
impl Cpu {
    /// DECrement memory.
    pub fn DEC(&mut self, mem: &mut Memory, addr_mode: AddressingMode) {
        let val = self.fetch_byte(mem, addr_mode);
        let result = val.wrapping_sub(1);

        self.set_zn_flags(result);
        
        self.set_byte(mem, addr_mode, result);
    }

    /// INCrement memory.
    pub fn INC(&mut self, mem: &mut Memory, addr_mode: AddressingMode) {
        let val = self.fetch_byte(mem, addr_mode);
        let result = val.wrapping_add(1);

        self.set_zn_flags(result);
        
        self.set_byte(mem, addr_mode, val.wrapping_add(1));
    }
}
