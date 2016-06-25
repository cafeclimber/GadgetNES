use cpu::Cpu;
use apu::Apu;
use cart::Cartridge;

#[derive(Default)]
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
            cart: Cartridge::default(),
        }
    }

    pub fn power_up (&mut self, cart_rom: Vec<u8>) {
        self.cpu.power_up();
        self.cart.load_cartridge(cart_rom);

        println!("{:#?}\n", self.cpu);
        println!("{:#?}", self.apu);
    }

}
