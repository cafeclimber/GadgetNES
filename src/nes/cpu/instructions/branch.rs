//! Branch instructions.
#![allow(non_snake_case)]

use nes::cpu::{Cpu, StatusFlag};
use nes::cpu::memory_map::read_byte;
use nes::memory::Memory;

/// Branch instructions.
impl Cpu {
    fn branch(&mut self,
              mem: &mut Memory,
              flag: StatusFlag,
              branch_if_set: bool)
    {
        let offset = read_byte(mem, self.pc + 1) as i8;
        // PRETTIFYME: Is there a cleaner way to do this addition?
        let branch_target = (((self.pc + 2) as i32) + (offset as i32)) as u16;
        let branch = self.check_flag(flag, branch_if_set);
        match branch {
            true => {
                self.pc = branch_target;
            }
            false => {
                self.pc += 2;
            }
        }
    }

    /// Branch on PLus. 
    pub fn BPL(&mut self, mem: &mut Memory) {
        self.branch(mem, StatusFlag::Negative, false);
    }

    /// Branch on MInus.
    pub fn BMI(&mut self, mem: &mut Memory) {
        self.branch(mem, StatusFlag::Negative, true);
    }

    /// Branch on oVerflow Clear.
    pub fn BVC(&mut self, mem: &mut Memory) {
        self.branch(mem, StatusFlag::Overflow, false);
    }

    /// Branch on oVerflow Set.
    pub fn BVS(&mut self, mem: &mut Memory) {
        self.branch(mem, StatusFlag::Overflow, true);
    }
    
    /// Branch on Carry Clear. 
    pub fn BCC(&mut self, mem: &mut Memory) {
        self.branch(mem, StatusFlag::Carry, false);
    }

    /// Branch on Carry Set. 
    pub fn BCS(&mut self, mem: &mut Memory) {
        self.branch(mem, StatusFlag::Carry, true);
    }

    /// Branch on Not Equal. 
    pub fn BNE(&mut self, mem: &mut Memory) {
        self.branch(mem, StatusFlag::Zero, false);
    }

    /// Branch on EQual. 
    pub fn BEQ(&mut self, mem: &mut Memory) {
        self.branch(mem, StatusFlag::Zero, true);
    }
}
