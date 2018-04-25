use std::fmt;

use super::interconnect::Interconnect;
use super::nes::KILOBYTE;

const RESET_VECTOR: u16 = 0xFFFC;

pub struct Cpu {
    registers: Registers,
    cycles: usize,
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

bitflags! {
    #[derive(Default)]
    struct ProcessorFlags: u8 {
        const CARRY      = 0b00000001;
        const ZERO       = 0b00000010;
        const INTERRUPT  = 0b00000100;
        const DECIMAL    = 0b00001000;
        const STACK_COPY = 0b00010000;
        const ALWAYS_SET = 0b00100000;
        const OVERFLOW   = 0b01000000;
        const NEGATIVE   = 0b10000000;
    }
}

trait AddressingMode {
    fn init(_cpu: &mut Cpu, _interconnect: &mut Interconnect) -> Self;
    fn load(&self, cpu: &mut Cpu, interconnect: &mut Interconnect) -> u8;
    fn store(&self, cpu: &mut Cpu, interconnect: &mut Interconnect, val: u8);
}

struct AccumulatorAM;
struct ImmediateAM { val: u8 }
struct ZeroPageAM { arg: u8, }
struct AbsoluteAM { arg: u16, }
struct RelativeAM { arg: u8, }
struct ZeroPageIdxXAM { arg: u8, }
struct ZeroPageIdxYAM { arg: u8, }
struct AbsoluteIdxXAM { arg: u16, }
struct AbsoluteIdxYAM { arg: u16, }
struct IndexedIndirectAM { addr: u16, }
struct IndirectIndexedAM { addr: u16, }

impl AddressingMode for AccumulatorAM {
    fn init(_cpu: &mut Cpu, _interconnect: &mut Interconnect) -> Self {
        AccumulatorAM
    }

    fn load(&self, cpu: &mut Cpu, _interconnect: &mut Interconnect) -> u8 {
        cpu.registers.a
    }

    fn store(&self, cpu: &mut Cpu, _interconnect: &mut Interconnect, val: u8) {
        cpu.registers.a = val;
    }
}

impl AddressingMode for ImmediateAM {
    fn init(cpu: &mut Cpu, interconnect: &mut Interconnect) -> Self {
        let val = cpu.load_next_byte_bump_pc(interconnect);
        ImmediateAM { val: val }
    }

    fn load(&self, _cpu: &mut Cpu, _interconnect: &mut Interconnect) -> u8 {
        self.val
    }

    fn store(&self, _cpu: &mut Cpu, _interconnect: &mut Interconnect, _val: u8) {
        panic!("Hah...no");
    }
}

impl RelativeAM {
    fn init(cpu: &mut Cpu, interconnect: &mut Interconnect) -> Self {
        RelativeAM{ arg: cpu.load_next_byte_bump_pc(interconnect) }
    }
}

impl AddressingMode for ZeroPageAM {
    fn init(cpu: &mut Cpu, interconnect: &mut Interconnect) -> Self {
        ZeroPageAM { arg: cpu.load_next_byte_bump_pc(interconnect) }
    }

    fn load(&self, cpu: &mut Cpu, interconnect: &mut Interconnect) -> u8 {
        cpu.fetch_byte(interconnect, self.arg as u16)
    }

    fn store(&self, cpu: &mut Cpu, interconnect: &mut Interconnect, val: u8) {
        cpu.store(interconnect, self.arg as u16, val);
    }
}

impl AddressingMode for AbsoluteAM {
    fn init(cpu: &mut Cpu, interconnect: &mut Interconnect) -> Self {
        AbsoluteAM { arg: cpu.load_next_word_bump_pc(interconnect) }
    }

    fn load(&self, cpu: &mut Cpu, interconnect: &mut Interconnect) -> u8 {
        cpu.fetch_byte(interconnect, self.arg)
    }

    fn store(&self, cpu: &mut Cpu, interconnect: &mut Interconnect, val: u8) {
        cpu.store(interconnect, self.arg, val);
    }
}

impl AddressingMode for ZeroPageIdxXAM {
    fn init(cpu: &mut Cpu, interconnect: &mut Interconnect) -> Self {
        ZeroPageIdxXAM{ arg: cpu.load_next_byte_bump_pc(interconnect) }
    }

    fn load(&self, cpu: &mut Cpu, interconnect: &mut Interconnect) -> u8 {
        let x = cpu.registers.x;
        cpu.fetch_byte(interconnect, self.arg.wrapping_add(x) as u16)
    }

    fn store(&self, cpu: &mut Cpu, interconnect: &mut Interconnect, val: u8) {
        let x = cpu.registers.x;
        cpu.store(interconnect, self.arg.wrapping_add(x) as u16, val);
    }
}

impl AddressingMode for ZeroPageIdxYAM {
    fn init(cpu: &mut Cpu, interconnect: &mut Interconnect) -> Self {
        ZeroPageIdxYAM { arg: cpu.load_next_byte_bump_pc(interconnect) }
    }

    fn load(&self, cpu: &mut Cpu, interconnect: &mut Interconnect) -> u8 {
        let y = cpu.registers.y;
        cpu.fetch_byte(interconnect, self.arg.wrapping_add(y) as u16)
    }

    fn store(&self, cpu: &mut Cpu, interconnect: &mut Interconnect, val: u8) {
        let y = cpu.registers.y;
        cpu.store(interconnect, self.arg.wrapping_add(y) as u16, val);
    }
}

impl AddressingMode for AbsoluteIdxXAM {
    fn init(cpu: &mut Cpu, interconnect: &mut Interconnect) -> Self {
        AbsoluteIdxXAM { arg: cpu.load_next_word_bump_pc(interconnect) }
    }

    fn load(&self, cpu: &mut Cpu, interconnect: &mut Interconnect) -> u8 {
        let x = cpu.registers.x as u16;
        cpu.fetch_byte(interconnect, self.arg.wrapping_add(x))
    }

    fn store(&self, cpu: &mut Cpu, interconnect: &mut Interconnect, val: u8) {
        let x = cpu.registers.x as u16;
        cpu.store(interconnect, self.arg.wrapping_add(x), val);
    }
}

impl AddressingMode for AbsoluteIdxYAM {
    fn init(cpu: &mut Cpu, interconnect: &mut Interconnect) -> Self {
        AbsoluteIdxYAM{ arg: cpu.load_next_word_bump_pc(interconnect) }
    }

    fn load(&self, cpu: &mut Cpu, interconnect: &mut Interconnect) -> u8 {
        let y = cpu.registers.y as u16;
        cpu.fetch_byte(interconnect, self.arg.wrapping_add(y))
    }

    fn store(&self, cpu: &mut Cpu, interconnect: &mut Interconnect, val: u8) {
        let y = cpu.registers.y as u16;
        cpu.store(interconnect, self.arg.wrapping_add(y), val);
    }
}

impl AddressingMode for IndexedIndirectAM {
    fn init(cpu: &mut Cpu, interconnect: &mut Interconnect) -> Self {
        let index = cpu.load_next_byte_bump_pc(interconnect).wrapping_add(cpu.registers.x);
        let addr = cpu.zero_page_addr(interconnect, index);
        IndexedIndirectAM{ addr: addr }
    }

    fn load(&self, cpu: &mut Cpu, interconnect: &mut Interconnect) -> u8 {
        cpu.fetch_byte(interconnect, self.addr)
    }

    fn store(&self, cpu: &mut Cpu, interconnect: &mut Interconnect, val: u8) {
        cpu.store(interconnect, self.addr, val);
    }
}

impl AddressingMode for IndirectIndexedAM {
    fn init(cpu: &mut Cpu, interconnect: &mut Interconnect) -> Self {
        let index = cpu.load_next_byte_bump_pc(interconnect);
        let addr = cpu.zero_page_addr(interconnect, index).wrapping_add(cpu.registers.y as u16);
        IndirectIndexedAM{ addr: addr }
    }

    fn load(&self, cpu: &mut Cpu, interconnect: &mut Interconnect) -> u8 {
        cpu.fetch_byte(interconnect, self.addr)
    }

    fn store(&self, cpu: &mut Cpu, interconnect: &mut Interconnect, val: u8) {
        cpu.store(interconnect, self.addr, val);
    }
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            registers: Registers::default(),
            cycles: 0,
            ram: [0u8; 2 * KILOBYTE],
        }
    }

    pub fn reset(&mut self, interconnect: &mut Interconnect) {
        self.registers.pc = 0xC000;
        // self.registers.pc = self.fetch_word(interconnect, RESET_VECTOR);
        self.registers.s = 0xFD;
        self.registers.p = ProcessorFlags::from_bits(0x24).unwrap();
    }

    fn load_next_byte_bump_pc(&mut self, interconnect: &mut Interconnect) -> u8 {
        let pc = self.registers.pc;
        self.registers.pc += 1;
        self.fetch_byte(interconnect, pc)
    }

    fn load_next_word_bump_pc(&mut self, interconnect: &mut Interconnect) -> u16 {
        self.load_next_byte_bump_pc(interconnect) as u16 | (self.load_next_byte_bump_pc(interconnect) as u16) << 8
    }

    fn zero_page_addr(&mut self, interconnect: &mut Interconnect, index: u8) -> u16 {
        if index == 0xFF {
            (self.fetch_byte(interconnect, 0x0000) as u16) << 8 | self.fetch_byte(interconnect, index as u16) as u16
        } else {
            self.fetch_word(interconnect, index as u16)
        }
    }

    pub fn pc(&self) -> u16 {
        self.registers.pc
    }

    pub fn cycles(&self) -> usize {
        self.cycles
    }

    pub fn fetch_byte(&mut self, interconnect: &mut Interconnect, addr: u16) -> u8 {
        match addr {
            0x0...0x07FF => self.ram[addr as usize],
            0x0800...0x1FFF => self.ram[(addr % 0x0800) as usize],
            0x2000...0xFFFF => interconnect.read_byte(addr),
            _ => unreachable!(),
        }
    }

    fn fetch_word(&mut self, interconnect: &mut Interconnect, addr: u16) -> u16 {
        // The 6502 is little endian (LSB first)
        ((self.fetch_byte(interconnect, addr + 1) as u16) << 8) | self.fetch_byte(interconnect, addr) as u16
    }

    fn store(&mut self, interconnect: &mut Interconnect, addr: u16, val: u8) {
        match addr {
            0x0...0x07FF => self.ram[addr as usize] = val,
            0x0800...0x1FFF => self.ram[(addr % 0x0800) as usize] = val,
            0x2000...0xFFFF => interconnect.write_byte(addr, val),
            _ => unreachable!(),
        }
    }

    fn stack_push_byte(&mut self, val: u8) {
        self.ram[(self.registers.s as usize) + 0x100] = val;
        self.registers.s -= 1;
    }

    fn stack_push_word(&mut self, val: u16) {
        let high_byte = ((val >> 8) & 0xFF) as u8;
        let low_byte = (val & 0xFF) as u8;
        self.stack_push_byte(high_byte);
        self.stack_push_byte(low_byte);
    }

    fn stack_pop_byte(&mut self) -> u8 {
        self.registers.s += 1;
        let val = self.ram[(self.registers.s as usize) + 0x100];
        val
    }

    fn stack_pop_word(&mut self) -> u16 {
        let low_byte = self.stack_pop_byte();
        let high_byte = self.stack_pop_byte();
        ((high_byte as u16) << 8) | (low_byte as u16)
    }

    fn add_cycles(&mut self, num_cycles: usize) {
        self.cycles += 3 * num_cycles;
        // TODO: Some kind of rollover?
    }

    pub fn step(&mut self, interconnect: &mut Interconnect) -> u8 {
        let opcode = self.load_next_byte_bump_pc(interconnect);
        match opcode {
            // Branches
            0x10 => { let am = RelativeAM::init(self, interconnect); self.bpl(am); }
            0x30 => { let am = RelativeAM::init(self, interconnect); self.bmi(am); }
            0x50 => { let am = RelativeAM::init(self, interconnect); self.bvc(am); }
            0x70 => { let am = RelativeAM::init(self, interconnect); self.bvs(am); }
            0x90 => { let am = RelativeAM::init(self, interconnect); self.bcc(am); }
            0xB0 => { let am = RelativeAM::init(self, interconnect); self.bcs(am); }
            0xD0 => { let am = RelativeAM::init(self, interconnect); self.bne(am); }
            0xF0 => { let am = RelativeAM::init(self, interconnect); self.beq(am); }

            // ALU operations
            0x61 => { let am = IndexedIndirectAM::init(self, interconnect); self.adc(interconnect, am); }
            0x65 => { let am = ZeroPageAM::init(self, interconnect); self.adc(interconnect, am); }
            0x69 => { let am = ImmediateAM::init(self, interconnect); self.adc(interconnect, am); }
            0x6D => { let am = AbsoluteAM::init(self, interconnect); self.adc(interconnect, am); }
            0x71 => { let am = IndirectIndexedAM::init(self, interconnect); self.adc(interconnect, am); }
            0x75 => { let am = ZeroPageIdxXAM::init(self, interconnect); self.adc(interconnect, am); }
            0x79 => { let am = AbsoluteIdxYAM::init(self, interconnect); self.adc(interconnect, am); }
            0x7D => { let am = AbsoluteIdxXAM::init(self, interconnect); self.adc(interconnect, am); }

            0x21 => { let am = IndexedIndirectAM::init(self, interconnect); self.and(interconnect, am); }
            0x25 => { let am = ZeroPageAM::init(self, interconnect); self.and(interconnect, am); }
            0x29 => { let am = ImmediateAM::init(self, interconnect); self.and(interconnect, am); }
            0x2D => { let am = AbsoluteAM::init(self, interconnect); self.and(interconnect, am); }
            0x31 => { let am = IndirectIndexedAM::init(self, interconnect); self.and(interconnect, am); }
            0x35 => { let am = ZeroPageIdxXAM::init(self, interconnect); self.and(interconnect, am); }
            0x39 => { let am = AbsoluteIdxYAM::init(self, interconnect); self.and(interconnect, am); }
            0x3D => { let am = AbsoluteIdxXAM::init(self, interconnect); self.and(interconnect, am); }

            0x06 => { let am = ZeroPageAM::init(self, interconnect); self.asl(interconnect, am); }
            0x0A => { let am = AccumulatorAM::init(self, interconnect); self.asl(interconnect, am); }
            0x0E => { let am = AbsoluteAM::init(self, interconnect); self.asl(interconnect, am); }
            0x16 => { let am = ZeroPageIdxXAM::init(self, interconnect); self.asl(interconnect, am); }
            0x1E => { let am = AbsoluteIdxXAM::init(self, interconnect); self.asl(interconnect, am); }

            0x24 => { let am = ZeroPageAM::init(self, interconnect); self.bit(interconnect, am); }
            0x2C => { let am = AbsoluteAM::init(self, interconnect); self.bit(interconnect, am); }

            0xC1 => { let am = IndexedIndirectAM::init(self, interconnect); self.cmp(interconnect, am); }
            0xC5 => { let am = ZeroPageAM::init(self, interconnect); self.cmp(interconnect, am); }
            0xC9 => { let am = ImmediateAM::init(self, interconnect); self.cmp(interconnect, am); }
            0xCD => { let am = AbsoluteAM::init(self, interconnect); self.cmp(interconnect, am); }
            0xD1 => { let am = IndirectIndexedAM::init(self, interconnect); self.cmp(interconnect, am); }
            0xD5 => { let am = ZeroPageIdxXAM::init(self, interconnect); self.cmp(interconnect, am); }
            0xD9 => { let am = AbsoluteIdxYAM::init(self, interconnect); self.cmp(interconnect, am); }
            0xDD => { let am = AbsoluteIdxXAM::init(self, interconnect); self.cmp(interconnect, am); }

            0xE0 => { let am = ImmediateAM::init(self, interconnect); self.cpx(interconnect, am); }
            0xE4 => { let am = ZeroPageAM::init(self, interconnect); self.cpx(interconnect, am); }
            0xEC => { let am = AbsoluteAM::init(self, interconnect); self.cpx(interconnect, am); }

            0xC0 => { let am = ImmediateAM::init(self, interconnect); self.cpy(interconnect, am); }
            0xC4 => { let am = ZeroPageAM::init(self, interconnect); self.cpy(interconnect, am); }
            0xCC => { let am = AbsoluteAM::init(self, interconnect); self.cpy(interconnect, am); }

            0x41 => { let am = IndexedIndirectAM::init(self, interconnect); self.eor(interconnect, am); }
            0x45 => { let am = ZeroPageAM::init(self, interconnect); self.eor(interconnect, am); }
            0x49 => { let am = ImmediateAM::init(self, interconnect); self.eor(interconnect, am); }
            0x4D => { let am = AbsoluteAM::init(self, interconnect); self.eor(interconnect, am); }
            0x51 => { let am = IndirectIndexedAM::init(self, interconnect); self.eor(interconnect, am); }
            0x55 => { let am = ZeroPageIdxXAM::init(self, interconnect); self.eor(interconnect, am); }
            0x59 => { let am = AbsoluteIdxYAM::init(self, interconnect); self.eor(interconnect, am); }
            0x5D => { let am = AbsoluteIdxXAM::init(self, interconnect); self.eor(interconnect, am); }

            0x4A => { let am = AccumulatorAM::init(self, interconnect); self.lsr(interconnect, am); }
            0x46 => { let am = ZeroPageAM::init(self, interconnect); self.lsr(interconnect, am); }
            0x4E => { let am = AbsoluteAM::init(self, interconnect); self.lsr(interconnect, am); }
            0x56 => { let am = ZeroPageIdxXAM::init(self, interconnect); self.lsr(interconnect, am); }
            0x5E => { let am = AbsoluteIdxXAM::init(self, interconnect); self.lsr(interconnect, am); }

            0x01 => { let am = IndexedIndirectAM::init(self, interconnect); self.ora(interconnect, am); }
            0x05 => { let am = ZeroPageAM::init(self, interconnect); self.ora(interconnect, am); }
            0x09 => { let am = ImmediateAM::init(self, interconnect); self.ora(interconnect, am); }
            0x0D => { let am = AbsoluteAM::init(self, interconnect); self.ora(interconnect, am); }
            0x11 => { let am = IndirectIndexedAM::init(self, interconnect); self.ora(interconnect, am); }
            0x15 => { let am = ZeroPageIdxXAM::init(self, interconnect); self.ora(interconnect, am); }
            0x19 => { let am = AbsoluteIdxYAM::init(self, interconnect); self.ora(interconnect, am); }
            0x1D => { let am = AbsoluteIdxXAM::init(self, interconnect); self.ora(interconnect, am); }

            0x2A => { let am = AccumulatorAM::init(self, interconnect);  self.rol(interconnect, am); }
            0x26 => { let am = ZeroPageAM::init(self, interconnect); self.rol(interconnect, am); }
            0x2E => { let am = AbsoluteAM::init(self, interconnect); self.rol(interconnect, am); }
            0x36 => { let am = ZeroPageIdxXAM::init(self, interconnect); self.rol(interconnect, am); }
            0x3E => { let am = AbsoluteIdxXAM::init(self, interconnect); self.rol(interconnect, am); }

            0x6A => { let am = AccumulatorAM::init(self, interconnect); self.ror(interconnect, am); }
            0x66 => { let am = ZeroPageAM::init(self, interconnect); self.ror(interconnect, am); }
            0x6E => { let am = AbsoluteAM::init(self, interconnect); self.ror(interconnect, am); }
            0x76 => { let am = ZeroPageIdxXAM::init(self, interconnect); self.ror(interconnect, am); }
            0x7E => { let am = AbsoluteIdxXAM::init(self, interconnect); self.ror(interconnect, am); }

            0xE1 => { let am = IndexedIndirectAM::init(self, interconnect); self.sbc(interconnect, am); }
            0xE5 => { let am = ZeroPageAM::init(self, interconnect); self.sbc(interconnect, am); }
            0xE9 => { let am = ImmediateAM::init(self, interconnect); self.sbc(interconnect, am); }
            0xED => { let am = AbsoluteAM::init(self, interconnect); self.sbc(interconnect, am); }
            0xF1 => { let am = IndirectIndexedAM::init(self, interconnect); self.sbc(interconnect, am); }
            0xF5 => { let am = ZeroPageIdxXAM::init(self, interconnect); self.sbc(interconnect, am); }
            0xF9 => { let am = AbsoluteIdxYAM::init(self, interconnect); self.sbc(interconnect, am); }
            0xFD => { let am = AbsoluteIdxXAM::init(self, interconnect); self.sbc(interconnect, am); }

            // Increments and Decrements
            0xE6 => { let am = ZeroPageAM::init(self, interconnect); self.inc(interconnect, am); }
            0xEE => { let am = AbsoluteAM::init(self, interconnect); self.inc(interconnect, am); }
            0xF6 => { let am = ZeroPageIdxXAM::init(self, interconnect); self.inc(interconnect, am); }
            0xFE => { let am = AbsoluteIdxXAM::init(self, interconnect); self.inc(interconnect, am); }
            0xE8 => { self.inx(); }
            0xC8 => { self.iny(); }

            0xC6 => { let am = ZeroPageAM::init(self, interconnect); self.dec(interconnect, am); }
            0xCE => { let am = AbsoluteAM::init(self, interconnect); self.dec(interconnect, am); }
            0xD6 => { let am = ZeroPageIdxXAM::init(self, interconnect); self.dec(interconnect, am); }
            0xDE => { let am = AbsoluteIdxXAM::init(self, interconnect); self.dec(interconnect, am); }
            0xCA => { self.dex(); }
            0x88 => { self.dey(); }

            // Loads
            0xA1 => { let am = IndexedIndirectAM::init(self, interconnect); self.lda(interconnect, am); }
            0xA5 => { let am = ZeroPageAM::init(self, interconnect); self.lda(interconnect, am); }
            0xA9 => { let am = ImmediateAM::init(self, interconnect); self.lda(interconnect, am); }
            0xAD => { let am = AbsoluteAM::init(self, interconnect); self.lda(interconnect, am); }
            0xB1 => { let am = IndirectIndexedAM::init(self, interconnect); self.lda(interconnect, am); }
            0xB5 => { let am = ZeroPageIdxXAM::init(self, interconnect); self.lda(interconnect, am); }
            0xB9 => { let am = AbsoluteIdxYAM::init(self, interconnect); self.lda(interconnect, am); }
            0xBD => { let am = AbsoluteIdxXAM::init(self, interconnect); self.lda(interconnect, am); }

            0xA2 => { let am = ImmediateAM::init(self, interconnect); self.ldx(interconnect, am); }
            0xA6 => { let am = ZeroPageAM::init(self, interconnect); self.ldx(interconnect, am); }
            0xAE => { let am = AbsoluteAM::init(self, interconnect); self.ldx(interconnect, am); }
            0xB6 => { let am = ZeroPageIdxYAM::init(self, interconnect); self.ldx(interconnect, am); }
            0xBE => { let am = AbsoluteIdxYAM::init(self, interconnect); self.ldx(interconnect, am); }

            0xA0 => { let am = ImmediateAM::init(self, interconnect); self.ldy(interconnect, am); }
            0xA4 => { let am = ZeroPageAM::init(self, interconnect); self.ldy(interconnect, am); }
            0xAC => { let am = AbsoluteAM::init(self, interconnect); self.ldy(interconnect, am); }
            0xB4 => { let am = ZeroPageIdxXAM::init(self, interconnect); self.ldy(interconnect, am); }
            0xBC => { let am = AbsoluteIdxXAM::init(self, interconnect); self.ldy(interconnect, am); }

            // Stores
            0x81 => { let am = IndexedIndirectAM::init(self, interconnect); self.sta(interconnect, am); }
            0x85 => { let am = ZeroPageAM::init(self, interconnect); self.sta(interconnect, am); }
            0x8D => { let am = AbsoluteAM::init(self, interconnect); self.sta(interconnect, am); }
            0x91 => { let am = IndirectIndexedAM::init(self, interconnect); self.sta(interconnect, am); }
            0x95 => { let am = ZeroPageIdxXAM::init(self, interconnect); self.sta(interconnect, am); }
            0x99 => { let am = AbsoluteIdxYAM::init(self, interconnect); self.sta(interconnect, am); }
            0x9D => { let am = AbsoluteIdxXAM::init(self, interconnect); self.sta(interconnect, am); }

            0x86 => { let am = ZeroPageAM::init(self, interconnect); self.stx(interconnect, am); }
            0x8E => { let am = AbsoluteAM::init(self, interconnect); self.stx(interconnect, am); }
            0x96 => { let am = ZeroPageIdxYAM::init(self, interconnect); self.stx(interconnect, am); }

            0x84 => { let am = ZeroPageAM::init(self, interconnect); self.sty(interconnect, am); }
            0x8C => { let am = AbsoluteAM::init(self, interconnect); self.sty(interconnect, am); }
            0x94 => { let am = ZeroPageIdxXAM::init(self, interconnect); self.sty(interconnect, am); }

            // Flag sets
            0x38 => { self.sec(); }
            0x78 => { self.sei(); }
            0xF8 => { self.sed(); }

            // Flag clears
            0x18 => { self.clc(); }
            0xB8 => { self.clv(); }
            0xD8 => { self.cld(); }

            // Stack
            0x08 => { self.php(); }
            0x28 => { self.plp(); }
            0x48 => { self.pha(); }
            0x68 => { self.pla(); }

            // Transfers
            0xAA => { self.tax(); }
            0xA8 => { self.tay(); }
            0xBA => { self.tsx(); }
            0x8A => { self.txa(); }
            0x9A => { self.txs(); }
            0x98 => { self.tya(); }

            // Jumps
            0x4C => self.jmp_absolute(interconnect),
            0x6C => self.jmp_indirect(interconnect),
            0x20 => self.jsr(interconnect),
            0x40 => self.rti(),
            0x60 => self.rts(),

            0xEA => self.nop(),

            _ => panic!("Unrecognized opcode: {:#X}", opcode),
        };
        opcode
    }

    // INSTRUCTIONS
    // Branches
    fn bcs(&mut self, am: RelativeAM) { self.branch(am, ProcessorFlags::CARRY, true); }
    fn bcc(&mut self, am: RelativeAM) { self.branch(am, ProcessorFlags::CARRY, false); }
    fn beq(&mut self, am: RelativeAM) { self.branch(am, ProcessorFlags::ZERO, true); }
    fn bne(&mut self, am: RelativeAM) { self.branch(am, ProcessorFlags::ZERO, false); }
    fn bvs(&mut self, am: RelativeAM) { self.branch(am, ProcessorFlags::OVERFLOW, true); }
    fn bvc(&mut self, am: RelativeAM) { self.branch(am, ProcessorFlags::OVERFLOW, false); }
    fn bpl(&mut self, am: RelativeAM) { self.branch(am, ProcessorFlags::NEGATIVE, false); }
    fn bmi(&mut self, am: RelativeAM) { self.branch(am, ProcessorFlags::NEGATIVE, true); }

    fn branch(&mut self, am: RelativeAM, flag: ProcessorFlags, set: bool) {
        let offset_addr = self.registers.pc.wrapping_add((am.arg as i8) as u16);
        if set == self.registers.p.contains(flag) {
            self.registers.pc = offset_addr;
        }
    }


    // Flag sets
    fn sec(&mut self) {
        self.registers.p.set(ProcessorFlags::CARRY, true);
    }

    fn sei(&mut self) {
        self.registers.p.set(ProcessorFlags::INTERRUPT, true);
    }

    fn sed(&mut self) {
        self.registers.p.set(ProcessorFlags::DECIMAL, true);
    }

    // Flag clears
    fn clc(&mut self) {
        self.registers.p.set(ProcessorFlags::CARRY, false);
    }

    fn cld(&mut self) {
        self.registers.p.set(ProcessorFlags::DECIMAL, false);
    }

    fn clv(&mut self) {
        self.registers.p.set(ProcessorFlags::OVERFLOW, false);
    }

    // Stack
    fn php(&mut self) {
        let p = self.registers.p.bits() | ProcessorFlags::STACK_COPY.bits();
        self.stack_push_byte(p);
    }

    fn pla(&mut self) {
        self.registers.a = self.stack_pop_byte();
        self.registers.p.set(ProcessorFlags::ZERO, self.registers.a == 0);
        self.registers.p.set(ProcessorFlags::NEGATIVE, self.registers.a & (1 << 7) != 0);
    }

    fn pha(&mut self) {
        let a = self.registers.a;
        self.stack_push_byte(a);
    }

    fn plp(&mut self) {
        let p = self.stack_pop_byte();
        self.registers.p = ProcessorFlags::from_bits(p & 0b1110_1111).unwrap(); // PLP ignores bit 5
        self.registers.p.set(ProcessorFlags::ALWAYS_SET, true);
    }

    // ALU Ops
    fn adc<AM: AddressingMode>(&mut self, interconnect: &mut Interconnect, am: AM) {
        let a = self.registers.a;
        let m = am.load(self, interconnect);
        let c = if self.registers.p.contains(ProcessorFlags::CARRY) { 1 } else { 0 };
        let result = a.wrapping_add(m).wrapping_add(c);
        self.registers.a = result;
        self.registers.p.set(ProcessorFlags::ZERO, result == 0);
        self.registers.p.set(ProcessorFlags::OVERFLOW, (!(a ^ m) & (a ^ result) & 0x80) != 0);
        self.registers.p.set(ProcessorFlags::CARRY, (a as u16 + m as u16 + c as u16) > (result as u16));
        self.registers.p.set(ProcessorFlags::NEGATIVE, result & (1 << 7) != 0);
    }

    fn asl<AM: AddressingMode>(&mut self, interconnect: &mut Interconnect, am: AM) {
        let mut arg = am.load(self, interconnect);
        self.registers.p.set(ProcessorFlags::CARRY, (arg & (1 << 7)) != 0); // set based on original value
        arg = arg << 1;
        am.store(self, interconnect, arg);
        self.registers.p.set(ProcessorFlags::ZERO, arg == 0);
        self.registers.p.set(ProcessorFlags::NEGATIVE, arg & (1 << 7) != 0);
    }

    fn and<AM: AddressingMode>(&mut self, interconnect: &mut Interconnect, am: AM) {
        let a = self.registers.a & am.load(self, interconnect);
        self.registers.a = a;
        self.registers.p.set(ProcessorFlags::ZERO, a == 0);
        self.registers.p.set(ProcessorFlags::NEGATIVE, a & (1 << 7) != 0);
    }

    fn bit<AM: AddressingMode>(&mut self, interconnect: &mut Interconnect, am: AM) {
        let a = self.registers.a;
        let m = am.load(self, interconnect);
        self.registers.p.set(ProcessorFlags::ZERO, a & m == 0);
        self.registers.p.set(ProcessorFlags::OVERFLOW, m & (1 << 6) != 0);
        self.registers.p.set(ProcessorFlags::NEGATIVE, m & (1 << 7) != 0);
    }

    fn compare(&mut self, lhs: u8, rhs: u8) {
        self.registers.p.set(ProcessorFlags::CARRY, lhs >= rhs);
        self.registers.p.set(ProcessorFlags::ZERO, lhs == rhs);
        self.registers.p.set(ProcessorFlags::NEGATIVE, lhs.wrapping_sub(rhs) & (1 << 7) != 0);
    }

    fn cmp<AM: AddressingMode>(&mut self, interconnect: &mut Interconnect, am: AM) {
        let a = self.registers.a;
        let m = am.load(self, interconnect);
        self.compare(a, m);
    }

    fn cpx<AM: AddressingMode>(&mut self, interconnect: &mut Interconnect, am: AM) {
        let x = self.registers.x;
        let m = am.load(self, interconnect);
        self.compare(x, m);
    }

    fn cpy<AM: AddressingMode>(&mut self, interconnect: &mut Interconnect, am: AM) {
        let y = self.registers.y;
        let m = am.load(self, interconnect);
        self.compare(y, m);
    }

    fn eor<AM: AddressingMode>(&mut self, interconnect: &mut Interconnect, am: AM) {
        let a = self.registers.a;
        let m = am.load(self, interconnect);
        self.registers.a = a ^ m;
        self.registers.p.set(ProcessorFlags::ZERO, self.registers.a == 0);
        self.registers.p.set(ProcessorFlags::NEGATIVE, self.registers.a & (1 << 7) != 0);
    }

    fn lsr<AM: AddressingMode>(&mut self, interconnect: &mut Interconnect, am: AM) {
        let mut arg = am.load(self, interconnect);
        self.registers.p.set(ProcessorFlags::CARRY, (arg & (1 << 0)) != 0); // set based on original value
        arg = arg >> 1;
        am.store(self, interconnect, arg);
        self.registers.p.set(ProcessorFlags::ZERO, arg == 0);
        self.registers.p.set(ProcessorFlags::NEGATIVE, arg & (1 << 7) != 0);
    }

    fn ora<AM: AddressingMode>(&mut self, interconnect: &mut Interconnect, am: AM) {
        let a = self.registers.a;
        let m = am.load(self, interconnect);
        self.registers.a = a | m;
        self.registers.p.set(ProcessorFlags::ZERO, self.registers.a == 0);
        self.registers.p.set(ProcessorFlags::NEGATIVE, self.registers.a & (1 << 7) != 0);
    }

    fn rol<AM: AddressingMode>(&mut self, interconnect: &mut Interconnect, am: AM) {
        let mut arg = am.load(self, interconnect);
        let old_carry = if self.registers.p.contains(ProcessorFlags::CARRY) { 1 } else { 0 };
        self.registers.p.set(ProcessorFlags::CARRY, arg & (1 << 7) != 0);
        arg = (arg << 1) | old_carry;
        am.store(self, interconnect, arg);
        self.registers.p.set(ProcessorFlags::ZERO, arg == 0);
        self.registers.p.set(ProcessorFlags::NEGATIVE, arg & (1 << 7) != 0);
    }

    fn ror<AM: AddressingMode>(&mut self, interconnect: &mut Interconnect, am: AM) {
        let mut arg = am.load(self, interconnect);
        let old_carry = if self.registers.p.contains(ProcessorFlags::CARRY) { 1 << 7 } else { 0 };
        self.registers.p.set(ProcessorFlags::CARRY, arg & (1 << 0) != 0);
        arg = (arg >> 1) | old_carry;
        am.store(self, interconnect, arg);
        self.registers.p.set(ProcessorFlags::ZERO, arg == 0);
        self.registers.p.set(ProcessorFlags::NEGATIVE, arg & (1 << 7) != 0);
    }

    fn sbc<AM: AddressingMode>(&mut self, interconnect: &mut Interconnect, am: AM) {
        let a = self.registers.a;
        let m = am.load(self, interconnect);
        let c = if self.registers.p.contains(ProcessorFlags::CARRY) { 1 } else { 0 };
        let result = a.wrapping_add(m).wrapping_add(c);
        self.registers.a = result;
        self.registers.p.set(ProcessorFlags::ZERO, result == 0);
        self.registers.p.set(ProcessorFlags::OVERFLOW, (!(a ^ m) & (a ^ result) & 0x80) != 0);
        self.registers.p.set(ProcessorFlags::CARRY, (a as u16 + m as u16 + c as u16) > (result as u16));
        self.registers.p.set(ProcessorFlags::NEGATIVE, result & (1 << 7) != 0);
    }

    // Increments and decrements
    fn inc<AM: AddressingMode>(&mut self, interconnect: &mut Interconnect, am: AM) {
        let m = am.load(self, interconnect);
        let val = m.wrapping_add(1);
        self.registers.p.set(ProcessorFlags::ZERO, val == 0);
        self.registers.p.set(ProcessorFlags::NEGATIVE, val & (1 << 7) != 0);
        am.store(self, interconnect, val);
    }

    fn inx(&mut self) {
        let x = self.registers.x.wrapping_add(1);
        self.registers.x = x;
        self.registers.p.set(ProcessorFlags::ZERO, self.registers.x == 0);
        self.registers.p.set(ProcessorFlags::NEGATIVE, self.registers.x & (1 << 7) != 0);
    }

    fn iny(&mut self) {
        let y = self.registers.y.wrapping_add(1);
        self.registers.y = y;
        self.registers.p.set(ProcessorFlags::ZERO, self.registers.y == 0);
        self.registers.p.set(ProcessorFlags::NEGATIVE, self.registers.y & (1 << 7) != 0);
    }

    fn dec<AM: AddressingMode>(&mut self, interconnect: &mut Interconnect, am: AM) {
        let m = am.load(self, interconnect);
        let val = m.wrapping_sub(1);
        self.registers.p.set(ProcessorFlags::ZERO, val == 0);
        self.registers.p.set(ProcessorFlags::NEGATIVE, val & (1 << 7) != 0);
        am.store(self, interconnect, val);
    }

    fn dex(&mut self) {
        self.registers.x = self.registers.x.wrapping_sub(1);
        self.registers.p.set(ProcessorFlags::ZERO, self.registers.x == 0);
        self.registers.p.set(ProcessorFlags::NEGATIVE, self.registers.x & (1 << 7) != 0);
    }

    fn dey(&mut self) {
        self.registers.y = self.registers.y.wrapping_sub(1);
        self.registers.p.set(ProcessorFlags::ZERO, self.registers.y == 0);
        self.registers.p.set(ProcessorFlags::NEGATIVE, self.registers.y & (1 << 7) != 0);
    }

    // Transfers
    fn tax(&mut self) {
        self.registers.x = self.registers.a;
        self.registers.p.set(ProcessorFlags::ZERO, self.registers.x == 0);
        self.registers.p.set(ProcessorFlags::NEGATIVE, self.registers.x & (1 << 7) != 0);
    }

    fn tay(&mut self) {
        self.registers.y = self.registers.a;
        self.registers.p.set(ProcessorFlags::ZERO, self.registers.y == 0);
        self.registers.p.set(ProcessorFlags::NEGATIVE, self.registers.y & (1 << 7) != 0);
    }

    fn tsx(&mut self) {
        self.registers.x = self.registers.s;
        self.registers.p.set(ProcessorFlags::ZERO, self.registers.x == 0);
        self.registers.p.set(ProcessorFlags::NEGATIVE, self.registers.x & (1 << 7) != 0);
    }

    fn txa(&mut self) {
        self.registers.a = self.registers.x;
        self.registers.p.set(ProcessorFlags::ZERO, self.registers.a == 0);
        self.registers.p.set(ProcessorFlags::NEGATIVE, self.registers.a & (1 << 7) != 0);
    }

    fn txs(&mut self) {
        self.registers.s = self.registers.x;
    }

    fn tya(&mut self) {
        self.registers.a = self.registers.y;
        self.registers.p.set(ProcessorFlags::ZERO, self.registers.a == 0);
        self.registers.p.set(ProcessorFlags::NEGATIVE, self.registers.a & (1 << 7) != 0);
    }

    // Loads
    fn lda<AM: AddressingMode>(&mut self, interconnect: &mut Interconnect, am: AM) {
        self.registers.a = am.load(self, interconnect);
        self.registers.p.set(ProcessorFlags::ZERO, self.registers.a == 0);
        self.registers.p.set(ProcessorFlags::NEGATIVE, self.registers.a & (1 << 7) != 0 );
    }

    fn ldx<AM: AddressingMode>(&mut self, interconnect: &mut Interconnect, am: AM) {
        self.registers.x = am.load(self, interconnect);
        self.registers.p.set(ProcessorFlags::ZERO, self.registers.x == 0);
        self.registers.p.set(ProcessorFlags::NEGATIVE, self.registers.x & (1 << 7) != 0);
    }

    fn ldy<AM: AddressingMode>(&mut self, interconnect: &mut Interconnect, am: AM) {
        self.registers.y = am.load(self, interconnect);
        self.registers.p.set(ProcessorFlags::ZERO, self.registers.y == 0);
        self.registers.p.set(ProcessorFlags::NEGATIVE, self.registers.y & (1 << 7) != 0);
    }

    // Stores
    fn sta<AM: AddressingMode>(&mut self, interconnect: &mut Interconnect, am: AM) {
        let a = self.registers.a;
        am.store(self, interconnect, a);
    }

    fn stx<AM: AddressingMode>(&mut self, interconnect: &mut Interconnect, am: AM) {
        let x = self.registers.x;
        am.store(self, interconnect, x);
    }

    fn sty<AM: AddressingMode>(&mut self, interconnect: &mut Interconnect, am: AM) {
        let y = self.registers.y;
        am.store(self, interconnect, y);
    }

    // Jumps are the only instructions that use absolute addressing, so they are given two methods
    fn jmp_absolute(&mut self, interconnect: &mut Interconnect) {
        let addr = self.load_next_word_bump_pc(interconnect);
        self.registers.pc = addr;
    }

    fn jmp_indirect(&mut self, interconnect: &mut Interconnect) {
        let addr = self.load_next_word_bump_pc(interconnect);
        // Handle page boundary
        let final_addr = if addr & 0xFF == 0xFF {
            (self.fetch_byte(interconnect, addr + 1 - 0x100) as u16) << 8 |
            self.fetch_byte(interconnect, addr) as u16
        } else {
            self.fetch_word(interconnect, addr)
        };
        self.registers.pc = final_addr;
    }

    fn jsr(&mut self, interconnect: &mut Interconnect) {
        let target = self.load_next_word_bump_pc(interconnect);
        let addr = self.registers.pc - 1;
        self.stack_push_word(addr);
        self.registers.pc = target;
    }

    fn rts(&mut self) {
        self.registers.pc = self.stack_pop_word() + 1;
    }

    fn rti(&mut self) {
        let p = self.stack_pop_byte();
        let pc = self.stack_pop_word();
        self.registers.p = ProcessorFlags::from_bits(p).unwrap();
        self.registers.p.set(ProcessorFlags::ALWAYS_SET, true);
        self.registers.pc = pc;
    }

    fn nop(&mut self) {}
}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PC: {:04X} {:?} CYC: {:?}",
                    self.pc(),
                    self.registers,
                    self.cycles)
    }
}

impl fmt::Debug for Registers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X}",
               self.a, self.x, self.y, self.p, self.s)
    }
}
