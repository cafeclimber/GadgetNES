//! For the nop instruction
//! All instructions reutrn the number of cycles taken
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]

use nes::cpu::Cpu;

impl Cpu {
    pub fn NOP(&self) {
        print!(" NOP         ");
        print!("                   ");
    }
}
