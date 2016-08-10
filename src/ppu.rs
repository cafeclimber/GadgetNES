use std::fmt;

const SCREEN_WIDTH: usize = 256;
const SCREEN_HEIGHT: usize = 240;
const CPU_CYCLES_PER_SCANLINE: u16 = 114;

// Shamlessly copied from SprocketNES...too much typing
static PALETTE: [u8; 192] = [
    124,124,124,    0,0,252,        0,0,188,        68,40,188,
    148,0,132,      168,0,32,       168,16,0,       136,20,0,
    80,48,0,        0,120,0,        0,104,0,        0,88,0,
    0,64,88,        0,0,0,          0,0,0,          0,0,0,
    188,188,188,    0,120,248,      0,88,248,       104,68,252,
    216,0,204,      228,0,88,       248,56,0,       228,92,16,
    172,124,0,      0,184,0,        0,168,0,        0,168,68,
    0,136,136,      0,0,0,          0,0,0,          0,0,0,
    248,248,248,    60,188,252,     104,136,252,    152,120,248,
    248,120,248,    248,88,152,     248,120,88,     252,160,68,
    248,184,0,      184,248,24,     88,216,84,      88,248,152,
    0,232,216,      120,120,120,    0,0,0,          0,0,0,
    252,252,252,    164,228,252,    184,184,248,    216,184,248,
    248,184,248,    248,164,192,    240,208,176,    252,224,168,
    248,216,120,    216,248,120,    184,248,184,    184,248,216,
    0,252,252,      248,216,248,    0,0,0,          0,0,0
];

pub struct Ppu {
    ppuctrl: u8,
    ppumask: u8,
    ppustatus: u8,
    oamaddr: u8,
    oamdata: u8,
    ppuscroll: u8,
    ppuaddr: u8,
    ppudata: u8,
    oamdma: u8,

    ppu_cycle: u16,
    cycles: 
    scanline: u16,
    pub frame: Box<[u8; SCREEN_WIDTH*SCREEN_HEIGHT*3]>,
}

impl Ppu {
    pub fn new() -> Ppu {
        Ppu {
            ppuctrl: 0,
            ppumask: 0,
            ppustatus: 0,
            oamaddr: 0,
            oamdata: 0,
            ppuscroll: 0,
            ppuaddr: 0,
            ppudata: 0,
            oamdma: 0,

            ppu_cycle: 0,
            scanline: 0,
            frame: Box::new([0u8; SCREEN_WIDTH*SCREEN_HEIGHT*3]),
        }
    }
    
    pub fn read_reg(&self, addr: u16) -> u8 {
        if addr == 0x4014 {
            self.oamdma
        } else {
            match addr {
                0x0 => self.ppuctrl,
                0x1 => self.ppumask,
                0x2 => self.ppustatus,
                0x3 => self.oamaddr,
                0x4 => self.oamdata,
                0x5 => self.ppuscroll,
                0x6 => self.ppuaddr,
                0x7 => self.ppudata,
                _ => panic!("Attemped access of nonexistent PPU register: {:#x}", addr),
            }
        }
    }

    pub fn write_to_reg(&mut self, addr: u16, val: u8) {
        if addr == 0x4014 {
            self.oamdma = val;
        } else {
            match addr {
                0x0 => self.ppuctrl = val,
                0x1 => self.ppumask = val,
                0x2 => self.ppustatus = val,
                0x3 => self.oamaddr = val,
                0x4 => self.oamdata = val,
                0x5 => self.ppuscroll = val,
                0x6 => self.ppuaddr = val,
                0x7 => self.ppudata = val,
                _ => panic!("Attemped write to nonexistent PPU register: {:#x}", addr),
            }
        }
    }

    pub fn power_up(&mut self) {
        self.ppustatus = 0b10100000;
    }

    fn render_scanline(&mut self) {
        for scanline_cycle in 0..340{
            match self.ppu_cycle {
                0 => {}, // Idle cyle
                1...256 => {}, // Visible
                257...320 => {}, // Sprite Pre-fetch
                321...336 => {}, // Tile Pre-fetch
                337...340 => {}, // Unused Nametable fetch
            }
        }
        self.scanline += 1;
    }

    pub fn step(&mut self, current_cpu_cycle: &u64) {
        while self.cycles < current_cpu_cycle {
            self.render_scanline();
            if self.scanline == VBLANK_SCANLINE {
                self.vblank();
            } else if self.scanline == FINAL_SCANLINE {
                self.scanline = 0;
            }
            self.cycles += CPU_CYCLES_PER_SCANLINE; // It's easier to just deal in cpu cycles.
        }
    }
}

impl fmt::Debug for Ppu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "PPU: ppuctrl: {:#x}, ppumask: {:#x}, ppustatus: {:#x}, oamaddr: {:#x}, oamdata: \
                {:#x}, ppuscroll: {:#x}, ppuaddr: {:#x},  ppudata: {:#x}, oamdma: {:#x}",
               self.ppuctrl,
               self.ppumask,
               self.ppustatus,
               self.oamaddr,
               self.oamdata,
               self.ppuscroll,
               self.ppuaddr,
               self.ppudata,
               self.oamdma)
    }
}
