use std::fmt;

use super::nes::KILOBYTE;
use super::cart::Cartridge;

const LAST_VISIBLE: u8 = 239; // 240 Total, 0 indexed
const LAST_VBLANK: u8 = 20;

pub struct Ppu {
    registers: Registers,
    cycles: usize,
    current_scanline: Scanline,
    pixel_shift_register: [u8; 2],
    ram: [u8; 2 * KILOBYTE],
    oam: [u8; 256],
    position: Position,
}

// Register names match what's listed on NESDevWiki
#[derive(Default)]
struct Registers {
    ppuctrl: PpuCtrl,
    ppumask: PpuMask,
    ppustatus: PpuStatus,
    oamaddr: u8,
    ppuscroll: u8,
    ppuaddr: u8,
    ppudata: u8,
    oamdma: u8,
}

pub enum Scanline {
    PreRender,
    Visible(u8),
    PostRender,
    VBlank(u8),
}

bitflags! {
    #[derive(Default)]
    struct PpuCtrl: u8 {
        const NAMETABLE           = 0b00000011;
        const VRAM_ADDR_INCREMENT = 0b00000100;
        const SPRITE_TABLE        = 0b00001000;
        const BG_TABLE            = 0b00010000;
        const SPRITE_SIZE         = 0b00100000;
        const PPU_MASTER          = 0b01000000;
        const NMI                 = 0b10000000;
    }
}

bitflags! {
    #[derive(Default)]
    struct PpuMask: u8 {
        const GREYSCALE        = 0b00000001;
        const SHOW_LEFT_BG     = 0b00000010;
        const SHOW_LEFT_SPRITE = 0b00000100;
        const SHOW_BG          = 0b00001000;
        const SHOW_SPRITES     = 0b00010000;
        const EMPH_RED         = 0b00100000;
        const EMPH_GREEN       = 0b01000000;
        const EMPH_BLUE        = 0b10000000;
    }
}

bitflags! {
    #[derive(Default)]
    struct PpuStatus: u8 {
        const LAST_WRITTEN_BITS = 0b00011111;
        const SPRITE_OVRFLW     = 0b00100000;
        const SPRITE_0_HIT      = 0b01000000;
        const VBLANK            = 0b10000000;
    }
}

#[derive(Default)]
struct Position {
    coarse_x: u8,
    coarse_y: u8,
}

impl Scanline {
    fn next(&self) -> Scanline {
        use self::Scanline::*;
        match *self {
            PreRender => Visible(0),
            Visible(LAST_VISIBLE) => PostRender,
            Visible(line) => Visible(line + 1),
            PostRender => VBlank(0),
            VBlank(LAST_VBLANK) => PreRender,
            VBlank(line) => Visible(line + 1),
        }
    }
}

impl Position {
    fn bump_x(&mut self) {}
    fn bump_y(&mut self) {}
}

impl Ppu {
    pub fn new() -> Self {
        Ppu {
            registers: Registers::default(),
            cycles: 0,
            current_scanline: Scanline::Visible(0),
            pixel_shift_register: [0; 2],
            ram: [0u8; 2*KILOBYTE],
            oam: [0u8; 256],
            position: Position::default(),
        }
    }

    pub fn reset(&mut self) {
        self.registers.ppuctrl = PpuCtrl::empty();
        self.registers.ppumask = PpuMask::empty();
        self.registers.ppuscroll = 0;
        self.registers.ppudata = 0;
    }

    pub fn step(&mut self, cart: &mut Cartridge, cpu_cycles: u8) {
        use self::Scanline::*;
        println!("{:?}", self);

        match self.current_scanline {
            PreRender => self.prerender(),
            Visible(line) => self.visible(line),
            PostRender => self.postrender(),
            VBlank(line) => self.vblank(line)
        }
        self.cycles += 3;
    }

    fn prerender(&mut self) {
        if self.cycles >= 341 {
            self.cycles = 0;
            self.current_scanline = self.current_scanline.next();
        }
    }

    fn visible(&mut self, line: u8) {
        if self.cycles >= 341 {
            self.cycles = 0;
            self.current_scanline = self.current_scanline.next();
        }
    }

    fn postrender(&mut self) {
        if self.cycles >= 341 {
            self.cycles = 0;
            self.current_scanline = self.current_scanline.next();
        }
    }

    fn vblank(&mut self, line: u8) {
        if self.cycles >= 341 {
            self.cycles = 0;
            self.current_scanline = self.current_scanline.next();
        }
    }

    // Methods for debugger
}

impl fmt::Debug for Ppu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SCANLINE: {:?} {:?} COARSE X: {:03} COARSE Y: {:03} CYCLES: {:03}",
            self.current_scanline,
            self.registers,
            self.position.coarse_x,
            self.position.coarse_y,
            self.cycles
        )
    }
}

impl fmt::Debug for Scanline {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let debug_str: String = match self {
                &Scanline::PreRender => "PreRender".to_owned(),
                &Scanline::Visible(line) => format!("Visible({:03})", line),
                &Scanline::PostRender => "PostRender".to_owned(),
                &Scanline::VBlank(line) => format!("VBlank({:03})", line),
        };
        write!(f, "{:}", debug_str.as_str())
    }
}

impl fmt::Debug for Registers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PPUCTRL: {:08b} PPUMASK: {:08b} PPUSTATUS: {:08b} OAMADDR: {:02X} PPUSCROLL: {:08b} PPUADDR: {:02X} PPUDATA: {:02X} OAMDMA: {:02X}",
            self.ppuctrl.bits(),
            self.ppumask.bits(),
            self.ppustatus.bits(),
            self.oamaddr,
            self.ppuscroll,
            self.ppuaddr,
            self.ppudata,
            self.oamdma
        )
    }
}
