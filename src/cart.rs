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
}

fn read_rom_header(cart_rom: &Vec<u8>) -> RomHeader {
    RomHeader {
        prg_rom_size: cart_rom[4],
        prg_ram_size: if cart_rom[5] == 0 {1} else {cart_rom[5]}, // FIXME: CHR_ROM vs CHR_RAM
        chr_rom_size: if cart_rom[8] == 0 {1} else {cart_rom[8]},
        mapper_number: (cart_rom[6] & 0xf0) >> 4 | cart_rom[7] & 0xf0,
        batt_ram: (cart_rom[6] & (1 << 1)) != 0,
        trainer: (cart_rom[6] & (1 << 2)) != 0,
    }
}

impl Cartridge  {
    pub fn new(cart_rom: &Vec<u8>) -> Cartridge {
        let rom_header = read_rom_header(cart_rom);
        Cartridge {
            mapper: choose_mapper(&rom_header),
        }
    }

    // TODO Make a match once I implement the rest of the cartridge
    pub fn read_cart(&self, addr: u16) -> u8 {
        0
    }

    pub fn write_byte_to_cart(&mut self, addr: u16, val: u8) {
    }

    // pub fn new_mapper() -> Mapper {}
}

