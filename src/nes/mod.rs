mod cpu;
mod memory;

use self::cpu::Cpu;
use self::memory::Memory;
use super::ines::InesRom;

pub struct Nes {
    cpu: Cpu,
    mem: Memory,
    state: NesState,
}

trait MemMapped {
    fn read_byte(&self, addr: u16) -> u8;
    fn write_byte(&mut self, addr: u16, val: u8);
}


#[derive(PartialEq)]
enum NesState {
    Running,
    Quit,
}

/// The primary interface for all NES components
///
/// Contains:
/// 6502 processor
/// Audio Processing Unit
/// Picture Processing Unit
/// Various data busses
impl Nes {
    pub fn init(rom: &InesRom) -> Nes {
        Nes {
            cpu: Cpu::init(),
            mem: Memory::init(rom),
            state: NesState::Running,
        }
    }

    pub fn run(&mut self) {
        while self.state == NesState::Running {
            self.cpu.step(&mut self.mem);
        }
    }
}
