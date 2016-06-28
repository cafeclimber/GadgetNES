use std::fmt;

#[derive(Default)]
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
}

impl Ppu {
    pub fn read_reg(&self, addr: u16) -> u8 {
        if addr == 0x4014 {self.oamdma} else {
            let reg_addr = (addr - 0x2000) % 8;
            match reg_addr {
                0x0 => {self.ppuctrl},
                0x1 => {self.ppumask},
                0x2 => {self.ppustatus},
                0x3 => {self.oamaddr},
                0x4 => {self.oamdata},
                0x5 => {self.ppuscroll},
                0x6 => {self.ppuaddr},
                0x7 => {self.ppudata},
                _   => panic!("Attemped access of nonexistent PPU register: {:#x}", addr),
            }
        }
    }
}

impl fmt::Debug for Ppu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PPU: ppuctrl: {:#x}, ppumask: {:#x}, ppustatus: {:#x}, oamaddr: {:#x}, oamdata: {:#x}, ppuscroll: {:#x}, ppuaddr: {:#x},  ppudata: {:#x}, oamdma: {:#x}", self.ppuctrl, self.ppumask, self.ppustatus, self.oamaddr, self.oamdata, self.ppuscroll, self.ppuaddr, self.ppudata, self.oamdma)
    }
}
