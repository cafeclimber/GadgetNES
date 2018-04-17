#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate nom;

mod cart;
mod cpu;
mod nes;
mod rom;
mod debugger;

use cart::Cartridge;
use debugger::Debugger;
use nes::Nes;
use std::env;
use std::path::Path;

const DEBUGGER: bool = true;

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
            if DEBUGGER {
                let mut debugger = Debugger::init(nes);
                debugger.run();
            } else {
                nes.reset();
                nes.run(None);
            }
        }
        Err(e) => println!("{}", e),
    }
}
