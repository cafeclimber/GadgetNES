use super::cart::RomHeader;

pub trait Mapper {
    fn prg_rom_read(&self, addr: u16) -> u8;
    fn prg_rom_write(&mut self, addr: u16, val: u8);
    fn chr_rom_read(&self, addr: u16) -> u8;
    fn chr_rom_write(&mut self, addr: u16, val: u8);
    fn load_rom(&mut self, rom: Vec<u8>);
}

pub fn choose_mapper(rom_header: &RomHeader) -> Box<Mapper> {
    match rom_header.mapper_number {
        0 => Box::new(Mapper0::new(rom_header)),
        _ => panic!("Unsupported mapper: {:#}", rom_header.mapper_number),
    }
}

struct Mapper0 {
    prg_ram: Vec<u8>,
    prg_rom: Vec<u8>,
    chr: Vec<u8>,
}

impl Mapper0 {
    pub fn new(rom_header: &RomHeader) -> Mapper0 {
        Mapper0 {
            prg_ram: {
                let prg_ram_size = rom_header.prg_ram_size as usize * 8192;
                vec![0; prg_ram_size]
            },
            prg_rom: {
                let prg_rom_size = rom_header.prg_rom_size as usize * 16384;
                vec![0; prg_rom_size]
            },
            chr: {
                let chr_size = rom_header.chr_rom_size as usize * 8192;
                vec![0; chr_size]
            },
        }
    }
}

impl Mapper for Mapper0 {
    fn prg_rom_read(&self, addr: u16) -> u8 {
        if self.prg_rom.len() > 0x4000 {
            self.prg_rom[addr as usize & 0x7fff]
        } else {
            self.prg_rom[addr as usize & 0x3fff]
        }
    }

    fn prg_rom_write(&mut self, addr: u16, val: u8) {}

    fn chr_rom_read(&self, addr: u16) -> u8 {
        if self.prg_rom.len() > 0x4000 {
            self.prg_rom[addr as usize & 0x7fff]
        } else {
            self.prg_rom[addr as usize & 0x3fff]
        }
    }

    fn chr_rom_write(&mut self, addr: u16, val: u8) {}

    fn load_rom(&mut self, rom: Vec<u8>) {
        self.prg_rom = rom[0..16392].to_owned();
        self.chr = rom[16392..].to_owned();
    }
}
