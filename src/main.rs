#[macro_use]
extern crate bitflags;

mod cart;
mod cpu;
mod nes;
mod rom;

use cart::Cartridge;
use nes::Nes;
use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Expected 1 parameter. Got {:}", args.len() - 1);
        return;
    }

    let rom_path = Path::new(&args[1]);
    match rom::read_rom(rom_path) {
        Ok(rom) => {
            let mut cart = Cartridge::new(rom);
            let mut nes = Nes::new(&mut cart);
            nes.reset();
            nes.run();
        }
        Err(e) => println!("{}", e),
    }
}
