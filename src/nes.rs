use cpu::Cpu;

#[derive(Debug)]
pub struct Nes {
    cpu: Cpu,
}

impl Nes {
    pub fn new() -> Nes {
        Nes {
            cpu: Cpu::default(),
        }
    }

    pub fn power_up (&mut self, rom: &Vec<u8>) {
        self.cpu.power_up();
    }
}
