use std::rc::Rc;
use std::cell::RefCell;

use nes::MemMapped;
use nes::ppu::Mirroring;

mod constants;
use self::constants::*;

use nes::memory::mapper::*;

pub struct MemoryMap {
    /// NES contains 2KiB of vram. This is where the nametables are located.
    vram: [u8; 0x800],
    /// Palette ram
    palette: [u8;0x20],
    /// PPU contains 256 bytes of object memory for sprite data.
    oam: [u8; 0x100],

    /// CHR memory (from cartridge)
    chr: Rc<RefCell<Box<Mapper>>>,

    mirroring: Mirroring,
}

impl MemoryMap {
    pub fn new(mapper: Rc<RefCell<Box<Mapper>>>,
               mirroring: Mirroring)
               -> MemoryMap
    {
        MemoryMap {
            vram: [0xFF; 0x800],
            palette: [0; 0x20],
            oam: [0; 0x100],
            chr: mapper,
            // Memory map has to be aware of nametable mirroring in order
            // to properly...map...memory
            mirroring: mirroring,
        }
    }

    /// Deals with mirrorring
    fn vram_addr(&self, addr: u16) -> u16 {
        let v_addr = match self.mirroring {
            /// 0x2000 = 0x2800 and 0x2400 = 0x2C00
            Mirroring::Horizontal => {
                match addr {
                    0x2000...0x27FF => addr - 0x2000,
                    0x2800...0x2FFF => addr - 0x2000 - 0x0800,
                    _ => panic!("Should be unreachable: {:04X}", addr)
                }
            },
            /// 0x2000 = 0x2400 and 0x2800 = 0x2C00
            Mirroring::Vertical => {
                match addr {
                    0x2000...0x23FF => addr - 0x2000,
                    0x2400...0x27FF => addr - 0x2000 - 0x400,
                    0x2800...0x2BFF => addr - 0x2000 - 0x400,
                    0x2C00...0x2FFF => addr - 0x2000 - 0x800,
                    _ => panic!("Should be unreachable: {:04X}", addr)
                }
            },
            Mirroring::FourWay => {
                panic!("Four Way mirroring not implemented")
            }
        };
        v_addr
    }
}

impl MemMapped for MemoryMap {
    fn read_byte(&mut self, addr: u16) -> u8 {
        match addr {
            // Maps both pattern tables to CHR-ROM
            // TODO: Not sure if correct
            PAT_TABLE_0_BEG...PAT_TABLE_1_END => {
                self.chr.borrow_mut().read_chr_byte(addr)
            },
            // Unclear how 4 way mirroring works
            NAMETABLE_0_BEG...NAMETABLE_3_END => {
                self.vram[self.vram_addr(addr) as usize]
            },
            NAMETABLE_MIRRORS_BEG...NAMETABLE_MIRRORS_END => {
                self.vram[self.vram_addr(addr - 0x1000) as usize]
            },
            PALETTE_RAM_BEG...PALETTE_RAM_END => {
                self.palette[(addr - PALETTE_RAM_BEG) as usize]
            },
            PALETTE_MIRRORS_BEG...PALETTE_MIRRORS_END => {0},
            _ => panic!("Invalid attempt to read PPU memory: {:04X}", addr)
        }
    }

    fn write_byte(&mut self, addr: u16, val: u8) {
        match addr {
            PAT_TABLE_0_BEG...PAT_TABLE_1_END => {
                println!("\nWARNING: Writes to CHR-ROM not supported");
            },
            NAMETABLE_0_BEG...NAMETABLE_3_END => {
                let addr = self.vram_addr(addr) as usize;
                self.vram[addr] = val;
            },
            NAMETABLE_MIRRORS_BEG...NAMETABLE_MIRRORS_END => {
                let addr = self.vram_addr(addr - 0x1000) as usize;
                self.vram[addr] = val;
            },
            PALETTE_RAM_BEG...PALETTE_RAM_END => {
                panic!("Wrote to palette ram! {:04X} = {:02X}", addr, val);
                self.palette[(addr - PALETTE_RAM_BEG) as usize] = val;
            },
            PALETTE_MIRRORS_BEG...PALETTE_MIRRORS_END => {},
            _ => panic!("Invalid attempt to write to PPU memory: {:04X}", addr)
        }
    }
}
