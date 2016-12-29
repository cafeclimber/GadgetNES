//! For branch related instructions
//! All instructions reutrn the number of cycles taken
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]

use nes::cpu::{Cpu, StatusFlag};
use nes::cpu::memory_map::read_byte;
use nes::memory::Memory;

// PRETTIFYME: Refactor with single branch function which takes a closure
// PRETTIFYME: Is there a cleaner way to do this addition?
impl Cpu {
    pub fn BPL(&mut self, mem: &Memory) {
        let offset = read_byte(mem, self.pc + 1) as i8;
        let branch_target = (((self.pc + 2) as i32) + (offset as i32)) as u16;
        print!(" BPL ${:04X} ", branch_target);
        print!("                     ");
        if self.check_no_flag(StatusFlag::Negative) {
            self.pc = branch_target;
        } else {
            self.pc += 2;
        }
    }

    pub fn BMI(&mut self, mem: &Memory) {
        let offset = read_byte(mem, self.pc + 1) as i8;
        let branch_target = (((self.pc + 2) as i32) + (offset as i32)) as u16;
        print!(" BMI ${:04X} ", branch_target);
        print!("                     ");
        if self.check_flag(StatusFlag::Negative) {
            self.pc = branch_target;
        } else {
            self.pc += 2;
        }
    }

    pub fn BVC(&mut self, mem: &Memory) {
        let offset = read_byte(mem, self.pc + 1) as i8;
        let branch_target = (((self.pc + 2) as i32) + (offset as i32)) as u16;
        print!(" BVC ${:04X} ", branch_target);
        print!("                     ");
        if self.check_no_flag(StatusFlag::Overflow) {
            self.pc = branch_target;
        } else {
            self.pc += 2;
        }
    }

    pub fn BVS(&mut self, mem: &Memory) {
        let offset = read_byte(mem, self.pc + 1) as i8;
        let branch_target = (((self.pc + 2) as i32) + (offset as i32)) as u16;
        print!(" BVS ${:04X} ", branch_target);
        print!("                     ");
        if self.check_flag(StatusFlag::Overflow) {
            self.pc = branch_target;
        } else {
            self.pc += 2;
        }
    }
    
    pub fn BCC(&mut self, mem: &Memory) {
        let offset = read_byte(mem, self.pc + 1) as i8;
        let branch_target = (((self.pc + 2) as i32) + (offset as i32)) as u16;
        print!(" BCC ${:04X} ", branch_target);
        print!("                     ");
        if self.check_no_flag(StatusFlag::Carry) {
            self.pc = branch_target;
        } else {
            self.pc += 2;
        }
    }

    pub fn BCS(&mut self, mem: &Memory) {
        let offset = read_byte(mem, self.pc + 1) as i8;
        let branch_target = (((self.pc + 2) as i32) + (offset as i32)) as u16;
        print!(" BCS ${:04X} ", branch_target);
        print!("                     ");
        if self.check_flag(StatusFlag::Carry) {
            self.pc = branch_target;
        } else {
            self.pc += 2;
        }
    }

    pub fn BNE(&mut self, mem: &Memory) {
        let offset = read_byte(mem, self.pc + 1) as i8;
        let branch_target = (((self.pc + 2) as i32) + (offset as i32)) as u16;
        print!(" BNE ${:04X} ", branch_target);
        print!("                     ");
        if self.check_no_flag(StatusFlag::Zero) {
            self.pc = branch_target;
        } else {
            self.pc += 2;
        }
    }

    pub fn BEQ(&mut self, mem: &Memory) {
        let offset = read_byte(mem, self.pc + 1) as i8;
        let branch_target = (((self.pc + 2) as i32) + (offset as i32)) as u16;
        print!(" BEQ ${:04X} ", branch_target);
        print!("                     ");
        if self.check_flag(StatusFlag::Zero) {
            self.pc = branch_target;
        } else {
            self.pc += 2;
        }
    }
}
