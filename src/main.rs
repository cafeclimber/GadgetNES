use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

mod nes;
mod cpu;
mod apu;
mod ppu;
mod cart;
mod mem_map;
mod mapper;
mod interconnect;

#[macro_use]
extern crate enum_primitive;
extern crate num;
extern crate byteorder;

fn main() {
    let rom_name = env::args().nth(1).unwrap();

    let cart_rom = read_cartridge(rom_name);
    // TODO implement header checking

    let mut nes = nes::Nes::new(&cart_rom);
    nes.power_up(cart_rom);
    nes.run();
}

// Thanks to yupferris for this!
fn read_cartridge<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let mut file = File::open(path).unwrap();
    let mut file_buf = Vec::new();
    file.read_to_end(&mut file_buf).unwrap();
    file_buf
}
