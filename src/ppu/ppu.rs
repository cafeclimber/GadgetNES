use std::fmt;

const SCREEN_WIDTH: usize = 256;
const SCREEN_HEIGHT: usize = 240;
const CPU_CYCLES_PER_SCANLINE: u64 = 114;
const IMG_SPR_MIRROR_BEG: u16 = 0x3f20;
const IMG_SPR_MIRROR_END: u16 = 0x3fff;
const MIRRORS_BEG: u16 = 0x4000;
const MIRRORS_END: u16 = 0xffff;

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

enum Scanline {
    Visible(u8),
    PostRender,
    VBlank,
    Final,
}

struct Vram {
    palette: [u8; 0x20],
    pattern_tables: [u8; 0x20],
    name_tables: [u8; 0x800],
}

impl Vram {
    // TODO: Handle Mappers
    pub fn new(cart_rom: &Vec<u8>) -> Vram {
        Vram {
            palette: [0; 0x20],
            pattern_tables: [0; 0x20],
            name_tables: [0; 0x800],
        }
    }
}

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

    vram: Vram,

    cycles: u64,
    scanline: Scanline,
    pub frame: Box<[u8; SCREEN_WIDTH*SCREEN_HEIGHT*3]>,
}

impl Ppu {
    pub fn new(cart_rom: &Vec<u8>) -> Ppu {
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

            vram: Vram::new(cart_rom),

            cycles: 0,
            scanline: Scanline::Visible(0),
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

    pub fn step(&mut self, current_cpu_cycle: &u64) {
        while self.cycles < *current_cpu_cycle {
            match self.scanline {
                Scanline::Visible(line) => {
                    self.render_scanline(line);
                },
                Scanline::PostRender => {},
                Scanline::VBlank => {
                    self.vblank();
                },
                Scanline::Final => {
                    self.scanline = Scanline::Visible(0);
                },
            }
            self.cycles += CPU_CYCLES_PER_SCANLINE; // It's easier to just deal in cpu cycles.
        }
    }

    fn render_scanline(&mut self, line: u8) {
        let mut ppu_cycle = 0;
        while ppu_cycle < 341 {
            match ppu_cycle {
                0 => {
                    ppu_cycle += 1;
                    println!("Pre-render cycle: {:?}", ppu_cycle);
                },
                1...256 => {
                    let nm_byte = self.fetch_nametable_byte(&mut ppu_cycle);
                    let attr_byte = self.fetch_attribute_byte(&mut ppu_cycle);
                    let tile_bitmap = self.fetch_tile_bitmap(&mut ppu_cycle);
                    println!("Scanline rendering cycle: {:?}", ppu_cycle);
                }, 
                257...320 => {
                    self.fetch_nametable_byte(&mut ppu_cycle);
                    self.fetch_attribute_byte(&mut ppu_cycle);
                    let tile_bitmap = self.fetch_tile_bitmap(&mut ppu_cycle);
                    println!("Fetching sprites for nect scanline: {:?}", ppu_cycle);
                },
                321...336 => {
                    let nm_byte = self.fetch_nametable_byte(&mut ppu_cycle);
                    let attr_byte = self.fetch_attribute_byte(&mut ppu_cycle);
                    let tile_bitmap = self.fetch_tile_bitmap(&mut ppu_cycle);
                    println!("Fetching first two tiles for next scanline: {:?}", ppu_cycle);
                },
                337...340 => {
                    self.fetch_nametable_byte(&mut ppu_cycle);
                    self.fetch_nametable_byte(&mut ppu_cycle);
                    println!("Unused nametable fetches: {:?}", ppu_cycle);
                },
                _ => unreachable!(),
            }
        }
        self.cycles += CPU_CYCLES_PER_SCANLINE;
        self.scanline = Scanline::Visible(line + 1);
    }

    fn vblank(&mut self) {}

    fn fetch_nametable_byte(&mut self, ppu_cycle: &mut u16) -> u8 {
        *ppu_cycle += 2;
        0
    }
    fn fetch_attribute_byte(&mut self, ppu_cycle: &mut u16) -> u8 {
        *ppu_cycle += 2;
        0
    }
    fn fetch_tile_bitmap(&mut self, ppu_cycle: &mut u16) -> u8 {
        *ppu_cycle += 4;
        0
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