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
    fn read_byte(&mut self, addr: u16) -> u8 {
        #[cfg(feature="debug")]
        println!("\nWARNING: Reads from I/O registers not yet implemented: {:#X}",
                 addr);
        0
    }

    fn write_byte(&mut self, addr: u16, val: u8) {
        #[cfg(feature="debug")]
        println!("\nWARNING: Writes to I/O registers not yet implemented: {:#X} -> {:#X}",
                 addr,
                 val);
    }
}
