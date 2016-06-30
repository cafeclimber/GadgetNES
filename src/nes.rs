use cpu::Cpu;
use interconnect::Interconnect;

pub struct Nes {
    cpu: Cpu,
    interconnect: Interconnect,
}

impl Nes {
    pub fn new(cart_rom: Vec<u8>) -> Nes {
        Nes {
            cpu: Cpu::new(),
            interconnect: Interconnect::new(cart_rom),
        }
    }

    pub fn power_up (&mut self) {
        self.cpu.power_up();
        self.interconnect.power_up();
        println!("{:#?}\n", self.cpu);
    }

    pub fn run(&mut self) {
        loop {
            self.cpu.run_instr(&mut self.interconnect);
            println!("{:#?}\n", self.cpu);
        }
    }
}
