use super::cart::Cartridge;
use super::cpu::Cpu;

pub const KILOBYTE: usize = 1024;

pub struct Nes<'a> {
    // apu: Apu
    cpu: Cpu<'a>,
    // ppu: Ppu,
    cart: &'a Cartridge,
}

impl<'a> Nes<'a> {
    pub fn new(cart: &'a Cartridge) -> Self {
        Nes {
            cpu: Cpu::new(&cart),
            cart: cart,
        }
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
    }

    pub fn run(&mut self) {
        loop {
            self.cpu.step();
        }
    }
}
