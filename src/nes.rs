use cpu::Cpu;
use interconnect::Interconnect;

pub struct Nes {
    cpu: Cpu,
    interconnect: Interconnect,
}

impl Nes {
    pub fn new(cart_rom: &Vec<u8>) -> Nes {
        Nes {
            cpu: Cpu::new(),
            interconnect: Interconnect::new(&cart_rom),
        }
    }

    pub fn power_up (&mut self, cart_rom: Vec<u8>) {
        self.cpu.power_up();
        self.interconnect.power_up(cart_rom);
    }

    pub fn run(&mut self) {
        let mut count = 0;
        loop {
            if count > 100 {return} else{
            self.cpu.run_instr(&mut self.interconnect);
            count += 1;
            }
        }
    }
}
