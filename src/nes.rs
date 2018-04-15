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

    pub fn run(&mut self) {
        loop {
            self.cpu.step(&mut self.cart);
        }
    }
}
