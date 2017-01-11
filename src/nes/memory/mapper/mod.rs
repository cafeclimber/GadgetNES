//! Defines all of the mappers used by NES cartridges. These define cartridge
//! memory.
use ines::InesRom;

/// All mappers should define what it means to read a byte from an address.
pub trait Mapper {
    fn read_rom_byte(&self, addr: u16) -> u8;
    fn read_chr_byte(&self, addr: u16) -> u8;
}

/// Reads information from the INES header to determine the mapper number,
/// then returns the appropriate mapper.
///
/// #Panics
/// This function panics if an attempt is made to load a rom with an unsupported
/// mapper. Currently only mapper 0 is implemented.
pub fn choose_mapper(rom: &InesRom) -> Box<Mapper> {
    match rom.mapper_number {
        0 => Box::new(Mapper0::new(rom)),
        _ => panic!("Unsupported mapper: {:#}", rom.mapper_number),
    }
}

/// Represents mapper 0, the simplest "mapper." Not actually a mapper, as
/// it were. Memory is directly interacted with
struct Mapper0 {
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>,
}

impl Mapper0 {
    pub fn new(rom: &InesRom) -> Mapper0 {
        Mapper0 {
            prg_rom: rom.prg_rom.to_owned(),
            chr_rom: rom.chr.to_owned(),
        }
    }
}

impl Mapper for Mapper0 {
    // deals with offset of beggining of cart being mapped to 0x8000
    /// Mapper 0 can be used with either 16KiB or 32KiB ROMs. Offsets are
    /// dealt with accordingly.
    fn read_rom_byte(&self, addr: u16) -> u8 {
        if self.prg_rom.len() == 0x8000 {
            self.prg_rom[addr as usize - 0x8000]
        } else {
            self.prg_rom[addr as usize - 0xC000]
        }
    }

    fn read_chr_byte(&self, addr: u16) -> u8 { self.chr_rom[addr as usize] }
}
