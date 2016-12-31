//! Provides I/O interface

use nes::MemMapped;

pub struct Io {
}

impl Io {
    pub fn new() -> Io {
        Io {
        }
    }
}

// TODO
impl MemMapped for Io {
    fn read_byte(&self, addr: u16) -> u8 {
        println!("\nWARNING: Reads from I/O registers not yet implemented");
        0
    }

    fn write_byte(&mut self, addr: u16, val: u8) {
        println!("\nWARNING: Writes to I/O registers not yet implemented");
    }
}
