use nes::MemMapped;
use nes::memory::Memory;

mod constants;
use self::constants::*;

pub fn read_word(mem: &Memory, addr: u16) -> u16 {
    match addr {
        CPU_RAM_BEG...CPU_RAM_END => {
            (mem.read_ram_byte(addr) as u16) |
            (mem.read_ram_byte(addr + 1) as u16) << 8
        },
        RAM_MIRRORS_BEG...RAM_MIRRORS_END => {
            (mem.read_ram_byte(addr % 0x0800) as u16) |
            (mem.read_ram_byte((addr % 0x0800) + 1) as u16) << 8
        },
        PPU_MIRRORS_BEG...PPU_MIRRORS_END => {
            panic!("Reads from PPU not yet supported: {:#X}", addr)
        },
        IO_REGS_BEG...IO_REGS_END => {
            panic!("I/O writes not yet supported")
            // self.io.read_byte(addr)
        },
        CART_SPACE_BEG...CART_SPACE_END => {
            (mem.read_rom_byte(addr) as u16) |
            (mem.read_rom_byte(addr + 1) as u16) << 8
        },
        _ => panic!("Attempt to read from unsupported location: {:#X}",
                    addr),
    }
}


pub fn read_byte(mem: &Memory, addr: u16) -> u8 {
    match addr {
        CPU_RAM_BEG...CPU_RAM_END => {
            mem.read_ram_byte(addr)
        },
        RAM_MIRRORS_BEG...RAM_MIRRORS_END => {
            mem.read_ram_byte(addr % 0x0800)
        },
        PPU_MIRRORS_BEG...PPU_MIRRORS_END => {
            panic!("Reads from PPU not yet supported: {:#X}", addr)
        },
        IO_REGS_BEG...IO_REGS_END => {
            panic!("I/O writes not yet supported")
            // self.io.read_byte(addr)
        },
        CART_SPACE_BEG...CART_SPACE_END => {
            mem.read_rom_byte(addr)
        },
        _ => panic!("Attempt to read from unsupported location: {:#X}",
                    addr),
    }
}

pub fn write_byte(mem: &mut Memory, addr: u16, val: u8) {
    match addr {
        CPU_RAM_BEG...CPU_RAM_END => {
            mem.write_ram_byte(addr, val);
        },
        RAM_MIRRORS_BEG...RAM_MIRRORS_END => {
            mem.write_ram_byte((addr % 0x0800), val);
        },
        PPU_MIRRORS_BEG...PPU_MIRRORS_END => {
            panic!("Reads from PPU not yet supported: {:#X}", addr)
        },
        IO_REGS_BEG...IO_REGS_END => {
            panic!("I/O writes not yet supported")
            // self.io.write_byte(addr, val);
        },
        _ => panic!("Attempt to write to unsupported location: {:#X}",
                    addr),
    }
}

// TODO: Make struct of addrs to ppu and apu
// Implements Memory trait
struct Io {
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
        panic!("Reads from I/O registers not yet implemented")
    }

    fn write_byte(&mut self, addr: u16, val: u8) {
        panic!("Writes to I/O registers not yet implemented")
    }
}
