use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use self::sdl::ScreenSize;

mod nes;
mod cpu;
mod apu;
mod ppu;
mod cart;
mod sdl;
mod mem_map;
mod mapper;
mod interconnect;
mod instructions;

#[macro_use]
extern crate enum_primitive;
#[macro_use]
extern crate num;

fn main() {
    let rom_name = env::args().nth(1).unwrap();
    let screen_size = match env::args().nth(2) {
        Some(ref x) if x == "default" => ScreenSize::Default,
        Some(ref x) if x == "medium" => ScreenSize::Medium,
        Some(ref x) if x == "large" => ScreenSize::Large,
        _ => panic!("Unsupported screen size"),
    };

    let cart_rom = read_cartridge(rom_name);

    let mut nes = nes::Nes::new(&cart_rom, screen_size);
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
