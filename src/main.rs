extern crate sdl2;

use std::env;
use self::nes::Nes;
use sdl2::Sdl;

mod nes;
mod ines;
mod graphics;

use self::ines::InesRom;

fn main() {
    let rom_name = env::args().nth(1).unwrap();
    let rom = InesRom::new(rom_name);

    assert_eq!(rom.header.magic_no,
               *b"NES\x1a",
               "ERROR: File is not of iNES format");

    let sdl_context = sdl2::init().unwrap();
    
    let mut nes = Nes::init(&rom, &sdl_context);

    nes.run();
}
