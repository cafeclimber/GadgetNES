use super::cart::Cartridge;
use super::nes::KILOBYTE;
use std::fmt;

pub struct Cpu<'a> {
    registers: Registers,
    cycles: u64,
    ram: [u8; 2 * KILOBYTE],
    cart: &'a Cartridge,
}

// Register names match what's listed on NESDevWiki
#[derive(Default)]
struct Registers {
    a: u8,
    x: u8,
    y: u8,
    s: u8, // Stack pointer
    p: ProcessorFlags,
    pc: u16,
}

bitflags! {
    #[derive(Default)]
    struct ProcessorFlags: u8 {
        const CARRY      = 0b00000001;
        const ZERO       = 0b00000010;
        const INTERRUPT  = 0b00000100;
        const DECIMAL    = 0b00001000;
        const STACK_COPY = 0b00110000;
        const OVERFLOW   = 0b01000000;
        const NEGATIVE   = 0b10000000;
    }
}

trait AddressingMode {
    fn load(&self, cpu: &mut Cpu) -> u8;
    fn store(&self, cpu: &mut Cpu, val: u8);
}
struct AccumulatorAM;
struct ImmediateAM;
struct ZeroPageAM {
    arg: u8,
}
struct AbsoluteAM {
    arg: u16,
}
struct RelativeAM {
    arg: u8,
}
struct ZeroPageIdxXAM {
    arg: u8,
}
struct ZeroPageIdxYAM {
    arg: u8,
}
struct AbsoluteIdxXAM {
    arg: u16,
}
struct AbsoluteIdxYAM {
    arg: u16,
}
struct IndexedIndirectAM {
    arg: u16,
}
struct IndirectIndexedAM {
    arg: u16,
}

impl AddressingMode for AccumulatorAM {
    fn load(&self, cpu: &mut Cpu) -> u8 {
        cpu.registers.a
    }
    fn store(&self, cpu: &mut Cpu, val: u8) {
        cpu.registers.a = val;
    }
}

impl AddressingMode for ImmediateAM {
    fn load(&self, cpu: &mut Cpu) -> u8 {
        cpu.load_next_byte_bump_pc()
    }
    fn store(&self, cpu: &mut Cpu, val: u8) {
        panic!("Hah...no");
    }
}

impl AddressingMode for ZeroPageAM {
    fn load(&self, cpu: &mut Cpu) -> u8 {
        cpu.fetch_byte(self.arg as u16)
    }
    fn store(&self, cpu: &mut Cpu, val: u8) {}
}

impl AddressingMode for AbsoluteAM {
    fn load(&self, cpu: &mut Cpu) -> u8 {
        cpu.fetch_byte(self.arg)
    }
    fn store(&self, cpu: &mut Cpu, val: u8) {}
}

impl AddressingMode for RelativeAM {
    fn load(&self, cpu: &mut Cpu) -> u8 {
        0 // TODO: Should this be handled in the branch function
    }
    fn store(&self, cpu: &mut Cpu, val: u8) {}
}

impl AddressingMode for ZeroPageIdxXAM {
    fn load(&self, cpu: &mut Cpu) -> u8 {
        let addr = self.arg.wrapping_add(cpu.registers.x);
        cpu.fetch_byte(addr as u16)
    }
    fn store(&self, cpu: &mut Cpu, val: u8) {}
}

impl AddressingMode for ZeroPageIdxYAM {
    fn load(&self, cpu: &mut Cpu) -> u8 {
        let addr = self.arg.wrapping_add(cpu.registers.y);
        cpu.fetch_byte(addr as u16)
    }
    fn store(&self, cpu: &mut Cpu, val: u8) {}
}

impl AddressingMode for AbsoluteIdxXAM {
    fn load(&self, cpu: &mut Cpu) -> u8 {
        let addr = self.arg.wrapping_add(cpu.registers.x as u16);
        cpu.fetch_byte(addr)
    }
    fn store(&self, cpu: &mut Cpu, val: u8) {}
}

impl AddressingMode for AbsoluteIdxYAM {
    fn load(&self, cpu: &mut Cpu) -> u8 {
        let addr = self.arg.wrapping_add(cpu.registers.y as u16);
        cpu.fetch_byte(addr)
    }
    fn store(&self, cpu: &mut Cpu, val: u8) {}
}

impl AddressingMode for IndexedIndirectAM {
    fn load(&self, cpu: &mut Cpu) -> u8 {
        let index = cpu.load_next_byte_bump_pc() + cpu.registers.x;
        let addr = cpu.fetch_word(index as u16);
        cpu.fetch_byte(addr)
    }
    fn store(&self, cpu: &mut Cpu, val: u8) {}
}

impl AddressingMode for IndirectIndexedAM {
    fn load(&self, cpu: &mut Cpu) -> u8 {
        let index = cpu.load_next_byte_bump_pc();
        let addr = cpu.fetch_word(index as u16) + cpu.registers.y as u16;
        cpu.fetch_byte(addr)
    }
    fn store(&self, cpu: &mut Cpu, val: u8) {}
}

impl<'a> Cpu<'a> {
    pub fn new(cart: &'a Cartridge) -> Self {
        Cpu {
            registers: Registers::default(),
            cycles: 0,
            ram: [0u8; 2 * KILOBYTE],
            cart: cart,
        }
    }

    pub fn reset(&mut self) {
        self.registers.pc = 0xC000; // FIXME: This is only for running NESTEST
        self.registers.s = 0xFD;
        self.registers.p = ProcessorFlags::STACK_COPY | ProcessorFlags::INTERRUPT;
    }

    pub fn step(&mut self) {
        let opcode = self.load_next_byte_bump_pc();
        match opcode {
            _ => panic!("Unrecognized opcode: {:#X}", opcode),
        }
    }

    fn load_next_byte_bump_pc(&mut self) -> u8 {
        self.registers.pc += 1;
        self.cart.prg_read(self.registers.pc - 1)
    }

    fn fetch_byte(&mut self, addr: u16) -> u8 {
        match addr {
            0x0...0x07FF => self.ram[addr as usize],
            0x0800...0x1FFF => self.ram[(addr % 0x0800) as usize],
            0x2000...0x2007 => unimplemented!(), // PPU registers
            0x2008...0x3FFF => unimplemented!(), // PPU register mirrors
            0x4000...0x4017 => unimplemented!(), // APU registers
            0x4018...0x401F => panic!("These registers are disabled during normal operation"),
            0x4020...0xFFFF => self.cart.prg_read(addr),
            _ => unreachable!(),
        }
    }

    pub fn fetch_word(&mut self, addr: u16) -> u16 {
        // The 6502 is little endian (LSB first)
        ((self.fetch_byte(addr + 1) as u16) << 8) | self.fetch_byte(addr) as u16
    }
}

impl<'a> fmt::Debug for Cpu<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#X}", self.registers.pc)
    }
}
