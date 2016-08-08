use cpu::Cpu;
use super::sdl::{SDLInterface, Input, ScreenSize};
use interconnect::Interconnect;

use std::thread::sleep_ms;

#[derive(PartialEq)]
enum GameState {
    Run,
    Quit,
}

pub struct Nes<'a> {
    cpu: Cpu,
    interconnect: Interconnect,
    sdl_interface: SDLInterface<'a>,
}

impl<'a> Nes<'a> {
    pub fn new(cart_rom: &Vec<u8>, scale: ScreenSize) -> Nes<'a> {
        Nes {
            cpu: Cpu::new(),
            interconnect: Interconnect::new(&cart_rom),
            sdl_interface: SDLInterface::new(scale),
        }
    }

    pub fn power_up (&mut self, cart_rom: Vec<u8>) {
        self.cpu.power_up();
        self.interconnect.power_up(cart_rom);
    }

    pub fn run(&mut self) {
        let mut game_state = GameState::Run;
        self.sdl_interface.load_bmp("assets/GadgetNES.bmp");
        sleep_ms(1500);

        while game_state != GameState::Quit {
            self.sdl_interface.set_clear_color(255, 0, 0);
            self.sdl_interface.display();
            game_state = match self.sdl_interface.check_input() {
                Input::Quit => { GameState::Quit },
                Input::Continue => { GameState::Run },
            }
        }

        loop {
            self.cpu.run_instr(&mut self.interconnect);
        }
    }
}
