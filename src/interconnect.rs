//! This module gathers all NES elements excluding the cpu

use super::ppu::Ppu;
use super::cart::Cartridge;

pub struct Interconnect<'a> {
    pub ppu: Ppu,
    pub cart: &'a mut Cartridge
}

impl<'a> Interconnect<'a> {
    pub fn new(cart: &'a mut Cartridge) -> Self {
        Interconnect {
            ppu: Ppu::new(),
            cart: cart,
        }
    }

    pub fn reset(&mut self) {
        self.ppu.reset();
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            0x2000...0x2007 => unimplemented!(), // PPU registers
            0x2008...0x3FFF => unimplemented!(), // PPU register mirrors
            0x4000...0x4017 => unimplemented!(), // APU registers
            0x4018...0x401F => panic!("These registers are disabled during normal operation"),
            0x4020...0xFFFF => self.cart.prg_read(addr),
            _ => panic!("Unrecognized interconnect address: {:04X}", addr),
        }
    }

    pub fn write_byte(&mut self, addr: u16, val: u8) {
        match addr {
            0x2000...0x2007 => unimplemented!(), // PPU registers
            0x2008...0x3FFF => unimplemented!(), // PPU register mirrors
            0x4000...0x4017 => unimplemented!(), // APU registers
            0x4018...0x401F => panic!("These registers are disabled during normal operation"),
            0x4020...0xFFFF => self.cart.prg_write(addr, val),
            _ => panic!("Unrecognized interconnect address: {:04X}", addr),
        };
    }
}
