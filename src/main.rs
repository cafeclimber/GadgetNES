#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate nom;
extern crate clap;

mod cart;
mod cpu;
mod nes;
mod rom;
mod debugger;

use std::path::Path;

use clap::{Arg, App};

use cart::Cartridge;
use debugger::Debugger;
use nes::Nes;

fn main() {
    let matches = App::new("GadgetNES")
                        .version("0.3")
                        .author("Ryan Campbell<rdcampbell1990@gmail.com>")
                        .about("Simplistic NES emulator written in Rust")
                        .arg(Arg::with_name("ROM")
                            .short("r")
                            .long("rom")
                            .value_name("ROM")
                            .help("Path to the rom")
                            .takes_value(true)
                            .required(true))
                        .arg(Arg::with_name("DEBUGGER")
                            .short("d")
                            .long("debug")
                            .help("Runs the emulator with the internal debugger"))
                        .get_matches();

    let rom_path = Path::new(matches.value_of("ROM").unwrap());
    match rom::read_rom(rom_path) {
        Ok(rom) => {
            let mut cart = Cartridge::new(rom);
            let mut nes = Nes::new(&mut cart);
            if matches.is_present("DEBUGGER") {
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
