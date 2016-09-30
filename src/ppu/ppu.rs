use std::fmt;
use super::super::interconnect::Interconnect;

const SCREEN_WIDTH: usize = 256;
const SCREEN_HEIGHT: usize = 240;
const CPU_CYCLES_PER_SCANLINE: u64 = 114;
const LAST_VISIBLE_SCANLINE: u8 = 239;
const VBLANK_SCANLINES: u64 = 20;

const VBLANK_FLAG: u8 = (1 << 7);

// Shamlessly copied from SprocketNES...too much typing
static PALETTE: [u8; 192] =
    [124, 124, 124, 0, 0, 252, 0, 0, 188, 68, 40, 188, 148, 0, 132, 168, 0, 32, 168, 16, 0, 136,
     20, 0, 80, 48, 0, 0, 120, 0, 0, 104, 0, 0, 88, 0, 0, 64, 88, 0, 0, 0, 0, 0, 0, 0, 0, 0, 188,
     188, 188, 0, 120, 248, 0, 88, 248, 104, 68, 252, 216, 0, 204, 228, 0, 88, 248, 56, 0, 228,
     92, 16, 172, 124, 0, 0, 184, 0, 0, 168, 0, 0, 168, 68, 0, 136, 136, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 248, 248, 248, 60, 188, 252, 104, 136, 252, 152, 120, 248, 248, 120, 248, 248, 88, 152,
     248, 120, 88, 252, 160, 68, 248, 184, 0, 184, 248, 24, 88, 216, 84, 88, 248, 152, 0, 232,
     216, 120, 120, 120, 0, 0, 0, 0, 0, 0, 252, 252, 252, 164, 228, 252, 184, 184, 248, 216, 184,
     248, 248, 184, 248, 248, 164, 192, 240, 208, 176, 252, 224, 168, 248, 216, 120, 216, 248,
     120, 184, 248, 184, 184, 248, 216, 0, 252, 252, 248, 216, 248, 0, 0, 0, 0, 0, 0];

#[derive(Debug, PartialEq)]
enum Scanline {
    PreRender,
    Visible(u8),
    PostRender,
    VBlank,
}

#[derive(Debug)]
struct RGB {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Default)]
struct BgPixelBuffer {
    pattern_low: u8,
    pattern_high: u8,
    attribute: u8,
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

    // oam: Box<[u8; 0xff]>,
    cycles: u64,
    scanline: Scanline,
    pub frame: Box<[u8; SCREEN_WIDTH * SCREEN_HEIGHT * 3]>,

    x_scroll: u16,
    y_scroll: u16,
    nametable: u16,
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

            // oam: Box::new([0u8; 0xff]),
            cycles: 0,
            scanline: Scanline::PreRender,
            frame: Box::new([0u8; SCREEN_WIDTH * SCREEN_HEIGHT * 3]),

            x_scroll: 0,
            y_scroll: 0,
            nametable: 0x2000,
        }
    }

    pub fn read_reg(&self, addr: u16) -> u8 {
        if addr == 0x4014 {
            self.oamdma
        } else {
            match addr & 1 << 0 {
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
            match addr & 1 << 0 {
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

    // Returns whether the NMI is currently allowed
    pub fn step(&mut self, interconnect: &mut Interconnect, current_cpu_cycle: &u64) -> bool {
        let mut vblank = false;
        while self.cycles < *current_cpu_cycle {
            match self.scanline {
                Scanline::PreRender => {
                    self.prerender();
                    vblank = false;
                }
                Scanline::Visible(line) => {
                    self.render_scanline(interconnect, line);
                    vblank = false;
                }
                Scanline::PostRender => {
                    self.postrender();
                    vblank = false;
                }
                Scanline::VBlank => {
                    println!("###################### V Blank ########################");
                    self.vblank(&mut vblank);
                    vblank = true;
                }
            }
            self.cycles += CPU_CYCLES_PER_SCANLINE; // It's easier to just deal in cpu cycles.
        }
        vblank
    }

    fn prerender(&mut self) {
        self.set_vblank(false);
        self.scanline = Scanline::Visible(0);
        self.cycles += CPU_CYCLES_PER_SCANLINE;
    }

    fn render_scanline(&mut self, interconnect: &mut Interconnect, line: u8) {
        // TODO: Refactor
        println!("################# Rendering scanline ##################: {:?}",
                 self.scanline);
        let mut bg_pixel_buffer = BgPixelBuffer::default();

        for x in 0..SCREEN_WIDTH {
            // I could probably just buffer the whole scanline, but I think this is more accurate
            if x % 8 == 0 {
                bg_pixel_buffer = self.refresh_buffer(interconnect);
            }
            //let background_pixel = self.make_background_pixel(interconnect, &bg_pixel_buffer, x as u8);
            let y = self.y_scroll as usize;
            //self.putpixel(x, y, background_pixel);
            self.x_scroll += 1;
        }

        self.cycles += CPU_CYCLES_PER_SCANLINE;
        self.y_scroll += 1;
        self.x_scroll = 0;
        if self.scanline == Scanline::Visible(LAST_VISIBLE_SCANLINE) {
            self.scanline = Scanline::PostRender;
        } else {
            self.scanline = Scanline::Visible(line + 1);
        }

    }

    fn postrender(&mut self) {
        self.scanline = Scanline::VBlank;
        self.cycles += CPU_CYCLES_PER_SCANLINE;
    }

    fn vblank(&mut self, vblank_nmi: &mut bool) {
        self.set_vblank(true);
        self.cycles += CPU_CYCLES_PER_SCANLINE * VBLANK_SCANLINES;
        self.scanline = Scanline::PreRender;
        self.x_scroll = 0;
        self.y_scroll = 0;
        *vblank_nmi = self.throw_nmi();
    }

    fn set_vblank(&mut self, status: bool) {
        match status {
            true => {
                self.ppustatus |= VBLANK_FLAG;
            }
            false => {
                self.ppustatus &= !VBLANK_FLAG;
            }
        }
    }

    fn throw_nmi(&self) -> bool {
        if (self.ppuctrl & (1 << 7)) != 0 {
            true
        } else {
            false
        }
    }

    fn refresh_buffer(&mut self, interconnect: &Interconnect) -> BgPixelBuffer {
        BgPixelBuffer {
            pattern_low: 0,
            pattern_high: 0,
            attribute: 0,
        }
    }

    /*fn make_background_pixel(&mut self, interconnect: &Interconnect, buffer: &BgPixelBuffer, x: u8) -> RGB {
        RGB {
            red: 0,
            green: 0,
            blue: 0,
        };

        println!("pattern_color: {:#b}", pattern_color);

        let tile_color = (attr_color << 2) | pattern_color;
        println!("Tile Color: {:#b}", tile_color);

        let palette_index = interconnect.ppu_read_byte(0x3f00 + (tile_color as u16)) & 0x3f;
        println!("palette index: {:#X}", palette_index);
        println!("RGB: {:?}", self.get_color(palette_index));
        self.get_color(palette_index)
    }*/

    fn get_color(&self, palette_index: u8) -> RGB {
        RGB {
            red: PALETTE[palette_index as usize * 3 + 2],
            green: PALETTE[palette_index as usize * 3 + 1],
            blue: PALETTE[palette_index as usize * 3 + 0],
        }
    }

    fn putpixel(&mut self, x: usize, y: usize, color: RGB) {
        self.frame[(y * SCREEN_WIDTH + x) * 3 + 0] = color.red;
        self.frame[(y * SCREEN_WIDTH + x) * 3 + 1] = color.green;
        self.frame[(y * SCREEN_WIDTH + x) * 3 + 2] = color.blue;
    }

    fn fetch_nametable_byte(&mut self, interconnect: &Interconnect) -> u8 {
        0
    }

    fn fetch_attribute(&mut self, interconnect: &Interconnect) -> u8 {
        0
    }
    fn fetch_pattern(&mut self, interconnect: &Interconnect, nametable_byte: u8) -> (u8, u8) {
        (0,0)
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
