use std::collections::HashMap;

use super::cart::Cartridge;
use super::cpu::Cpu;

pub const KILOBYTE: usize = 1024;

pub struct Nes<'a> {
    // apu: Apu
    cpu: Cpu,
    // ppu: Ppu,
    cart: &'a mut Cartridge,
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
                    println!("Encountered breakpoint");
                    break;
                }
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

    // TODO: Move this to debugger and make CPU public?
    pub fn print(&mut self, addr: u16) {
        println!("M[{:04X}] = {:X}", addr, self.cpu.fetch_byte(&mut self.cart, addr));
    }
}
