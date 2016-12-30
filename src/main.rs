use std::env;
use self::nes::Nes;

mod nes;
mod ines;
mod graphics;

use self::ines::InesRom;
use self::graphics::GraphicsInterface;

fn main() {
    let rom_name = env::args().nth(1).unwrap();
    let rom = InesRom::new(rom_name);
    let (mut graphics_interface, sdl) = GraphicsInterface::new();

    let mut nes = Nes::init(&rom);

    nes.run();
}
