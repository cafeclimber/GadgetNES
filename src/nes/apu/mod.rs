//! Provides interface to APU (Audio Processing Unit)

use nes::MemMapped;

pub struct Apu {
}

iml Apu {
    pub fn new() -> Apu {
        Apu {
        }
    }
}

// TODO
impl MemMapped for Io {
    fn read_byte(&self, addr: u16) -> u8 {
        println!("Reads from APU registers not yet implemented");
        0
    }

    fn write_byte(&mut self, addr: u16, val: u8) {
        println!("Writes to APU registers not yet implemented");
    }
}
