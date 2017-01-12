//! This module provides an interface for the 6502 as used in the NES.
use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;

use sdl2::Sdl;

use nes::MemMapped;
use nes::memory::mapper::Mapper;
use graphics::Graphics;
use graphics::{SCREEN_WIDTH, SCREEN_SIZE};

mod memory_map;
use self::memory_map::MemoryMap;

const CPU_CYCLES_PER_SCANLINE: u32 = 114;
const LAST_VISIBLE_SCANLINE: u8 = 239;
const BYTES_PER_SCANLINE: usize = 32; // Number of rendered bytes per scanline
const LAST_VBLANK_LINE: u8 = 19; // For timing.

// From thealmightyguru.com
// TODO: Is an array of tuples usable
const PALETTE: [(u8, u8, u8); 64] = [
    (124, 124, 124),
    (  0,   0, 252),
    (  0,   0, 188),
    ( 68,  40, 188),
    (148,   0, 132),
    (168,   0,  32),
    (168,  16,   0),
    (136,  20,   0),
    ( 80,  48,   0),
    (  0,  20,   0),
    (  0, 104,   0),
    (  0,  88,   0),
    (  0,  64,  88),
    (  0,   0,   0),
    (  0,   0,   0),
    (  0,   0,   0),
    (188, 188, 188),
    (  0, 120, 248),
    (  0,  88, 248),
    (104,  68, 252),
    (216,   0, 204),
    (228,   0,  88),
    (248,  56,   0),
    (228,  92,  16),
    (172, 124,   0),
    (  0, 184,   0),
    (  0, 168,   0),
    (  0, 168,  68),
    (  0, 136, 136),
    (  0,   0,   0),
    (  0,   0,   0),
    (  0,   0,   0),
    (248, 248, 248),
    ( 60, 188, 252),
    (104, 136, 252),
    (152, 120, 248),
    (248, 120, 248),
    (248,  88, 152),
    (248, 120,  88),
    (252, 160,  68),
    (248, 184,   0),
    (184, 248,  24),
    ( 88, 216,  84),
    ( 88, 248, 152),
    (  0, 232, 216),
    (120, 120, 120),
    (  0,   0,   0),
    (  0,   0,   0),
    (252, 252, 252),
    (164, 228, 252),
    (184, 184, 248),
    (216, 184, 248),
    (248, 184, 248),
    (248, 164, 192),
    (240, 208, 176),
    (252, 224, 168),
    (248, 216, 120),
    (216, 248, 120),
    (184, 248, 184),
    (184, 248, 216),
    (  0, 252, 252),
    (248, 216, 248),
    (  0,   0,   0),
    (  0,   0,   0),
];

#[derive(Debug)]
enum Write {
    One,
    Two,
}

#[derive(Default, Debug)]
struct PpuCtrl(u8);
#[derive(Default, Debug)]
struct PpuMask(u8);

#[derive(Default, Debug)]
struct PpuStatus(u8);

impl PpuStatus {
    fn reset(&mut self) {
        *self = PpuStatus(0b1010_0000)
    }

    fn read(&mut self) -> u8 {
        let byte = self.0;
        *self = PpuStatus(byte & 0x7F); // Turn off bit 7
        byte
    }
}

#[derive(Default, Debug)]
struct OamAddr(u8);

#[derive(Default, Debug)]
struct OamData(u8);

impl OamData {
    fn read(&self) -> u8 {
        // TODO: Read @oam addr then increment
        0
    }
}

#[derive(Default, Debug)]
struct PpuScroll(u8);
#[derive(Default, Debug)]
struct PpuAddr(u8);

#[derive(Default, Debug)]
struct PpuData(u8);

impl PpuData {
    fn read(&self) -> u8 {
        // TODO: Read @Vram addr then increment
        0
    }
}

#[derive(Default, Debug)]
struct OamDma(u8);

/// The NES Picture Processing Unit or PPU.
pub struct Ppu<'a> {
    /*############################## Registers ###############################*/
    /// CPU:$2000.
    /// Contains a number of flags used in controlling the PPU.
    ppu_ctrl: PpuCtrl,
    /// CPU:$2001.
    /// Controls rendering of sprites and backgrounds as well as color effects.
    ppu_mask: PpuMask,
    /// CPU:$2002.
    /// Contains information regarding the state of the PPU.
    ppu_status: PpuStatus,
    /// CPU:$2003.
    /// The address of OAM to access.
    oam_addr: OamAddr,
    /// CPU:$2004.
    /// OAM data is written here.
    oam_data: OamData,
    /// CPU:$2005.
    /// Changes the scroll position.
    ppu_scroll: PpuScroll,
    /// CPU:$2006.
    /// This is how the CPU interacts with PPU memory. CPU sets the address here.
    ppu_addr: PpuAddr,
    /// CPU:$2007.
    /// And reads/writes occur here.
    ppu_data: PpuData,

    /// CPU:$4014.
    /// How large amounts of data are transferred quickly.
    oam_dma: OamDma,

    // Using nomenclature from nesdev wiki
    /// 15 bits keeps track of the current VRAM address.
    current_v_addr: u16,
    /// Address of top left on screen tile.
    temp_v_addr: u16,
    /// Fine X scroll
    x_scroll_fine: u8,
    /// Reflects wheter this is the first or second write to a register
    write: Write,
    /*########################################################################*/

    /*################################ State #################################*/
    /// Which cycle the PPU is on (kept in terms of CPU cycles)
    cycle: u32,
    /// Which scanline the PPU is currently handling
    scanline: Scanline,
    /// Essentially a frame buffer for SDL
    frame: [u8; SCREEN_SIZE],
    /*########################################################################*/

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
    VBlank(u8),
}

impl Scanline {
    fn next(&self) -> Scanline {
        match *self {
            Scanline::PreRender => Scanline::Visible(0),
            Scanline::Visible(LAST_VISIBLE_SCANLINE) => Scanline::PostRender,
            Scanline::Visible(line) => Scanline::Visible(line + 1),
            Scanline::PostRender => Scanline::VBlank(0),
            Scanline::VBlank(LAST_VBLANK_LINE) => Scanline::PreRender,
            Scanline::VBlank(line) => Scanline::VBlank(line + 1),
        }
    }
}

impl<'a> Ppu<'a> {
    pub fn new(mapper: Rc<RefCell<Box<Mapper>>>,
               sdl_context: &Sdl,
               mirroring: Mirroring)
               -> Ppu<'a>
    {
        Ppu {
            ppu_ctrl: PpuCtrl::default(),
            ppu_mask: PpuMask::default(),
            ppu_status: PpuStatus::default(),
            oam_addr: OamAddr::default(),
            oam_data: OamData::default(),
            ppu_scroll: PpuScroll::default(),
            ppu_addr: PpuAddr::default(),
            ppu_data: PpuData::default(),
            oam_dma: OamDma::default(),

            current_v_addr: 0,
            temp_v_addr: 0,
            x_scroll_fine: 0,
            write: Write::One,

            cycle: 0,
            scanline: Scanline::Visible(0),
            frame: [0; SCREEN_SIZE],

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
        #[cfg(feature="debug_ppu")]
        println!("{:?}", self);
        let mut nmi = false;
        while self.cycle < cpu_cycle {
            use self::Scanline::*;
            match self.scanline {
                PreRender => { self.prerender(); },
                Visible(line) => { self.scanline(line); },
                PostRender => { self.postrender(); },
                VBlank(0) => { nmi = self.vblank(); },
                VBlank(_) => { self.cycle += CPU_CYCLES_PER_SCANLINE; },
            };
            self.scanline = self.scanline.next();
        }
        if nmi { self.cycle = 0; }
        nmi
    }

    fn prerender(&mut self) {
    }

    fn scanline(&mut self, line: u8) {
        // TODO
    }

    fn postrender(&mut self) {
        // TODO
    }

    fn vblank(&mut self) -> bool {
        // TODO
        false
    }

    /// Sets the vblank and sprite overflow bits of PPUSTATUS as this
    /// was commonly the state of the PPU after power on and warm up.
    pub fn power_on_reset(&mut self) {
        self.ppu_status.reset();
    }
}

impl<'a> MemMapped for Ppu<'a> {
    fn read_byte(&mut self, addr: u16) -> u8 {
        match addr {
            0x2000 => 0,
            0x2001 => 0,
            0x2002 => self.ppu_status.read(),
            0x2003 => 0,
            0x2004 => self.oam_data.read(),
            0x2005 => 0,
            0x2006 => 0,
            0x2007 => self.ppu_data.read(),
            0x4014 => 0,
            _ => panic!("Invalid PPU read: {:04X}", addr),
        }
    }

    fn write_byte(&mut self, addr: u16, val: u8) {
        // TODO
    }
}

impl<'a> fmt::Debug for Ppu<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PPUCTRL:{:02X} PPUMASK:{:02X} PPUSTATUS:{:02X}              \
SCANLINE:{:?}    PPU CYC:{:5?}
OAMADDR:{:02X} OAMDATA:{:02X} PPUSCROLL:{:02X}
PPUADDR:{:02X} PPUDATA:{:02X} OAMDMA:   {:02X}
T:{:04X} V:{:04X} X:{:03b} W:{:?}",
               self.ppu_ctrl.0,
               self.ppu_mask.0,
               self.ppu_status.0,
               self.scanline,
               self.cycle,
               self.oam_addr.0,
               self.oam_data.0,
               self.ppu_scroll.0,
               self.ppu_addr.0,
               self.ppu_data.0,
               self.oam_data.0,
               self.temp_v_addr,
               self.current_v_addr,
               self.x_scroll_fine,
               self.write)
    }
}
