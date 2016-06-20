use std::fmt;
use super::apu::Apu;

const RAM_SIZE: usize = 2048;

#[derive(Default)]
pub struct Cpu {
    a: u8, // Accumulator

    x: u8, // x-Index
    y: u8, // y-index

    pub pc: u16, // Program counter
    
    s: u8, // Stack pointer

    p: u8, // Status register

    ram: Box<[u8]>, // RAM

    // Because all instructions are first run by the cpu,
    // it is easiest to let it own both the APU and the PPU
    apu: Apu,
    // ppu: Ppu,
}

impl Cpu {
    pub fn new() -> Cpu{
        Cpu {
            a: 0,

            x: 0,
            y: 0,

            pc: 0,

            s: 0,

            p: 0,

            ram: vec![0u8; RAM_SIZE].into_boxed_slice(),

            apu: Apu::default(),
        } 
    }
    pub fn power_up(&mut self) {
        self.p = 0x34;
        self.s = 0xfd;
    }

    pub fn run_instr(instr: u8) {
        map_instr(instr);
    }
}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CPU: a:0x{:x} x:0x{:x} y:0x{:x} pc:0x{:x} s:0x{:x} p:0x{:x}",
               self.a, self.x, self.y, self.pc, self.s, self.p)
    }
}

fn map_instr(instr: u8);
