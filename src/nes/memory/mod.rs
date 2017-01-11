//! Defines the various kinds of memory in the NES and provides an interface
//! for them,
use std::rc::Rc;
use std::cell::RefCell;

use nes::MemMapped;
use nes::ppu::Ppu;
//use nes::apu::Apu; // TODO
use nes::io::Io;
use ines::InesRom;

use sdl2::Sdl;

pub mod mapper;
use self::mapper::*;

mod constants;
use self::constants::*;

// TODO: Add CHR
/// This is a raw container of the various memories in the NES.
/// This does NOT include memory mapped I/O.
/// Things that are mapped are handled by their respective components
/// memory_map modules.
pub struct Memory<'a> {
    pub ppu: Ppu<'a>,
    // apu: Apu, // TODO
    io:  Io,
    ram: Ram,
    // exp: ExpansionRom // TODO
    // sram: Sram // TODO
    prg: Rc<RefCell<Box<Mapper>>>,
}

impl<'a> Memory<'a> {
    pub fn new(rom: &InesRom, sdl_context: &Sdl) -> Memory<'a> {
        let mapper = choose_mapper(rom);
        let mapper = Rc::new(RefCell::new(mapper));
        Memory {
            ppu: Ppu::new(mapper.clone(), sdl_context, rom.mirroring()),
            // apu: Apu::new(), // TODO
            io:  Io::new(),
            ram: Ram::new(),
            prg: mapper.clone(),
        }
    }

    pub fn read_ppu_byte(&mut self, addr: u16) -> u8 {
        self.ppu.read_byte(addr)
    }
    
    pub fn write_ppu_byte(&mut self, addr: u16, val: u8) {
        self.ppu.write_byte(addr, val);
    }

    pub fn read_io_byte(&mut self, addr: u16) -> u8 {
        self.io.read_byte(addr)
    }
    
    pub fn write_io_byte(&mut self, addr: u16, val: u8) {
        self.io.write_byte(addr, val);
    }

    /// Reads a byte from RAM.
    pub fn read_ram_byte(&mut self, addr: u16) -> u8 {
        self.ram.read_byte(addr)
    }

    /// Writes a byte to RAM.
    pub fn write_ram_byte(&mut self, addr: u16, val: u8) {
        self.ram.write_byte(addr, val);
    }

    /// Reads a byte from PRG-ROM as defined by the mapper.
    pub fn read_rom_byte(&self, addr: u16) -> u8 {
        self.prg.borrow().read_rom_byte(addr)
    }
}

/// Provides an interface for the CPU's RAM
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
    /// Reads a byte from RAM.
    ///
    /// #Panics
    /// Will panic if the address is not in RAM.
    fn read_byte(&mut self, addr: u16) -> u8 {
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

    /// Writes a byte to RAM.
    ///
    /// #Panics
    /// Will panic if the address is not in RAM.
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
