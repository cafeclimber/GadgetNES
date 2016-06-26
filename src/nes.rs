use cpu::Cpu;
use cart::Cartridge;

#[derive(Default)]
pub struct Nes {
    cpu: Cpu,
    cart: Cartridge,
}

impl Nes {
    pub fn new() -> Nes {
        Nes {
            cpu: Cpu::new(),
            cart: Cartridge::default(),
        }
    }

    pub fn power_up (&mut self, cart_rom: Vec<u8>) {
        self.cpu.power_up();
        self.cart.load_cartridge(cart_rom);

        println!("{:#?}\n", self.cpu);
    }

    pub fn run(self) {
        loop {
            let instr = self.cpu.read_instr(&self.cart);
            self.cpu.run_instr(instr);
        }
    }
}
