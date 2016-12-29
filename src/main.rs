extern crate sdl2;

use std::env;
use self::nes::Nes;

mod ines;
mod nes;

// TODO: Add other ROM types in the future
use self::ines::InesRom;

fn main() {
    let rom_name = env::args().nth(1).unwrap();
    let rom = InesRom::new(rom_name);

    let mut nes = Nes::init(&rom);
    nes.run();
}
