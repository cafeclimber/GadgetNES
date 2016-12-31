//! This module provides an interface for the 6502 as used in the NES.
use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;

use sdl2::Sdl;

use nes::MemMapped;
use nes::memory::mapper::Mapper;
use graphics::Graphics;
use graphics::{SCREEN_WIDTH, SCREEN_HEIGHT, SCREEN_SIZE};

mod memory_map;
use self::memory_map::MemoryMap;

/// The NES Picture Processing Unit or PPU.
pub struct Ppu<'a> {
    /// CPU:$2000.
    /// Contains a number of flags used in controlling the PPU.
    ppu_ctrl: u8,
    /// CPU:$2001.
    /// Controls rendering of sprites and backgrounds as well as color effects.
    ppu_mask: u8,
    /// CPU:$2002.
    /// Contains information regarding the state of the PPU.
    ppu_status: u8,
    /// CPU:$2003.
    /// The address of OAM to access.
    oam_addr: u8,
    /// CPU:$2004.
    /// OAM data is written here.
    oam_data: u8,
    /// CPU:$2005.
    /// Changes the scroll position.
    ppu_scroll: u8,
    /// CPU:$2006.
    /// This is how the CPU interacts with PPU memory. CPU sets the address here.
    ppu_addr: u8,
    /// CPU:$2007.
    /// And reads/writes occur here.
    ppu_data: u8,

    /// CPU:$4014.
    /// How large amounts of data are transferred quickly.
    oam_dma: u8,

    cycle: u32,
    scanline: Scanline,

    /// PPU has its own memory map which is modeled here as owning
    /// VRAM, OAM, and CHR
    memory_map: MemoryMap,

    graphics: Graphics<'a>,
}

pub enum Mirroring {
    Horizontal,
    Vertical,
    // SingleScreen,
    FourWay,
    // Other,
}

#[derive(Debug)]
enum Scanline {
    PreRender,
    Visible(u8),
    PostRender,
    VBlank,
}

// TODO: Add power_up function
impl<'a> Ppu<'a> {
    pub fn new(mapper: Rc<RefCell<Box<Mapper>>>,
               sdl_context: &Sdl,
               mirroring: Mirroring)
               -> Ppu<'a>
    {
        Ppu {
            ppu_ctrl: 0,
            ppu_mask: 0,
            ppu_status: 0,
            oam_addr: 0,
            oam_data: 0,
            ppu_scroll: 0,
            ppu_addr: 0,
            ppu_data: 0,
            oam_dma: 0,

            cycle: 0,
            scanline: Scanline::PreRender,

            memory_map: MemoryMap::new(mapper, mirroring),

            graphics: Graphics::new(sdl_context),
        }
    } 

    /// The primary interface of the PPU. On step will move
    /// the ppu forward in state by scanlines until its cycle
    /// count meets or exceeds the cpu cycle count. The primary
    /// purpose of this function is to fill the frame buffer
    /// and generate an NMI when it enters VBLANK.
    pub fn step(&mut self, cpu_cycle: u32) -> bool {
        while self.cycle < cpu_cycle {
            use self::Scanline::*;
            match self.scanline {
                PreRender => {},
                Visible(line) => {},
                PostRender => {},
                VBlank => {},
            };
        }
        #[cfg="debug"]
        println!("{:?}", self);
        false // TODO: Properly implement NMI
    }

    /// Sets the vblank and sprite overflow bits of PPUSTATUS as this
    /// was commonly the state of the PPU after power on and warm up.
    pub fn power_up(&mut self) {
        // TODO implement some kind of wait for cycles? :P
        self.ppu_status = 0b1010_0000;
    }

    /// Returns which nametable is indicated by PPUCTRL
    fn nametable_base_addr(&self) -> usize {
        match self.ppu_ctrl & 0b11 {
            0b00 => 0x2000 as usize,
            0b01 => 0x2400 as usize,
            0b10 => 0x2800 as usize,
            0b11 => 0x2C00 as usize,
            _ => unreachable!()
        }
    }

    /* ######################### PPUCTRL helpers ######################### */
    /// Bit 0 of PPUCTRL determines the increment of VRAM address to
    /// be either 1 or 32
    fn vram_increment(&self) -> usize {
        (((self.ppu_ctrl & (1 << 2)) >> 2) * 32) as usize
    }
    /// Returns the base address of the current nametable.
    fn sprite_pattern_table_base(&self) -> usize {
        (((self.ppu_ctrl & (1 << 3)) >> 3) as u16 * 0x1000) as usize
    }
    /// Returns the base address of the current background pattern table.
    fn bg_pattern_table_base(&self) -> usize {
        (((self.ppu_ctrl & (1 << 4)) >> 4) as u16 * 0x1000) as usize
    }
    /// Returns whether an NMI should be generated at the start of the next
    /// VBLANK interval.
    fn generate_nmi(&self) -> bool { self.ppu_ctrl & (1 << 7) != 0 }

    /* ######################### PPUMASK helpers ######################### */
    fn greyscale(&self) -> bool { self.ppu_mask & (1 << 0) != 0 }
    fn show_bg_left(&self) -> bool { self.ppu_mask & (1 << 1) != 0 }
    fn show_sprites_left(&self) -> bool { self.ppu_mask & (1 << 2) != 0 }
    fn show_bg(&self) -> bool { self.ppu_mask & (1 << 3) != 0 }
    fn show_sprites(&self) -> bool { self.ppu_mask & (1 << 4) != 0 }
    fn emph_red(&self) -> bool { self.ppu_mask & (1 << 5) != 0 }
    fn emph_gre(&self) -> bool { self.ppu_mask & (1 << 6) != 0 }
    fn emph_blu(&self) -> bool { self.ppu_mask & (1 << 7) != 0 }

    /* ######################### PPUSTATUS helpers ######################### */
    pub fn sprite_overflow(&self) -> bool { self.ppu_mask & (1 << 5) != 0 }
    pub fn sprite_0_hit(&self) -> bool { self.ppu_mask & (1 << 6) != 0 }
    pub fn vblank(&self) -> bool { self.ppu_mask & (1 << 6) != 0 }
}

// TODO: Correctly implement these
impl<'a> MemMapped for Ppu<'a> {
    fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            0x2000 => self.ppu_ctrl,
            0x2001 => self.ppu_mask,
            0x2002 => self.ppu_status,
            0x2003 => self.oam_addr,
            0x2004 => self.oam_data,
            0x2005 => self.ppu_scroll,
            0x2006 => self.ppu_addr,
            0x2007 => self.ppu_data,
            0x4014 => self.oam_dma,
            _ => panic!("Unrecognized PPU location: {:#04X}", addr)
        }
    }

    fn write_byte(&mut self, addr: u16, val: u8) {
        match addr {
            0x2000 => self.ppu_ctrl = val,
            0x2001 => self.ppu_mask = val,
            0x2002 => panic!("Read only register: PPUSTATUS"),
            0x2003 => self.oam_addr = val,
            0x2004 => self.oam_data = val,
            0x2005 => self.ppu_scroll = val,
            0x2006 => self.ppu_addr = val,
            0x2007 => self.ppu_data = val,
            0x4014 => self.oam_addr = val,
            _ => panic!("Unrecognized PPU location: {:#04X}", addr)
        }
        
    }
}

impl<'a> fmt::Debug for Ppu<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PPUCTRL:{:02X} PPUMASK:{:02X} PPUSTATUS:{:02X}\nOAMADDR:{:02X} OAMDATA:{:02X} PPUSCROLL:{:02X}              SCANLINE:{:?}\nPPUADDR:{:02X} PPUDATA:{:02X} OAMDMA:   {:02X}",
               self.ppu_ctrl,
               self.ppu_mask,
               self.ppu_status,
               self.oam_addr,
               self.oam_data,
               self.ppu_scroll,
               self.scanline,
               self.ppu_addr,
               self.ppu_data,
               self.oam_data)
    }
}
