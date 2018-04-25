use std::collections::HashMap;
use std::time::Duration;

use super::cart::Cartridge;
use super::cpu::Cpu;
use super::interconnect::Interconnect;
use super::screen::Screen;

use super::sdl2::event::Event;
use super::sdl2::keyboard::Keycode;
use super::sdl2::Sdl;

pub const KILOBYTE: usize = 1024;

// Fields are public for debugger
pub struct Nes<'a> {
    // apu: Apu
    pub cpu: Cpu,
    pub interconnect: Interconnect<'a>,

    pub screen: Screen<'a>,
}

impl<'a> Nes<'a> {
    pub fn new(cart: &'a mut Cartridge, screen: Screen<'a>) -> Self {
        Nes {
            cpu: Cpu::new(),
            interconnect: Interconnect::new(cart),

            screen: screen,
        }
    }

    pub fn reset(&mut self) {
        self.cpu.reset(&mut self.interconnect);
        self.interconnect.reset();
    }

    pub fn run(&mut self, breakpoints: Option<&HashMap<usize, usize>>) {
        let mut event_pump = self.screen.sdl().event_pump().unwrap();
        let mut i = 0;
        'running: loop {
            i = (i+1) % 255;
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    _ => {}
                }
            }
            if breakpoints.is_some() {
                // TODO: Fix this
                /*{
                    if breakpoints.unwrap().values().any(|&bp| (bp as u16) == self.cpu.pc()) {
                        println!("Encountered breakpoint @ {:04X}", self.cpu.pc());
                        break;
                    };
                }*/
                self.step();
            }
            else {
                self.step();
            }
            self.screen.refresh();
        }
    }

    pub fn step(&mut self) -> (u8) {
        let opcode = self.cpu.step(&mut self.interconnect);
        self.interconnect.ppu.step(&mut self.interconnect.cart, self.cpu.cycles()); 
        (opcode)
    }

}
