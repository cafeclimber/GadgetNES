//! Extracts instruction fetch and decode for use by CPU module

use nes::cpu::Cpu;
use nes::memory::Memory;

mod alu;
mod branch;
mod flag;
mod increments;
mod jumps;
mod loads_stores;
mod nop;
mod registers;
mod stack;

/// Various addressing modes used when interfacing with memory
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AddressingMode {
    ZeroPageIndexedX,
    ZeroPageIndexedY,
    AbsoluteIndexedX,
    AbsoluteIndexedY,
    IndexedIndirect,
    IndirectIndexed,
    Implied,
    Accumulator,
    Immediate,
    ZeroPage,
    Absolute,
    Relative,
    Indirect,
}

// TODO: Add unofficial ops
#[derive(Debug, PartialEq, Copy, Clone)]
#[allow(dead_code)]
pub enum Instruction {
    BRK, PHP, PLP, PHA, PLA, TXS, TSX, BPL,
    BMI, BVC, BVS, BCC, BCS, BNE, BEQ, CLC,
    SEC, CLI, SEI, CLV, CLD, SED, DEY, DEX,
    INX, INY, TAX, TXA, TAY, TYA, CPY, CPX,
    LDA, LDX, LDY, STA, STX, STY, JSR, JMP,
    RTI, RTS, BIT, ORA, AND, EOR, ADC, CMP,
    SBC, ASL, LSR, ROL, ROR, DEC, INC, NOP,
}

/// Takes an op_code fetched by the cpu and returns the
/// instruction and addressing mode
pub fn decode(op_code: u8) -> (Instruction, AddressingMode) {
    use self::Instruction;
    match op_code {
        /********OFFICIAL OPCODES*********/
        0x00 => (Instruction::BRK,AddressingMode::Implied),
        
        // Stack
        0x08 => (Instruction::PHP, AddressingMode::Implied),
        0x28 => (Instruction::PLP, AddressingMode::Implied),
        0x48 => (Instruction::PHA, AddressingMode::Implied),
        0x68 => (Instruction::PLA, AddressingMode::Implied),
        0x9A => (Instruction::TXS, AddressingMode::Implied),
        0xBA => (Instruction::TSX, AddressingMode::Implied),

        // Branch
        0x10 => (Instruction::BPL, AddressingMode::Relative),
        0x30 => (Instruction::BMI, AddressingMode::Relative),
        0x50 => (Instruction::BVC, AddressingMode::Relative),
        0x70 => (Instruction::BVS, AddressingMode::Relative),
        0x90 => (Instruction::BCC, AddressingMode::Relative),
        0xB0 => (Instruction::BCS, AddressingMode::Relative),
        0xD0 => (Instruction::BNE, AddressingMode::Relative),
        0xF0 => (Instruction::BEQ, AddressingMode::Relative),

        // Flag instructions
        0x18 => (Instruction::CLC, AddressingMode::Implied),
        0x38 => (Instruction::SEC, AddressingMode::Implied),
        // 0x58 => (Instruction::CLI, AddressingMode::Implied),
        0x78 => (Instruction::SEI, AddressingMode::Implied),
        0xB8 => (Instruction::CLV, AddressingMode::Implied),
        0xD8 => (Instruction::CLD, AddressingMode::Implied),
        0xF8 => (Instruction::SED, AddressingMode::Implied),

        // Register instructions
        0x88 => (Instruction::DEY, AddressingMode::Implied),
        0xCA => (Instruction::DEX, AddressingMode::Implied),
        0xE8 => (Instruction::INX, AddressingMode::Implied),
        0xC8 => (Instruction::INY, AddressingMode::Implied),
        0xAA => (Instruction::TAX, AddressingMode::Implied),
        0x8A => (Instruction::TXA, AddressingMode::Implied),
        0xA8 => (Instruction::TAY, AddressingMode::Implied),
        0x98 => (Instruction::TYA, AddressingMode::Implied),

        // Compares
        0xC0 => (Instruction::CPY, AddressingMode::Immediate),
        0xC4 => (Instruction::CPY, AddressingMode::ZeroPage),
        0xCC => (Instruction::CPY, AddressingMode::Absolute),
        0xE0 => (Instruction::CPX, AddressingMode::Immediate),
        0xE4 => (Instruction::CPX, AddressingMode::ZeroPage),
        0xEC => (Instruction::CPX, AddressingMode::Absolute),

        // Loads
        0xA1 => (Instruction::LDA, AddressingMode::IndexedIndirect),
        0xA5 => (Instruction::LDA, AddressingMode::ZeroPage),
        0xA9 => (Instruction::LDA, AddressingMode::Immediate),
        0xAD => (Instruction::LDA, AddressingMode::Absolute),
        0xB1 => (Instruction::LDA, AddressingMode::IndirectIndexed),
        0xB5 => (Instruction::LDA, AddressingMode::ZeroPageIndexedX),
        0xBD => (Instruction::LDA, AddressingMode::AbsoluteIndexedX),
        0xB9 => (Instruction::LDA, AddressingMode::AbsoluteIndexedY),

        0xA2 => (Instruction::LDX, AddressingMode::Immediate),
        0xA6 => (Instruction::LDX, AddressingMode::ZeroPage),
        0xAE => (Instruction::LDX, AddressingMode::Absolute),
        0xB6 => (Instruction::LDX, AddressingMode::ZeroPageIndexedY),
        0xBE => (Instruction::LDX, AddressingMode::AbsoluteIndexedX),

        0xA0 => (Instruction::LDY, AddressingMode::Immediate),
        0xA4 => (Instruction::LDY, AddressingMode::ZeroPage),
        0xAC => (Instruction::LDY, AddressingMode::Absolute),
        0xB4 => (Instruction::LDY, AddressingMode::ZeroPageIndexedX),
        0xBC => (Instruction::LDY, AddressingMode::AbsoluteIndexedX),

        // Stores
        0x81 => (Instruction::STA, AddressingMode::IndexedIndirect),
        0x85 => (Instruction::STA, AddressingMode::ZeroPage),
        0x8D => (Instruction::STA, AddressingMode::Absolute),
        0x91 => (Instruction::STA, AddressingMode::IndirectIndexed),
        0x95 => (Instruction::STA, AddressingMode::ZeroPageIndexedX),
        0x9D => (Instruction::STA, AddressingMode::AbsoluteIndexedX),
        0x99 => (Instruction::STA, AddressingMode::AbsoluteIndexedY),

        0x86 => (Instruction::STX, AddressingMode::ZeroPage),
        0x8E => (Instruction::STX, AddressingMode::Absolute),
        0x96 => (Instruction::STX, AddressingMode::ZeroPageIndexedY),

        0x84 => (Instruction::STY, AddressingMode::ZeroPage),
        0x8C => (Instruction::STY, AddressingMode::Absolute),
        0x94 => (Instruction::STY, AddressingMode::ZeroPageIndexedX),

        // Jumps
        0x20 => (Instruction::JSR, AddressingMode::Absolute),
        0x4C => (Instruction::JMP, AddressingMode::Absolute),
        0x6C => (Instruction::JMP, AddressingMode::Indirect),

        0x40 => (Instruction::RTI, AddressingMode::Implied),
        0x60 => (Instruction::RTS, AddressingMode::Implied),

        // Bit tests
        0x24 => (Instruction::BIT, AddressingMode::ZeroPage),
        0x2C => (Instruction::BIT, AddressingMode::Absolute),

        // ALU operations
        0x01 => (Instruction::ORA, AddressingMode::IndexedIndirect),
        0x05 => (Instruction::ORA, AddressingMode::ZeroPage),
        0x09 => (Instruction::ORA, AddressingMode::Immediate),
        0x0D => (Instruction::ORA, AddressingMode::Absolute),
        0x11 => (Instruction::ORA, AddressingMode::IndirectIndexed),
        0x15 => (Instruction::ORA, AddressingMode::ZeroPageIndexedX),
        0x1D => (Instruction::ORA, AddressingMode::AbsoluteIndexedX),
        0x19 => (Instruction::ORA, AddressingMode::AbsoluteIndexedY),

        0x21 => (Instruction::AND, AddressingMode::IndexedIndirect),
        0x25 => (Instruction::AND, AddressingMode::ZeroPage),
        0x29 => (Instruction::AND, AddressingMode::Immediate),
        0x2D => (Instruction::AND, AddressingMode::Absolute),
        0x31 => (Instruction::AND, AddressingMode::IndirectIndexed),
        0x35 => (Instruction::AND, AddressingMode::ZeroPageIndexedX),
        0x3D => (Instruction::AND, AddressingMode::AbsoluteIndexedX),
        0x39 => (Instruction::AND, AddressingMode::AbsoluteIndexedY),

        0x41 => (Instruction::EOR, AddressingMode::IndexedIndirect),
        0x45 => (Instruction::EOR, AddressingMode::ZeroPage),
        0x49 => (Instruction::EOR, AddressingMode::Immediate),
        0x4D => (Instruction::EOR, AddressingMode::Absolute),
        0x51 => (Instruction::EOR, AddressingMode::IndirectIndexed),
        0x55 => (Instruction::EOR, AddressingMode::ZeroPageIndexedX),
        0x5D => (Instruction::EOR, AddressingMode::AbsoluteIndexedX),
        0x59 => (Instruction::EOR, AddressingMode::AbsoluteIndexedY),

        0x61 => (Instruction::ADC, AddressingMode::IndexedIndirect),
        0x65 => (Instruction::ADC, AddressingMode::ZeroPage),
        0x69 => (Instruction::ADC, AddressingMode::Immediate),
        0x6D => (Instruction::ADC, AddressingMode::Absolute),
        0x71 => (Instruction::ADC, AddressingMode::IndirectIndexed),
        0x75 => (Instruction::ADC, AddressingMode::ZeroPageIndexedX),
        0x7D => (Instruction::ADC, AddressingMode::AbsoluteIndexedX),
        0x79 => (Instruction::ADC, AddressingMode::AbsoluteIndexedY),

        0xC1 => (Instruction::CMP, AddressingMode::IndexedIndirect),
        0xC5 => (Instruction::CMP, AddressingMode::ZeroPage),
        0xC9 => (Instruction::CMP, AddressingMode::Immediate),
        0xCD => (Instruction::CMP, AddressingMode::Absolute),
        0xD1 => (Instruction::CMP, AddressingMode::IndirectIndexed),
        0xD5 => (Instruction::CMP, AddressingMode::ZeroPageIndexedX),
        0xDD => (Instruction::CMP, AddressingMode::AbsoluteIndexedX),
        0xD9 => (Instruction::CMP, AddressingMode::AbsoluteIndexedY),

        0xE1 => (Instruction::SBC, AddressingMode::IndexedIndirect),
        0xE5 => (Instruction::SBC, AddressingMode::ZeroPage),
        0xE9 => (Instruction::SBC, AddressingMode::Immediate),
        0xED => (Instruction::SBC, AddressingMode::Absolute),
        0xF1 => (Instruction::SBC, AddressingMode::IndirectIndexed),
        0xF5 => (Instruction::SBC, AddressingMode::ZeroPageIndexedX),
        0xFD => (Instruction::SBC, AddressingMode::AbsoluteIndexedX),
        0xF9 => (Instruction::SBC, AddressingMode::AbsoluteIndexedY),

        0x06 => (Instruction::ASL, AddressingMode::ZeroPage),
        0x0A => (Instruction::ASL, AddressingMode::Accumulator),
        0x0E => (Instruction::ASL, AddressingMode::Absolute),
        0x16 => (Instruction::ASL, AddressingMode::ZeroPageIndexedX),
        0x1E => (Instruction::ASL, AddressingMode::AbsoluteIndexedX),

        0x46 => (Instruction::LSR, AddressingMode::ZeroPage),
        0x4A => (Instruction::LSR, AddressingMode::Accumulator),
        0x4E => (Instruction::LSR, AddressingMode::Absolute),
        0x56 => (Instruction::LSR, AddressingMode::ZeroPageIndexedX),
        0x5E => (Instruction::LSR, AddressingMode::AbsoluteIndexedX),

        // Rotates
        0x26 => (Instruction::ROL, AddressingMode::ZeroPage),
        0x2A => (Instruction::ROL, AddressingMode::Accumulator),
        0x2E => (Instruction::ROL, AddressingMode::Absolute),
        0x36 => (Instruction::ROL, AddressingMode::ZeroPageIndexedX),
        0x3E => (Instruction::ROL, AddressingMode::AbsoluteIndexedX),

        0x66 => (Instruction::ROR, AddressingMode::ZeroPage),
        0x6A => (Instruction::ROR, AddressingMode::Accumulator),
        0x6E => (Instruction::ROR, AddressingMode::Absolute),
        0x76 => (Instruction::ROR, AddressingMode::ZeroPageIndexedX),
        0x7E => (Instruction::ROR, AddressingMode::AbsoluteIndexedX),

        // Increments
        0xC6 => (Instruction::DEC, AddressingMode::ZeroPage),
        0xCE => (Instruction::DEC, AddressingMode::Absolute),
        0xD6 => (Instruction::DEC, AddressingMode::ZeroPageIndexedX),
        0xDE => (Instruction::DEC, AddressingMode::AbsoluteIndexedX),

        0xE6 => (Instruction::INC, AddressingMode::ZeroPage),
        0xEE => (Instruction::INC, AddressingMode::Absolute),
        0xF6 => (Instruction::INC, AddressingMode::ZeroPageIndexedX),
        0xFE => (Instruction::INC, AddressingMode::AbsoluteIndexedX),

        // The ever important nop
        // Observe all its majesty
        0xEA => (Instruction::NOP, AddressingMode::Implied),

        /********END OF OFFICIAL OPCODES*********/
        _ => panic!("Unrecognized opcode: ${:02X}", op_code)
    }
}

// All instructions increment pc by 1.
// PC is also incremented in MemoryMap depending on addressing mode
pub fn execute(cpu: &mut Cpu,
               mem: &mut Memory,
               instr: (Instruction, AddressingMode))
{
    use self::Instruction;
    match instr.0 {
        Instruction::BRK => { cpu.BRK(mem); },
        Instruction::PHP => { cpu.PHP(mem); },
        Instruction::PLP => { cpu.PLP(mem); },
        Instruction::PHA => { cpu.PHA(mem); },
        Instruction::PLA => { cpu.PLA(mem); },
        Instruction::TXS => { cpu.TXS(); },
        Instruction::TSX => { cpu.TSX(); },
        Instruction::BPL => { cpu.BPL(mem); },
        Instruction::BMI => { cpu.BMI(mem); },
        Instruction::BVC => { cpu.BVC(mem); },
        Instruction::BVS => { cpu.BVS(mem); },
        Instruction::BCC => { cpu.BCC(mem); },
        Instruction::BCS => { cpu.BCS(mem); },
        Instruction::BNE => { cpu.BNE(mem); },
        Instruction::BEQ => { cpu.BEQ(mem); },
        Instruction::CLC => { cpu.CLC(); },
        Instruction::SEC => { cpu.SEC(); },
        Instruction::CLI => { cpu.CLI(); },
        Instruction::SEI => { cpu.SEI(); },
        Instruction::CLV => { cpu.CLV(); },
        Instruction::CLD => { cpu.CLD(); },
        Instruction::SED => { cpu.SED(); },
        Instruction::DEY => { cpu.DEY(); },
        Instruction::DEX => { cpu.DEX(); },
        Instruction::INX => { cpu.INX(); },
        Instruction::INY => { cpu.INY(); },
        Instruction::TAX => { cpu.TAX(); },
        Instruction::TXA => { cpu.TXA(); },
        Instruction::TAY => { cpu.TAY(); },
        Instruction::TYA => { cpu.TYA(); },
        Instruction::CPY => { cpu.CPY(mem, instr.1); },
        Instruction::CPX => { cpu.CPX(mem, instr.1); },
        Instruction::LDA => { cpu.LDA(mem, instr.1); },
        Instruction::LDX => { cpu.LDX(mem, instr.1); },
        Instruction::LDY => { cpu.LDY(mem, instr.1); },
        Instruction::STA => { cpu.STA(mem, instr.1); },
        Instruction::STX => { cpu.STX(mem, instr.1); },
        Instruction::STY => { cpu.STY(mem, instr.1); },
        Instruction::JSR => { cpu.JSR(mem); },
        Instruction::JMP => { cpu.JMP(mem, instr.1); },
        Instruction::RTI => { cpu.RTI(mem); },
        Instruction::RTS => { cpu.RTS(mem); },
        Instruction::BIT => { cpu.BIT(mem, instr.1); },
        Instruction::ORA => { cpu.ORA(mem, instr.1); },
        Instruction::AND => { cpu.AND(mem, instr.1); },
        Instruction::EOR => { cpu.EOR(mem, instr.1); },
        Instruction::ADC => { cpu.ADC(mem, instr.1); },
        Instruction::CMP => { cpu.CMP(mem, instr.1); },
        Instruction::SBC => { cpu.SBC(mem, instr.1); },
        Instruction::ASL => { cpu.ASL(mem, instr.1); },
        Instruction::LSR => { cpu.LSR(mem, instr.1); },
        Instruction::ROL => { cpu.ROL(mem, instr.1); },
        Instruction::ROR => { cpu.ROR(mem, instr.1); },
        Instruction::DEC => { cpu.DEC(mem, instr.1); },
        Instruction::INC => { cpu.INC(mem, instr.1); },
        Instruction::NOP => { cpu.NOP(); },
    }
}
