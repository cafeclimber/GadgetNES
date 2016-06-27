use cpu::Cpu;

#[derive(Default)]
pub struct Nes {
    cpu: Cpu,
}

impl Nes {
    pub fn new() -> Nes {
        Nes {
            cpu: Cpu::new(),
        }
    }

    pub fn power_up (&mut self, cart_rom: Vec<u8>) {
        self.cpu.power_up(cart_rom);
        println!("{:#?}\n", self.cpu);
    }

    pub fn run(& mut self) {
        loop {
            let instr = self.cpu.read_instr();
            self.cpu.run_instr(instr);
        }
    }
}
