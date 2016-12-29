use ines::InesRom;

// Unsure if I want to implement this in this way or just implement
// Memory for each mapper
pub trait Mapper {
    fn read_rom_byte(&self, addr: u16) -> u8;
}

pub fn choose_mapper(rom: &InesRom) -> Box<Mapper> {
    match rom.mapper_number {
        0 => Box::new(Mapper0::new(rom)),
        _ => panic!("Unsupported mapper: {:#}", rom.mapper_number),
    }
}

struct Mapper0 {
    prg_rom: Vec<u8>,
}

impl Mapper0 {
    pub fn new(rom: &InesRom) -> Mapper0 {
        Mapper0 {
            prg_rom: rom.prg_rom.to_owned(),
        }
    }
}

impl Mapper for Mapper0 {
    // deals with offset of beggining of cart being mapped to 0x8000
    fn read_rom_byte(&self, addr: u16) -> u8 {
        if self.prg_rom.len() > 0x8000 {
            self.prg_rom[(addr - 0x8000) as usize]
        } else if addr >= 0xC000 {
            self.prg_rom[(addr - 0xC000) as usize]
        } else {
            self.prg_rom[(addr - 0x8000) as usize]
        }
    }
}
