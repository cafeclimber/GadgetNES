use super::mapper::*;

// TODO: extract info
pub struct Cartridge {
    pub mapper: Box<Mapper>,
}

pub struct RomHeader {
    pub prg_rom_size: u8, // In 16kB units
    pub prg_ram_size: u8, // In 8kB units
    pub chr_rom_size: u8, // If present, 8kB units
    pub mapper_number: u8,
    pub batt_ram: bool,
    pub trainer: bool,
    pub mirroring: Mirroring,
}

pub enum Mirroring {
    Vertical,
    Horizontal,
    FourWay,
}

fn read_rom_header(cart_rom: &Vec<u8>) -> RomHeader {
    RomHeader {
        prg_rom_size: cart_rom[4],
        prg_ram_size: if cart_rom[5] == 0 { 1 } else { cart_rom[5] }, // FIXME: CHR_ROM vs CHR_RAM
        chr_rom_size: if cart_rom[8] == 0 { 1 } else { cart_rom[8] },
        mapper_number: (cart_rom[6] & 0xf0) >> 4 | cart_rom[7] & 0xf0,
        batt_ram: (cart_rom[6] & (1 << 1)) != 0,
        trainer: (cart_rom[6] & (1 << 2)) != 0,
        mirroring: mirroring((cart_rom[6] & 1 << 0) | (cart_rom[6] & 1 << 3)),
    }
}

fn mirroring(mirror_bytes: u8) -> Mirroring {
    match mirror_bytes {
        0b0000 => Mirroring::Vertical,
        0b0001 => Mirroring::Horizontal,
        0b1000 | 0b1001 => Mirroring::FourWay,
        _ => unreachable!(),
    }
}

impl Cartridge {
    pub fn new(cart_rom: &Vec<u8>) -> Cartridge {
        let rom_header = read_rom_header(cart_rom);
        Cartridge { mapper: choose_mapper(&rom_header) }
    }

    pub fn read_cart(&self, addr: u16) -> u8 {
        match addr {
            0x6000...0x7fff => self.mapper.prg_ram_read(addr),
            0x8000...0xffff => self.mapper.prg_rom_read(addr),
            _ => {
                panic!("Attempt to read from unrecognized memory location: {:#x}",
                       addr)
            }
        }
    }

    pub fn write_byte_to_cart(&mut self, addr: u16, val: u8) {
        match addr {
            0x6000...0x7fff => self.mapper.prg_ram_write(addr, val),
            _ => {
                panic!("Attempt to write to unrecognized memory location: {:#x}",
                       addr)
            }
        }
    }
}
