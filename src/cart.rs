#![allow(dead_code)]

use super::nes::KILOBYTE;
use super::rom::Rom;

pub struct Cartridge {
    mapper: Box<Mapper>,
}

impl Cartridge {
    pub fn new(rom: Rom) -> Self {
        Cartridge {
            mapper: match rom.header.mapper_no() {
                0 => Box::new(Mapper000::new(rom)),
                _ => panic!("Mapper {:#} not implemented", rom.header.mapper_no()),
            },
        }
    }

    pub fn prg_read(&self, addr: u16) -> u8 {
        self.mapper.prg_read(addr)
    }

    pub fn prg_write(&mut self, addr: u16, val: u8) {
        self.mapper.prg_write(addr, val);
    }

    pub fn chr_read(&self, addr: u16) -> u8 {
        self.mapper.chr_read(addr)
    }

    pub fn chr_write(&mut self, addr: u16, val: u8) {
        self.mapper.chr_write(addr, val);
    }
}

trait Mapper {
    fn prg_read(&self, addr: u16) -> u8;
    fn prg_write(&mut self, addr: u16, val: u8);
    fn chr_read(&self, addr: u16) -> u8;
    fn chr_write(&mut self, addr: u16, val: u8);
}

struct Mapper000 {
    prg_rom: Vec<u8>,
    prg_ram: Vec<u8>,
    chr_rom: Vec<u8>,
}

impl Mapper000 {
    pub fn new(rom: Rom) -> Self {
        let prg_ram = vec![0; 8 * KILOBYTE];

        Mapper000 {
            prg_rom: {
                let mut temp: Vec<u8> = Vec::new();
                if rom.prg_rom.len() <= 16 * KILOBYTE {
                    // Duplicate instead of dealing with mirroring arithmetic
                    temp.extend(rom.prg_rom.iter());
                    temp.extend(rom.prg_rom.iter());
                } else {
                    temp.extend(rom.prg_rom.iter());
                }
                temp
            },
            prg_ram: prg_ram,
            chr_rom: rom.chr_rom,
        }
    }
}

impl Mapper for Mapper000 {
    fn prg_read(&self, addr: u16) -> u8 {
        if (addr >= 0x6000) & (addr < 0x7FFF) {
            self.prg_ram[(addr - 0x6000) as usize]
        } else if addr >= 0x8000 {
            self.prg_rom[(addr - 0x8000) as usize]
        } else {
            panic!("Unrecognized PRG address: {:#X}", addr);
        }
    }

    fn prg_write(&mut self, addr: u16, val: u8) {
        if (addr >= 0x6000) & (addr < 0x7FFF) {
            self.prg_ram[(addr - 0x6000) as usize] = val;
        } else if addr >= 0x8000 {
            self.prg_rom[(addr - 0x8000) as usize] = val;
        } else {
            panic!("Unrecognized PRG address: {:#X}", addr);
        }
    }

    fn chr_read(&self, addr: u16) -> u8 {
        if addr < 0x2000 {
            self.chr_rom[(addr - 0x2000) as usize]
        } else {
            panic!("Unrecognized CHR address: {:#X}", addr);
        }
    }

    fn chr_write(&mut self, addr: u16, val: u8) {
        if addr < 0x2000 {
            self.chr_rom[(addr - 0x2000) as usize] = val;
        } else {
            panic!("Unrecognized CHR address: {:#X}", addr);
        }
    }
}
