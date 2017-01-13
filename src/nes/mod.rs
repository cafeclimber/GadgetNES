//! Represents all NES hardware.
mod cpu;
pub mod ppu;
//mod apu; // TODO
mod io;
mod memory;

use sdl2::Sdl;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;

use self::cpu::Cpu;
use self::cpu::Interrupt::NMI;
use self::memory::Memory;
use ines::InesRom;

use std::time;
use std::thread::sleep;

const NS_PER_FRAME: u32 = 16_670_000;

/// Contains the CPU, PPU, and Memory.
pub struct Nes<'a> {
    cpu: Cpu,
    mem: Memory<'a>, // Owns all other hardware
    state: NesState,

    event_pump: EventPump,
}

trait MemMapped {
    fn read_byte(&mut self, addr: u16) -> u8;
    fn write_byte(&mut self, addr: u16, val: u8);
}

#[derive(PartialEq)]
enum NesState {
    Running,
    Quit,
}

impl<'a> Nes<'a> {
    pub fn init(rom: &InesRom, sdl_context: &Sdl) -> Nes<'a> {

        Nes {
            cpu: Cpu::default(),
            mem: Memory::new(rom, sdl_context),
            state: NesState::Running,

            event_pump: sdl_context.event_pump().unwrap(),
        }
    }

    pub fn run(&mut self) {
        self.cpu.power_on_reset(&mut self.mem);
        self.mem.ppu.power_on_reset();
        while self.state == NesState::Running {
            self.cpu.step(&mut self.mem);

            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        self.state = NesState::Quit;
                    },
                    _ => {}
                }
            }
        }
    }
}
