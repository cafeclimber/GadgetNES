#[derive(Debug)]
pub struct Cpu {
    a: u8, // Accumulator

    x: u8, // x-Index
    y: u8, // y-index

    pc: u16, // Program counter
    
    s: u8, // Stack pointer

    p: u8, // Status register
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            a: 0,

            x: 0,
            y: 0,

            pc: 0x0000,

            s: 0xfd,

            p: 0x34,
        }
    }
}
