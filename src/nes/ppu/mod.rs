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

const CPU_CYCLES_PER_SCANLINE: u32 = 114;
const LAST_VISIBLE_SCANLINE: u8 = 239;
const BYTES_PER_SCANLINE: usize = 32; // Number of rendered bytes per scanline
const VBLANK_LINES: u32 = 20;

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

/// The NES Picture Processing Unit or PPU.
pub struct Ppu<'a> {
    /*############################## Registers ###############################*/
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

    /// One of two buffer of pixel information, meant to model the
    /// PPU's internal latches. Buffer 1 is the initial buffer filled.
    bg_buffer_1: BgPixelBuffer,
    /// Buffer 2 is the buffer which is about to be rendered.
    bg_buffer_2: BgPixelBuffer,
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
    VBlank,
}

#[derive(Copy, Clone)]
pub struct Pixel {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Default for Pixel {
    fn default() -> Pixel {
        Pixel {
            red: 255,
            green: 255,
            blue: 255,
        }
    }
}

/// The NES had two latches for buffering pixel data.
#[derive(Default, Copy, Clone)]
struct BgPixelBuffer {
    pixels: [Pixel; 8],
}

impl BgPixelBuffer {
    // From NES Dev wiki
    pub fn refresh_buffer(&mut self, addr: u16) {
        let tile_addr = 0x2000 | (addr & 0x0FFF);
        let patt_addr = 0x23C0 |
                        (addr & 0x0C00) |
                        ((addr >> 4) & 0x38) |
                        ((addr >> 2) & 0x07);
    }
}

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

            current_v_addr: 0,
            temp_v_addr: 0,
            x_scroll_fine: 0,
            write: Write::One,

            cycle: 0,
            scanline: Scanline::Visible(0),
            frame: [0; SCREEN_SIZE],

            bg_buffer_1: BgPixelBuffer::default(),
            bg_buffer_2: BgPixelBuffer::default(),

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
        #[cfg(feature="debug")]
        println!("{:?}", self);
        let mut nmi = false;
        while self.cycle < cpu_cycle {
            use self::Scanline::*;
            match self.scanline {
                PreRender => { self.prerender(); },
                Visible(line) => { self.scanline(line); },
                PostRender => { self.postrender(); },
                VBlank => { self.vblank(); nmi = true },
            };
        }
        if nmi { self.cycle = 0; }
        nmi
    }

    fn prerender(&mut self) {
        self.cycle += CPU_CYCLES_PER_SCANLINE;
        self.set_vblank(false);
        self.scanline = Scanline::Visible(0);
    }

    fn scanline(&mut self, line: u8) {
        self.cycle += CPU_CYCLES_PER_SCANLINE;

        for byte in 0..BYTES_PER_SCANLINE {
            let offset = (line as usize) * (SCREEN_WIDTH * 3 as usize) + (byte * 24);
            for bit in 0..8 {
                self.frame[offset + (3 * bit)] = self.bg_buffer_2.pixels[bit].red;
                self.frame[offset + (3 * bit) + 1] = self.bg_buffer_2.pixels[bit].green;
                self.frame[offset + (3 * bit) + 2] = self.bg_buffer_2.pixels[bit].blue;
            }
            self.coarse_x_increment();
            self.bg_buffer_2 = self.bg_buffer_1;
            let addr = self.current_v_addr;
            self.bg_buffer_1.refresh_buffer(addr);
        }
        self.coarse_y_increment();
        self.bg_buffer_2 = self.bg_buffer_1;
        let addr = self.current_v_addr;
        self.bg_buffer_1.refresh_buffer(addr);
        
        if line == LAST_VISIBLE_SCANLINE { self.scanline = Scanline::PostRender }
        else { self.scanline = Scanline::Visible(line + 1); }
    }

    // Directly from dev wiki
    // TODO: Optimize
    fn coarse_x_increment(&mut self) {
        if self.current_v_addr & 0x001F == 31 {
            self.current_v_addr &= !(0x001F);
            self.current_v_addr ^=0x0400;
        } else {
            self.current_v_addr += 1;
        }
    }

    // Directly from NES dev wiki
    fn coarse_y_increment(&mut self) {
        if (self.current_v_addr & 0x7000) != 0x7000 {
            self.current_v_addr += 0x1000;
        }
        else {
            self.current_v_addr &= !(0x7000);
            let mut y = (self.current_v_addr & 0x03E0) >> 5;
            if y == 29 {
                y = 0;
                self.current_v_addr ^= 0x0800;
            }
            else if y == 31 { y = 0; }
            else { y += 1; }
            self.current_v_addr = (self.current_v_addr & !(0x03E0)) | (y << 5);
        }
    }

    fn postrender(&mut self) {
        self.cycle += CPU_CYCLES_PER_SCANLINE;
        self.scanline = Scanline::VBlank;
    }

    fn vblank(&mut self) {
        self.cycle += VBLANK_LINES * CPU_CYCLES_PER_SCANLINE;
        self.set_vblank(true);
        self.scanline = Scanline::PreRender;
        self.graphics.display_frame(&mut self.frame);
        self.current_v_addr = self.temp_v_addr;
    }

    fn set_vblank(&mut self, set: bool) {
        match set {
            true => self.ppu_status |= 1 << 7,
            false => self.ppu_status &= !(1 << 7),
        }
    }

    /// Sets the vblank and sprite overflow bits of PPUSTATUS as this
    /// was commonly the state of the PPU after power on and warm up.
    pub fn power_up(&mut self) {
        self.ppu_status = 0b1010_0000;

    }

    /* ######################### PPUCTRL helpers ######################### */
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

    /* ######################### PPUMASK helpers ########################## */
    /* fn greyscale(&self) -> bool { self.ppu_mask & (1 << 0) != 0 }
    fn show_bg_left(&self) -> bool { self.ppu_mask & (1 << 1) != 0 }
    fn show_sprites_left(&self) -> bool { self.ppu_mask & (1 << 2) != 0 }
    fn show_bg(&self) -> bool { self.ppu_mask & (1 << 3) != 0 }
    fn show_sprites(&self) -> bool { self.ppu_mask & (1 << 4) != 0 }
    fn emph_red(&self) -> bool { self.ppu_mask & (1 << 5) != 0 }
    fn emph_gre(&self) -> bool { self.ppu_mask & (1 << 6) != 0 }
    fn emph_blu(&self) -> bool { self.ppu_mask & (1 << 7) != 0 }

    /* ######################### PPUSTATUS helpers ######################## */
    pub fn sprite_overflow(&self) -> bool { self.ppu_mask & (1 << 5) != 0 }
    pub fn sprite_0_hit(&self) -> bool { self.ppu_mask & (1 << 6) != 0 }
    pub fn gen_vblank(&self) -> bool { self.ppu_mask & (1 << 6) != 0 } */
}

// TODO: Correctly implement these
// TODO: Read is supposed to clear the W latch...
impl<'a> MemMapped for Ppu<'a> {
    fn read_byte(&mut self, addr: u16) -> u8 {
        match addr {
            0x2000 => self.ppu_ctrl,
            0x2001 => self.ppu_mask,
            0x2002 => {
                self.write = Write::One;
                let val = self.ppu_status;
                self.ppu_status &= 0x7F;
                val
            },
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
            0x2000 => {
                self.ppu_ctrl = val;
                // TODO Should I just or this in?
                self.temp_v_addr = ((val & 0b11) as u16) << 10;
            },
            0x2001 => self.ppu_mask = val,
            0x2002 => panic!("Read only register: PPUSTATUS"),
            0x2003 => self.oam_addr = val,
            0x2004 => self.oam_data = val,
            0x2005 => {
                self.ppu_scroll = val;
                match self.write {
                    Write::One => {
                        // Turn off lower five bits then OR in upper 5 of val
                        self.temp_v_addr =
                            (self.temp_v_addr & 0xFFE0) | ((val as u16) >> 3);

                        self.x_scroll_fine = val & 0b111; // Keep lower 3 bits
                        self.write = Write::Two;
                    },
                    Write::Two => {
                        // Turn off bits 4-8 and OR upper 5 of val
                        self.temp_v_addr =
                            (self.temp_v_addr & 0x0C1F) | ((val as u16) << 2);

                        // Do the same with bits 13-15 of t and lower 3 of val
                        self.temp_v_addr =
                            (self.temp_v_addr & 0x1FFF) | ((val as u16) << 12);

                        self.x_scroll_fine = val & 0b111; // Keep lower 3 bits
                        self.write = Write::One;
                    },
                }
            }
            0x2006 => {
                self.ppu_addr = val;
                match self.write {
                    Write::One => {
                        self.temp_v_addr =
                            (self.temp_v_addr & 0x00FF) |
                            ((val as u16) & 0x3F) << 9;
                        self.write = Write::Two;
                    },
                    Write::Two => {
                        self.temp_v_addr =
                            (self.temp_v_addr & 0xFF00) | val as u16;

                        self.current_v_addr = self.temp_v_addr;
                        self.write = Write::One;
                    },
                }
            }
            0x2007 => self.ppu_data = val,
            0x4014 => self.oam_addr = val,
            _ => panic!("Unrecognized PPU location: {:#04X}", addr)
        }
        
    }
}

impl fmt::Debug for BgPixelBuffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:02X},{:02X},{:02X}) \
                   ({:02X},{:02X},{:02X}) \
                   ({:02X},{:02X},{:02X}) \
                   ({:02X},{:02X},{:02X}) \
                   ({:02X},{:02X},{:02X}) \
                   ({:02X},{:02X},{:02X}) \
                   ({:02X},{:02X},{:02X}) \
                   ({:02X},{:02X},{:02X}) ",
               self.pixels[0].red, self.pixels[0].green, self.pixels[0].blue,
               self.pixels[1].red, self.pixels[1].green, self.pixels[1].blue,
               self.pixels[2].red, self.pixels[2].green, self.pixels[2].blue,
               self.pixels[3].red, self.pixels[3].green, self.pixels[3].blue,
               self.pixels[4].red, self.pixels[4].green, self.pixels[4].blue,
               self.pixels[5].red, self.pixels[5].green, self.pixels[5].blue,
               self.pixels[6].red, self.pixels[6].green, self.pixels[6].blue,
               self.pixels[7].red, self.pixels[7].green, self.pixels[7].blue)
    }
}

impl<'a> fmt::Debug for Ppu<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PPUCTRL:{:02X} PPUMASK:{:02X} PPUSTATUS:{:02X}              \
SCANLINE:{:?}    PPU CYC:{:5?}
OAMADDR:{:02X} OAMDATA:{:02X} PPUSCROLL:{:02X}              BG_BUFFER_1: {:?}
PPUADDR:{:02X} PPUDATA:{:02X} OAMDMA:   {:02X}              BG_BUFFER_2: {:?}
T:{:04X} V:{:04X} X:{:03b} W:{:?}",
               self.ppu_ctrl,
               self.ppu_mask,
               self.ppu_status,
               self.scanline,
               self.cycle,
               self.oam_addr,
               self.oam_data,
               self.ppu_scroll,
               self.bg_buffer_1,
               self.ppu_addr,
               self.ppu_data,
               self.oam_data,
               self.bg_buffer_2,
               self.temp_v_addr,
               self.current_v_addr,
               self.x_scroll_fine,
               self.write)
    }
}
