extern crate sdl2;
#[macro_use]
extern crate bitflags;

use std::env;
use self::nes::Nes;

mod nes;
mod ines;
mod graphics;

use self::ines::InesRom;

fn main() {
    let rom_name = env::args().nth(1).unwrap();
    #[cfg(not(feature="debug_cpu"))]
    println!("Loading: {:?}", rom_name);
    let rom = InesRom::new(rom_name);
    #[cfg(not(feature="debug_cpu"))]
    println!("{:?}", rom);

    assert_eq!(rom.header.magic_no,
               *b"NES\x1a",
               "ERROR: File is not a vaild iNES ROM");

    let sdl_context = sdl2::init().unwrap();
    
    let mut nes = Nes::init(&rom, &sdl_context);

    nes.run();
}
