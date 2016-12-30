//! For branch related instructions
//! All instructions reutrn the number of cycles taken
#![allow(non_snake_case)]

use nes::cpu::{Cpu, StatusFlag};
use nes::cpu::memory_map::read_byte;
use nes::memory::Memory;

impl Cpu {
    fn branch(&mut self,
              mem: &Memory,
              flag: StatusFlag,
              branch_if_set: bool)
    {
        let offset = read_byte(mem, self.pc + 1) as i8;
        // PRETTIFYME: Is there a cleaner way to do this addition?
        let branch_target = (((self.pc + 2) as i32) + (offset as i32)) as u16;
        let branch = self.check_flag(flag, branch_if_set);
        match branch {
            true => self.pc = branch_target,
            false => self.pc += 2,
        }
    }

    pub fn BPL(&mut self, mem: &Memory) {
        self.branch(mem, StatusFlag::Negative, false);
    }

    pub fn BMI(&mut self, mem: &Memory) {
        self.branch(mem, StatusFlag::Negative, true);
    }

    pub fn BVC(&mut self, mem: &Memory) {
        self.branch(mem, StatusFlag::Overflow, false);
    }

    pub fn BVS(&mut self, mem: &Memory) {
        self.branch(mem, StatusFlag::Overflow, true);
    }
    
    pub fn BCC(&mut self, mem: &Memory) {
        self.branch(mem, StatusFlag::Carry, false);
    }

    pub fn BCS(&mut self, mem: &Memory) {
        self.branch(mem, StatusFlag::Carry, true);
    }

    pub fn BNE(&mut self, mem: &Memory) {
        self.branch(mem, StatusFlag::Zero, false);
    }

    pub fn BEQ(&mut self, mem: &Memory) {
        self.branch(mem, StatusFlag::Zero, true);
    }
}
