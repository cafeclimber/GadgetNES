use cpu::Cpu;
use apu::Apu;
use mem::*;

#[derive(Debug)]
pub struct Nes {
    cpu: Cpu,
    apu: Apu,
    //ppu: Ppu,
    cart: Cartridge,
}

impl Nes {
    pub fn new() -> Nes {
        Nes {
            cpu: Cpu::default(),
            apu: Apu::default(),
        }
    }

    pub fn power_up (&mut self, cart_rom: &Vec<u8>) {
        self.cpu.power_up();
    }
}
