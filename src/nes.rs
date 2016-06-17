use cpu::Cpu;
use apu::Apu;
use cart::Cartridge;

// Constants for header file reading
const NES: u32 = 0x4e4553;

#[derive(Default)]
pub struct Nes {
    cpu: Cpu,
    apu: Apu,
    //ppu: Ppu,
    pub cart: Cartridge,
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
        self.read_rom_header(&cart_rom);
        self.cart.load_cartridge(cart_rom);

        println!("{:#?}\n", self.cpu);
        println!("{:#?}", self.apu);
    }

    fn read_rom_header(&mut self, cart_rom: &Vec<u8>) {
        // TODO: Implement more reading of header formats
        let header = ((cart_rom[0] as u32) << 16) | ((cart_rom[1] as u32) << 8) | (cart_rom[2] as u32);

        match header {
            NES => println!("Read iNES file format"),
            _ => panic!("Unrecognized file type"),
        }
    }
}
