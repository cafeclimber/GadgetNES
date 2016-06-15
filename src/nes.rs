use cpu::Cpu;
use mem::*;

#[derive(Debug)]
pub struct Nes {
    cpu: Cpu,
}

impl Nes {
    pub fn new() -> Nes {
        Nes {
            cpu: Cpu::default(), // 2A03
            // PPU
        }
    }

    pub fn power_up (&mut self, cart_rom: &Vec<u8>) {
        self.cpu.power_up();

        let mut memory = Memory::new(&cart_rom);
        
        self.run(cart_rom, &mut memory);
    }
}

impl Nes {
    fn run(&mut self, cart_rom: &Vec<u8>, memory: &mut Memory) {
    }
}
