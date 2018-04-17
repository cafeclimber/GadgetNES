use std::collections::HashMap;

use super::cart::Cartridge;
use super::cpu::Cpu;

pub const KILOBYTE: usize = 1024;

// Fields are public for debugger
pub struct Nes<'a> {
    // apu: Apu
    pub cpu: Cpu,
    // ppu: Ppu,
    pub cart: &'a mut Cartridge,
}

impl<'a> Nes<'a> {
    pub fn new(cart: &'a mut Cartridge) -> Self {
        Nes {
            cpu: Cpu::new(),
            cart: cart,
        }
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
    }

    pub fn run(&mut self, breakpoints: Option<&HashMap<usize, usize>>) {
        if let Some(bps) = breakpoints {
            loop {
                if bps.values().any(|&bp| (bp as u16) == self.cpu.pc()) {
                    println!("Encountered breakpoint @ {:04X}", self.cpu.pc());
                    break;
                };
                self.step();
            }
        }
        else {
            loop {
                self.step();
            }
        }
    }

    pub fn step(&mut self) {
        self.cpu.step(&mut self.cart);
    }

}
