#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate nom;
extern crate clap;
extern crate sdl2;

use std::path::Path;

use clap::{Arg, App};

use sdl2::pixels::PixelFormatEnum::RGB24;
use sdl2::render::{Texture, TextureCreator};

mod cart;
mod cpu;
mod debugger;
mod interconnect;
mod nes;
mod ppu;
mod rom;
mod screen;

use cart::Cartridge;
use debugger::Debugger;
use nes::Nes;
use screen::Screen;
use screen::{NES_WIDTH, NES_HEIGHT};

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
            let mut sdl = sdl2::init().unwrap();

            // Setup SDL here because the lifetime crap for lib is ridiculous
            let video_subsystem = sdl.video().unwrap();
            let window = video_subsystem.window("GadgetNES", NES_WIDTH, NES_HEIGHT)
                .position_centered()
                .build()
                .unwrap();

            let canvas = window.into_canvas().build().unwrap();
            let texture_creator = canvas.texture_creator();

            let mut screen = Screen::new(canvas, &texture_creator);

            let mut nes = Nes::new(&mut cart, screen);

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
