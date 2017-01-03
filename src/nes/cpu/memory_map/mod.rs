//! Handles memory mapping for the CPU.
use nes::memory::Memory;

mod constants;
use self::constants::*;

/// Reads a word from the specified location
///
/// #Panics
/// Will panic on an attempt to read a word from PPU or I/O
pub fn read_word(mem: &mut Memory, addr: u16) -> u16 {
    match addr {
        CPU_RAM_BEG...CPU_RAM_END => {
            (mem.read_ram_byte(addr) as u16) |
            (mem.read_ram_byte(addr + 1) as u16) << 8
        },
        RAM_MIRRORS_BEG...RAM_MIRRORS_END => {
            (mem.read_ram_byte(addr % 0x0800) as u16) |
            (mem.read_ram_byte((addr % 0x0800) + 1) as u16) << 8
        },
        PPU_REGS_BEG...PPU_REGS_END => {
            panic!("read_word not implemented for PPU");
        },
        PPU_MIRRORS_BEG...PPU_MIRRORS_END => {
            panic!("read_word not implemented for PPU");
        },
        IO_REGS_BEG...IO_REGS_END => {
            panic!("read_word not implemented for I/O");
        },
        CART_SPACE_BEG...CART_SPACE_END => {
            (mem.read_rom_byte(addr) as u16) |
            (mem.read_rom_byte(addr + 1) as u16) << 8
        },
        _ => panic!("Erroneous read...how did you get here: {:#04X}", addr),
    }
}


/// Reads a byte from the specified address.
pub fn read_byte(mem: &mut Memory, addr: u16) -> u8 {
    match addr {
        CPU_RAM_BEG...CPU_RAM_END => {
            mem.read_ram_byte(addr)
        },
        RAM_MIRRORS_BEG...RAM_MIRRORS_END => {
            mem.read_ram_byte(addr & 0x07FF)
        },
        PPU_REGS_BEG...PPU_REGS_END => {
            mem.read_ppu_byte(addr)
        },
        PPU_MIRRORS_BEG...PPU_MIRRORS_END => {
            mem.read_ppu_byte((addr % 8) + 0x2000)
        },
        IO_REGS_BEG...IO_REGS_END => {
            mem.read_io_byte(addr)
        },
        CART_SPACE_BEG...CART_SPACE_END => {
            mem.read_rom_byte(addr)
        },
        _ => panic!("Erroneous read...how did you get here: {:#04X}", addr)
    }
}

/// Writes a byte to the specified address.
pub fn write_byte(mem: &mut Memory, addr: u16, val: u8) {
    match addr {
        CPU_RAM_BEG...CPU_RAM_END => {
            mem.write_ram_byte(addr, val);
        },
        RAM_MIRRORS_BEG...RAM_MIRRORS_END => {
            mem.write_ram_byte((addr % 0x0800), val);
        },
        PPU_REGS_BEG...PPU_REGS_END => {
            mem.write_ppu_byte(addr, val);
        },
        PPU_MIRRORS_BEG...PPU_MIRRORS_END => {
            mem.write_ppu_byte(addr % 8, val);
        },
        IO_REGS_BEG...IO_REGS_END => {
            mem.write_io_byte(addr, val);
        },
        CART_SPACE_BEG...CART_SPACE_END => {
            panic!("Cant write to cart space: {:#04X}", addr);
        },
        _ => panic!("Erroneous write...how did you get here: {:#04X}", addr),
    }
}
