use nes::MemMapped;
use ines::InesRom;

mod mapper;
use self::mapper::*;

mod constants;
use self::constants::*;

// TODO: Add CHR
/// This is a raw container of the various memories in the NES.
/// This does NOT include memory mapped I/O.
/// Things that are mapped are handled by their respective components
/// memory_map modules.
pub struct Memory {
    ram: Ram,
    // exp: ExpansionRom TODO
    // sram: Sram TODO
    prg: Box<Mapper>,
}

impl Memory {
    pub fn init(rom: &InesRom) -> Memory {
        Memory {
            ram: Ram::new(),
            prg: choose_mapper(rom),
        }
    }

    pub fn read_ram_byte(&self, addr: u16) -> u8 {
        self.ram.read_byte(addr)
    }

    pub fn read_rom_byte(&self, addr: u16) -> u8 {
        self.prg.read_rom_byte(addr)
    }

    pub fn write_ram_byte(&mut self, addr: u16, val: u8) {
        self.ram.write_byte(addr, val);
    }
}

struct Ram {
    zero_page: Vec<u8>,
    stack: Vec<u8>,
    ram: Vec<u8>
}

impl Ram {
    pub fn new() -> Ram {
        Ram {
            zero_page: vec![0; ZERO_PAGE_SIZE],
            stack: vec![0; STACK_SIZE],
            ram: vec![0; RAM_SIZE],
        }
    }
}

impl MemMapped for Ram {
    fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            ZERO_PAGE_BEG...ZERO_PAGE_END => {
                self.zero_page[addr as usize]
            },
            STACK_BEG...STACK_END => {
                let rel_addr = (addr - STACK_BEG) as usize;
                self.stack[rel_addr]
            },
            RAM_BEG...RAM_END => {
                let rel_addr = (addr - RAM_BEG) as usize;
                self.ram[rel_addr]
            }
            _ => panic!("Improper attempt to read from RAM at addr: {:#X}", addr)
        }
    }

    fn write_byte(&mut self, addr: u16, val: u8) {
        match addr {
            ZERO_PAGE_BEG...ZERO_PAGE_END => {
                self.zero_page[addr as usize] = val
            },
            STACK_BEG...STACK_END => {
                let rel_addr = (addr - STACK_BEG) as usize;
                self.stack[rel_addr] = val
            },
            RAM_BEG...RAM_END => {
                let rel_addr = (addr - RAM_BEG) as usize;
                self.ram[rel_addr] = val
            }
            _ => panic!("Improper attempt to write to RAM at addr: {:#X}", addr)
        }
    }
}
