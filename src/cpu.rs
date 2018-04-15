use super::cart::Cartridge;
use super::nes::KILOBYTE;
use std::fmt;

pub struct Cpu {
    registers: Registers,
    cycles: u64,
    ram: [u8; 2 * KILOBYTE],
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

impl fmt::Debug for Registers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X}",
                self.a, self.x, self.y, self.p, self.s)
    }
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
    fn load(&self, cpu: &mut Cpu, cart: &mut Cartridge) -> u8;
    fn store(&self, cpu: &mut Cpu, cart: &mut Cartridge, val: u8);
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
    fn load(&self, cpu: &mut Cpu, cart: &mut Cartridge) -> u8 {
        cpu.registers.a
    }
    fn store(&self, cpu: &mut Cpu, cart: &mut Cartridge, val: u8) {
        cpu.registers.a = val;
    }
}

impl AddressingMode for ImmediateAM {
    fn load(&self, cpu: &mut Cpu, cart: &mut Cartridge) -> u8 {
        let ret = cpu.load_next_byte_bump_pc(cart);
        print!(" #${:02X}{:23}", ret, " ");
        ret
    }
    fn store(&self, cpu: &mut Cpu, cart: &mut Cartridge, val: u8) {
        panic!("Hah...no");
    }
}

impl AddressingMode for ZeroPageAM {
    fn load(&self, cpu: &mut Cpu, cart: &mut Cartridge) -> u8 {
        cpu.fetch_byte(cart, self.arg as u16)
    }
    fn store(&self, cpu: &mut Cpu, cart: &mut Cartridge, val: u8) {
        let addr = self.arg as u16;
        print!("${:02X} = {:02X}{:19}", addr as u8, val, " ");
        cpu.store(cart, addr, val);
    }
}

impl AddressingMode for AbsoluteAM {
    fn load(&self, cpu: &mut Cpu, cart: &mut Cartridge) -> u8 {
        cpu.fetch_byte(cart, self.arg)
    }
    fn store(&self, cpu: &mut Cpu, cart: &mut Cartridge, val: u8) {}
}

impl AddressingMode for RelativeAM {
    fn load(&self, cpu: &mut Cpu, cart: &mut Cartridge) -> u8 {
        0 // TODO: Should this be handled in the branch function
    }
    fn store(&self, cpu: &mut Cpu, cart: &mut Cartridge, val: u8) {}
}

impl AddressingMode for ZeroPageIdxXAM {
    fn load(&self, cpu: &mut Cpu, cart: &mut Cartridge) -> u8 {
        let addr = self.arg.wrapping_add(cpu.registers.x);
        cpu.fetch_byte(cart, addr as u16)
    }
    fn store(&self, cpu: &mut Cpu, cart: &mut Cartridge, val: u8) {}
}

impl AddressingMode for ZeroPageIdxYAM {
    fn load(&self, cpu: &mut Cpu, cart: &mut Cartridge) -> u8 {
        let addr = self.arg.wrapping_add(cpu.registers.y);
        cpu.fetch_byte(cart, addr as u16)
    }
    fn store(&self, cpu: &mut Cpu, cart: &mut Cartridge, val: u8) {}
}

impl AddressingMode for AbsoluteIdxXAM {
    fn load(&self, cpu: &mut Cpu, cart: &mut Cartridge) -> u8 {
        let addr = self.arg.wrapping_add(cpu.registers.x as u16);
        cpu.fetch_byte(cart, addr)
    }
    fn store(&self, cpu: &mut Cpu, cart: &mut Cartridge, val: u8) {}
}

impl AddressingMode for AbsoluteIdxYAM {
    fn load(&self, cpu: &mut Cpu, cart: &mut Cartridge) -> u8 {
        let addr = self.arg.wrapping_add(cpu.registers.y as u16);
        cpu.fetch_byte(cart, addr)
    }
    fn store(&self, cpu: &mut Cpu, cart: &mut Cartridge, val: u8) {}
}

impl AddressingMode for IndexedIndirectAM {
    fn load(&self, cpu: &mut Cpu, cart: &mut Cartridge) -> u8 {
        let index = cpu.load_next_byte_bump_pc(cart) + cpu.registers.x;
        let addr = cpu.fetch_word(cart, index as u16);
        cpu.fetch_byte(cart, addr)
    }
    fn store(&self, cpu: &mut Cpu, cart: &mut Cartridge, val: u8) {}
}

impl AddressingMode for IndirectIndexedAM {
    fn load(&self, cpu: &mut Cpu, cart: &mut Cartridge) -> u8 {
        let index = cpu.load_next_byte_bump_pc(cart);
        let addr = cpu.fetch_word(cart, index as u16) + cpu.registers.y as u16;
        cpu.fetch_byte(cart, addr)
    }
    fn store(&self, cpu: &mut Cpu, cart: &mut Cartridge, val: u8) {}
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            registers: Registers::default(),
            cycles: 0,
            ram: [0u8; 2 * KILOBYTE],
        }
    }

    pub fn reset(&mut self) {
        self.registers.pc = 0xC000; // FIXME: This is only for running NESTEST
        self.registers.s = 0xFD;
        self.registers.p = ProcessorFlags::STACK_COPY | ProcessorFlags::INTERRUPT;
    }

    pub fn step(&mut self, cart: &mut Cartridge) {
        let pc = self.registers.pc;
        let opcode = self.load_next_byte_bump_pc(cart);
        print!("{:X}  {:} ", pc, self.debug_print(cart, opcode));
        match opcode {
            // LDX
            0xA2 => { let am = ImmediateAM; self.ldx(cart, am); },

            // STX
            0x86 => {let am = ZeroPageAM{ arg: self.load_next_byte_bump_pc(cart) }; self.stx(cart, am); }

            0x4C => self.jmp_indirect(cart),
            0x20 => self.jsr(cart),

            0xEA => self.nop(),
            _ => panic!("Unrecognized opcode: {:#X}", opcode),
        };
        println!(" {:?} CYC:{:3}", self.registers, self.cycles);
    }

    fn load_next_byte_bump_pc(&mut self, cart: &mut Cartridge) -> u8 {
        self.registers.pc += 1;
        cart.prg_read(self.registers.pc - 1)
    }

    fn load_next_word_bump_pc(&mut self, cart: &mut Cartridge) -> u16 {
        self.load_next_byte_bump_pc(cart) as u16 |
        (self.load_next_byte_bump_pc(cart) as u16) << 8
    }

    fn fetch_byte(&mut self, cart: &mut Cartridge, addr: u16) -> u8 {
        match addr {
            0x0...0x07FF => self.ram[addr as usize],
            0x0800...0x1FFF => self.ram[(addr % 0x0800) as usize],
            0x2000...0x2007 => unimplemented!(), // PPU registers
            0x2008...0x3FFF => unimplemented!(), // PPU register mirrors
            0x4000...0x4017 => unimplemented!(), // APU registers
            0x4018...0x401F => panic!("These registers are disabled during normal operation"),
            0x4020...0xFFFF => cart.prg_read(addr),
            _ => unreachable!(),
        }
    }

    fn fetch_word(&mut self, cart: &mut Cartridge, addr: u16) -> u16 {
        // The 6502 is little endian (LSB first)
        ((self.fetch_byte(cart, addr + 1) as u16) << 8) | self.fetch_byte(cart, addr) as u16
    }

    fn store(&mut self, cart: &mut Cartridge, addr: u16, val: u8) {
        match addr {
            0x0...0x07FF => self.ram[addr as usize] = val,
            0x0800...0x1FFF => self.ram[(addr % 0x0800) as usize] = val,
            0x2000...0x2007 => unimplemented!(), // PPU registers
            0x2008...0x3FFF => unimplemented!(), // PPU register mirrors
            0x4000...0x4017 => unimplemented!(), // APU registers
            0x4018...0x401F => panic!("These registers are disabled during normal operation"),
            0x4020...0xFFFF => cart.prg_write(addr, val),
            _ => unreachable!(),
        }
    }

    fn stack_push_byte(&mut self, val: u8) {
        self.ram[self.registers.s as usize] = val;
        self.registers.s -= 1;
    }

    fn stack_push_word(&mut self, val: u16) {
        let high_byte = ((val >> 8) & 0xFF) as u8;
        let low_byte = (val & 0xFF) as u8;
        self.stack_push_byte(high_byte);
        self.stack_push_byte(low_byte);
    }

    // INSTRUCTIONS
    fn ldx<AM: AddressingMode>(&mut self, cart: &mut Cartridge, am: AM) {
        print!(" LDX");
        self.registers.x = am.load(self, cart);
        if self.registers.x == 0 {
            self.registers.p |= ProcessorFlags::ZERO;
        }
        if self.registers.x & (1 << 7) != 0 {
            self.registers.p |= ProcessorFlags::NEGATIVE;
        }
    }

    fn stx<AM: AddressingMode>(&mut self, cart: &mut Cartridge, am: AM) {
        print!(" STX ");
        let x = self.registers.x;
        am.store(self, cart, x);
    }

    // Jumps are the only instructions that use absolute addressing, so they are given two methods
    fn jmp_indirect(&mut self, cart: &mut Cartridge) {
        let addr = self.load_next_word_bump_pc(cart);
        print!(" JMP ${:X}{:22}", addr, " ");
        self.registers.pc = addr;
    }

    fn jsr(&mut self, cart: &mut Cartridge) {
        let target = self.load_next_word_bump_pc(cart);
        print!(" JSR ${:X}{:22}", target, " ");
        let addr = self.registers.pc - 1;
        self.stack_push_word(addr);
        self.registers.pc = target;
    }

    fn nop(&mut self) {
        print!(" NOP{:28}", " ")
    }
}

impl Cpu {
    fn debug_print(&mut self, cart: &mut Cartridge, op: u8) -> String {
        let pc = self.registers.pc;
        let debug_string = match op {
            // 2 byte instructions
            0x69 | 0x65 | 0x75 | 0x61 | 0x71 | 0x29 | 0x25 | 0x35 | 0x21 | 0x31 |
            0x06 | 0x16 | 0x90 | 0xB0 | 0xF0 | 0x24 | 0x30 | 0xD0 | 0x10 | 0x50 |
            0x70 | 0xC9 | 0xC5 | 0xD5 | 0xC1 | 0xD1 | 0xE0 | 0xE4 | 0xC0 | 0xC4 |
            0xC6 | 0xD6 | 0x49 | 0x45 | 0x55 | 0x41 | 0x51 | 0xE6 | 0xF6 | 0xA9 |
            0xA5 | 0xB5 | 0xA1 | 0xB1 | 0xA2 | 0xA6 | 0xB6 | 0xA0 | 0x46 | 0x56 |
            0x09 | 0x05 | 0x15 | 0x01 | 0x11 | 0x26 | 0x36 | 0x66 | 0x76 | 0xE9 |
            0xE5 | 0xF5 | 0xE1 | 0xF1 | 0x85 | 0x95 | 0x81 | 0x91 | 0x86 | 0x96 |
            0x84 | 0x94 => format!("{:02X} {:02X}{:3}", op, self.fetch_byte(cart, pc), " "),

            // 3 byte instructions
            0x6D | 0x7D | 0x79 | 0x2D | 0x3D | 0x39 | 0x0E | 0x1E | 0x2C | 0xCD |
            0xDD | 0xD9 | 0xEC | 0xCC | 0xCE | 0xDE | 0x4D | 0x5D | 0x59 | 0xEE |
            0xFE | 0x4C | 0x6C | 0x20 | 0xAD | 0xBD | 0xB9 | 0xAE | 0xBE | 0xAC |
            0xBC | 0x4E | 0x5E | 0x0D | 0x1D | 0x19 | 0x2E | 0x3E | 0x6E | 0x7E |
            0xED | 0xFD | 0xF9 | 0x8D | 0x9D | 0x99 | 0x8E | 0x8C  => format!(
                "{:02X} {:02X} {:02X}", op, self.fetch_byte(cart, pc),
                self.fetch_byte(cart, pc + 1)),

            // 1 byte instructions
            _ => format!("{:02X}{:6}", op, " ")
        };
        debug_string
    }
}
