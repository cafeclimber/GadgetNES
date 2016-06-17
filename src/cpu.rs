#[derive(Default, Debug)]
pub struct Cpu {
    a: u8, // Accumulator

    x: u8, // x-Index
    y: u8, // y-index

    pc: u16, // Program counter
    
    s: u8, // Stack pointer

    p: u8, // Status register
}

impl Cpu {
    pub fn power_up(&mut self) {
        self.p = 0x34;
        self.s = 0xfd;
    }
}
