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

    pub fn run(&mut self) {
        loop {
            let instr = self.read_instr();
            self.run_instr(instr);
        }
    }

    fn read_instr(&mut self) -> u8 {
        let pc = self.cpu.pc;
        self.cart.read_rom(&pc);
        0
    }

    fn run_instr(&mut self, instr: u8) {
        //self.cpu.run_instr(instr);
    }
}
