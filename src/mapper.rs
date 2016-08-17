use super::cart::{RomHeader, Mirroring};

const VRAM_SIZE: usize = 0x800;

pub trait Mapper {
    // CPU Helpers
    fn prg_rom_read(&self, addr: u16) -> u8;
    fn prg_ram_read(&self, addr: u16) -> u8;
    fn prg_ram_write(&mut self, addr: u16, val: u8);
    // PPU Helpers
    fn get_pattern_table_byte(&self, addr: u16) -> u8;
    fn get_nametable_byte(&self, addr: u16) -> u8;
    fn get_palette_byte(&self, addr: u16) -> u8;
    // Initialization
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
    chr_mem: Vec<u8>,
    vram: Vec<u8>,
    mirroring: Mirroring,
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
            chr_mem: {
                let chr_mem_size = rom_header.chr_mem_size as usize * 8192;
                vec![0; chr_mem_size]
            },
            vram: {
                vec![0; VRAM_SIZE]
            },
            mirroring: rom_header.mirroring,
        }
    }
}

impl Mapper for Mapper0 {
    fn prg_rom_read(&self, addr: u16) -> u8 {
        if addr < 0x8000 {
            panic!("Attempted to read from RAM using ROM Read");
        } else if self.prg_rom.len() > 16392 {
            self.prg_rom[addr as usize & 0x7fff]
        } else {
            self.prg_rom[addr as usize & 0x3fff]
        }
    }

    // TODO: Correct?
    fn prg_ram_read(&self, addr: u16) -> u8 {
        self.prg_ram[addr as usize]
    }

    fn prg_ram_write(&mut self, addr: u16, val: u8) {
        self.prg_ram[addr as usize] = val;
    }

    fn get_pattern_table_byte(&self, addr: u16) -> u8 {
        if addr > 0x2000 {
            panic!("Attempted to get pattern table byte from outside pattern table: {:#X}",
                   addr);
        }
        self.chr_mem[addr as usize]
    }
    fn get_nametable_byte(&self, addr: u16) -> u8 {
        match self.mirroring {
            Mirroring::Vertical => {
                match addr {
                    // 0x2000 = 0x2800 and 0x2400 = 0x2c00
                    0x2000...0x23ff => self.vram[(addr - 0x2000) as usize],
                    0x2800...0x2bff => self.vram[(addr - 0x2000 - 0x0800) as usize],

                    0x2400...0x27ff => self.vram[(addr - 0x2000) as usize],
                    0x2c00...0x2fff => self.vram[(addr - 0x2000 - 0x0800) as usize],
                    _ => {
                        panic!("Attempted to get pattern table byte from outside pattern table: \
                                {:#X}",
                               addr)
                    }
                }
            }
            Mirroring::Horizontal => {
                match addr {
                    // 0x2000 = 0x2400 and 0x2800 = 0x2c00
                    0x2000...0x23ff => self.vram[(addr - 0x2000) as usize],
                    0x2400...0x27ff => self.vram[(addr - 0x2000 - 0x0400) as usize],

                    0x2800...0x2bff => self.vram[(addr - 0x2000) as usize],
                    0x2c00...0x2fff => self.vram[(addr - 0x2000 - 0x0400) as usize],
                    _ => {
                        panic!("Attempted to get pattern table byte from outside pattern table: \
                                {:#X}",
                               addr)
                    }
                }
            }
            Mirroring::FourWay => panic!("This mapper doesn't support 4-way mirroring"),
        }
    }
    fn get_palette_byte(&self, addr: u16) -> u8 {
        if addr < 0x3f00 || addr > 0x3f1f {
            panic!("Attempted to get palette byte from outside palette ram: {:#X}",
                   addr);
        }
        self.chr_mem[addr as usize]
    }

    fn load_rom(&mut self, rom: Vec<u8>) {
        self.prg_rom = rom[16..16400].to_owned();
        self.chr_mem = rom[16400..24592].to_owned(); // 8kB
    }
}
