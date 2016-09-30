use cpu::{Cpu, Interrupt};
use ppu::ppu::Ppu;
use super::sdl::{SDLInterface, Input, ScreenSize};
use interconnect::Interconnect;

use std::thread::sleep;
use std::time::Duration;

#[derive(PartialEq)]
enum GameState {
    Run,
    Quit,
}

pub struct Nes<'a> {
    cpu: Cpu,
    ppu: Ppu,
    interconnect: Interconnect,
    sdl_interface: SDLInterface<'a>,
}

impl<'a> Nes<'a> {
    pub fn new(cart_rom: &Vec<u8>, scale: ScreenSize) -> Nes<'a> {
        Nes {
            cpu: Cpu::new(),
            ppu: Ppu::new(),
            interconnect: Interconnect::new(cart_rom),
            sdl_interface: SDLInterface::new(scale),
        }
    }

    pub fn power_up(&mut self, cart_rom: Vec<u8>) {
        self.cpu.power_up();
        self.ppu.power_up();
        self.interconnect.power_up(cart_rom);
    }

    pub fn run(&mut self) {
        let mut game_state = GameState::Run;
        self.sdl_interface.load_bmp("assets/GadgetNES.bmp");
        sleep(Duration::from_millis(1500));

        while game_state != GameState::Quit {
            self.cpu.run_instr(&mut self.interconnect, &mut self.ppu);
            let vblank = self.ppu.step(&mut self.interconnect, &self.cpu.cycles);
            if vblank {
                self.cpu.interrupt(&mut self.interconnect, &mut self.ppu, Interrupt::NMI);
                println!("***************DISPLAY FRAME*************");
                self.sdl_interface.display_frame(&self.ppu.frame);
            }
            game_state = match self.sdl_interface.check_input() {
                Input::Quit => GameState::Quit,
                Input::Continue => GameState::Run,
            }
        }
    }
}
