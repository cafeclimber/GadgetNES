//! Handles instruction fetch and decode for the CPU.
//! Instructions are split into files by their approximate functionality.
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

/// All addressing modes used by the 6502 processor
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
/// All 6502 instructions
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
///
/// #Panics
/// Will panic if an instruction is not recognized.
pub fn decode(cpu: &mut Cpu, op_code: u8) -> (Instruction, AddressingMode) {
    use self::Instruction::*;
    use self::AddressingMode::*;
    match op_code {
        /********OFFICIAL OPCODES*********/
        0x00 => {cpu.cycle+= 7; (Instruction::BRK,AddressingMode::Implied)},
        
        // Stack
        0x08 => {cpu.cycle+= 3; (PHP, Implied)},
        0x28 => {cpu.cycle+= 4; (PLP, Implied)},
        0x48 => {cpu.cycle+= 3; (PHA, Implied)},
        0x68 => {cpu.cycle+= 4; (PLA, Implied)},
        0x9A => {cpu.cycle+= 2; (TXS, Implied)},
        0xBA => {cpu.cycle+= 2; (TSX, Implied)},

        // Branch
        0x10 => {cpu.cycle+= 2; (BPL, Relative)},
        0x30 => {cpu.cycle+= 2; (BMI, Relative)},
        0x50 => {cpu.cycle+= 2; (BVC, Relative)},
        0x70 => {cpu.cycle+= 2; (BVS, Relative)},
        0x90 => {cpu.cycle+= 2; (BCC, Relative)},
        0xB0 => {cpu.cycle+= 2; (BCS, Relative)},
        0xD0 => {cpu.cycle+= 2; (BNE, Relative)},
        0xF0 => {cpu.cycle+= 2; (BEQ, Relative)},

        // Flag instructions
        0x18 => {cpu.cycle+= 2; (CLC, Implied)},
        0x38 => {cpu.cycle+= 2; (SEC, Implied)},
        0x58 => {cpu.cycle+= 2; (CLI, Implied)},
        0x78 => {cpu.cycle+= 2; (SEI, Implied)},
        0xB8 => {cpu.cycle+= 2; (CLV, Implied)},
        0xD8 => {cpu.cycle+= 2; (CLD, Implied)},
        0xF8 => {cpu.cycle+= 2; (SED, Implied)},

        // Register instructions
        0x88 => {cpu.cycle+= 2; (DEY, Implied)},
        0xCA => {cpu.cycle+= 2; (DEX, Implied)},
        0xE8 => {cpu.cycle+= 2; (INX, Implied)},
        0xC8 => {cpu.cycle+= 2; (INY, Implied)},
        0xAA => {cpu.cycle+= 2; (TAX, Implied)},
        0x8A => {cpu.cycle+= 2; (TXA, Implied)},
        0xA8 => {cpu.cycle+= 2; (TAY, Implied)},
        0x98 => {cpu.cycle+= 2; (TYA, Implied)},

        // Compares
        0xC0 => {cpu.cycle+= 2; (CPY, Immediate)},
        0xC4 => {cpu.cycle+= 3; (CPY, ZeroPage)},
        0xCC => {cpu.cycle+= 4; (CPY, Absolute)},
        0xE0 => {cpu.cycle+= 2; (CPX, Immediate)},
        0xE4 => {cpu.cycle+= 3; (CPX, ZeroPage)},
        0xEC => {cpu.cycle+= 4; (CPX, Absolute)},

        // Loads
        0xA1 => {cpu.cycle+= 6; (LDA, IndexedIndirect)},
        0xA5 => {cpu.cycle+= 3; (LDA, ZeroPage)},
        0xA9 => {cpu.cycle+= 2; (LDA, Immediate)},
        0xAD => {cpu.cycle+= 4; (LDA, Absolute)},
        0xB1 => {cpu.cycle+= 5; (LDA, IndirectIndexed)},
        0xB5 => {cpu.cycle+= 4; (LDA, ZeroPageIndexedX)},
        0xBD => {cpu.cycle+= 4; (LDA, AbsoluteIndexedX)},
        0xB9 => {cpu.cycle+= 4; (LDA, AbsoluteIndexedY)},

        0xA2 => {cpu.cycle+= 2; (LDX, Immediate)},
        0xA6 => {cpu.cycle+= 3; (LDX, ZeroPage)},
        0xAE => {cpu.cycle+= 4; (LDX, Absolute)},
        0xB6 => {cpu.cycle+= 4; (LDX, ZeroPageIndexedY)},
        0xBE => {cpu.cycle+= 4; (LDX, AbsoluteIndexedY)},

        0xA0 => {cpu.cycle+= 2; (LDY, Immediate)},
        0xA4 => {cpu.cycle+= 3; (LDY, ZeroPage)},
        0xAC => {cpu.cycle+= 4; (LDY, Absolute)},
        0xB4 => {cpu.cycle+= 4; (LDY, ZeroPageIndexedX)},
        0xBC => {cpu.cycle+= 4; (LDY, AbsoluteIndexedX)},

        // Stores
        0x81 => {cpu.cycle+= 6; (STA, IndexedIndirect)},
        0x85 => {cpu.cycle+= 3; (STA, ZeroPage)},
        0x8D => {cpu.cycle+= 4; (STA, Absolute)},
        0x91 => {cpu.cycle+= 6; (STA, IndirectIndexed)},
        0x95 => {cpu.cycle+= 3; (STA, ZeroPageIndexedX)},
        0x9D => {cpu.cycle+= 5; (STA, AbsoluteIndexedX)},
        0x99 => {cpu.cycle+= 5; (STA, AbsoluteIndexedY)},

        0x86 => {cpu.cycle+= 3; (STX, ZeroPage)},
        0x8E => {cpu.cycle+= 4; (STX, Absolute)},
        0x96 => {cpu.cycle+= 4; (STX, ZeroPageIndexedY)},

        0x84 => {cpu.cycle+= 3; (STY, ZeroPage)},
        0x8C => {cpu.cycle+= 4; (STY, Absolute)},
        0x94 => {cpu.cycle+= 4; (STY, ZeroPageIndexedX)},

        // Jumps
        0x20 => {cpu.cycle+= 6; (JSR, Absolute)},
        0x4C => {cpu.cycle+= 3; (JMP, Absolute)},
        0x6C => {cpu.cycle+= 5; (JMP, Indirect)},

        0x40 => {cpu.cycle+= 6; (RTI, Implied)},
        0x60 => {cpu.cycle+= 6; (RTS, Implied)},

        // Bit tests
        0x24 => {cpu.cycle+= 3; (BIT, ZeroPage)},
        0x2C => {cpu.cycle+= 4; (BIT, Absolute)},

        // ALU operations
        0x01 => {cpu.cycle+= 6; (ORA, IndexedIndirect)},
        0x05 => {cpu.cycle+= 3; (ORA, ZeroPage)},
        0x09 => {cpu.cycle+= 2; (ORA, Immediate)},
        0x0D => {cpu.cycle+= 4; (ORA, Absolute)},
        0x11 => {cpu.cycle+= 5; (ORA, IndirectIndexed)},
        0x15 => {cpu.cycle+= 4; (ORA, ZeroPageIndexedX)},
        0x1D => {cpu.cycle+= 4; (ORA, AbsoluteIndexedX)},
        0x19 => {cpu.cycle+= 4; (ORA, AbsoluteIndexedY)},

        0x21 => {cpu.cycle+= 6; (AND, IndexedIndirect)},
        0x25 => {cpu.cycle+= 3; (AND, ZeroPage)},
        0x29 => {cpu.cycle+= 2; (AND, Immediate)},
        0x2D => {cpu.cycle+= 4; (AND, Absolute)},
        0x31 => {cpu.cycle+= 5; (AND, IndirectIndexed)},
        0x35 => {cpu.cycle+= 4; (AND, ZeroPageIndexedX)},
        0x3D => {cpu.cycle+= 4; (AND, AbsoluteIndexedX)},
        0x39 => {cpu.cycle+= 4; (AND, AbsoluteIndexedY)},

        0x41 => {cpu.cycle+= 6; (EOR, IndexedIndirect)},
        0x45 => {cpu.cycle+= 3; (EOR, ZeroPage)},
        0x49 => {cpu.cycle+= 2; (EOR, Immediate)},
        0x4D => {cpu.cycle+= 4; (EOR, Absolute)},
        0x51 => {cpu.cycle+= 5; (EOR, IndirectIndexed)},
        0x55 => {cpu.cycle+= 4; (EOR, ZeroPageIndexedX)},
        0x5D => {cpu.cycle+= 4; (EOR, AbsoluteIndexedX)},
        0x59 => {cpu.cycle+= 4; (EOR, AbsoluteIndexedY)},

        0x61 => {cpu.cycle+= 6; (ADC, IndexedIndirect)},
        0x65 => {cpu.cycle+= 3; (ADC, ZeroPage)},
        0x69 => {cpu.cycle+= 2; (ADC, Immediate)},
        0x6D => {cpu.cycle+= 4; (ADC, Absolute)},
        0x71 => {cpu.cycle+= 5; (ADC, IndirectIndexed)},
        0x75 => {cpu.cycle+= 4; (ADC, ZeroPageIndexedX)},
        0x7D => {cpu.cycle+= 4; (ADC, AbsoluteIndexedX)},
        0x79 => {cpu.cycle+= 4; (ADC, AbsoluteIndexedY)},

        0xC1 => {cpu.cycle+= 6; (CMP, IndexedIndirect)},
        0xC5 => {cpu.cycle+= 3; (CMP, ZeroPage)},
        0xC9 => {cpu.cycle+= 2; (CMP, Immediate)},
        0xCD => {cpu.cycle+= 4; (CMP, Absolute)},
        0xD1 => {cpu.cycle+= 5; (CMP, IndirectIndexed)},
        0xD5 => {cpu.cycle+= 4; (CMP, ZeroPageIndexedX)},
        0xDD => {cpu.cycle+= 4; (CMP, AbsoluteIndexedX)},
        0xD9 => {cpu.cycle+= 4; (CMP, AbsoluteIndexedY)},

        0xE1 => {cpu.cycle+= 6; (SBC, IndexedIndirect)},
        0xE5 => {cpu.cycle+= 3; (SBC, ZeroPage)},
        0xE9 => {cpu.cycle+= 2; (SBC, Immediate)},
        0xED => {cpu.cycle+= 4; (SBC, Absolute)},
        0xF1 => {cpu.cycle+= 5; (SBC, IndirectIndexed)},
        0xF5 => {cpu.cycle+= 4; (SBC, ZeroPageIndexedX)},
        0xFD => {cpu.cycle+= 4; (SBC, AbsoluteIndexedX)},
        0xF9 => {cpu.cycle+= 4; (SBC, AbsoluteIndexedY)},

        0x06 => {cpu.cycle+= 5; (ASL, ZeroPage)},
        0x0A => {cpu.cycle+= 2; (ASL, Accumulator)},
        0x0E => {cpu.cycle+= 6; (ASL, Absolute)},
        0x16 => {cpu.cycle+= 6; (ASL, ZeroPageIndexedX)},
        0x1E => {cpu.cycle+= 7; (ASL, AbsoluteIndexedX)},

        0x46 => {cpu.cycle+= 5; (LSR, ZeroPage)},
        0x4A => {cpu.cycle+= 2; (LSR, Accumulator)},
        0x4E => {cpu.cycle+= 6; (LSR, Absolute)},
        0x56 => {cpu.cycle+= 6; (LSR, ZeroPageIndexedX)},
        0x5E => {cpu.cycle+= 7; (LSR, AbsoluteIndexedX)},

        // Rotates
        0x26 => {cpu.cycle+= 5; (ROL, ZeroPage)},
        0x2A => {cpu.cycle+= 2; (ROL, Accumulator)},
        0x2E => {cpu.cycle+= 6; (ROL, Absolute)},
        0x36 => {cpu.cycle+= 6; (ROL, ZeroPageIndexedX)},
        0x3E => {cpu.cycle+= 7; (ROL, AbsoluteIndexedX)},

        0x66 => {cpu.cycle+= 5; (ROR, ZeroPage)},
        0x6A => {cpu.cycle+= 2; (ROR, Accumulator)},
        0x6E => {cpu.cycle+= 6; (ROR, Absolute)},
        0x76 => {cpu.cycle+= 6; (ROR, ZeroPageIndexedX)},
        0x7E => {cpu.cycle+= 7; (ROR, AbsoluteIndexedX)},

        // Increments
        0xC6 => {cpu.cycle+= 5; (DEC, ZeroPage)},
        0xCE => {cpu.cycle+= 6; (DEC, Absolute)},
        0xD6 => {cpu.cycle+= 6; (DEC, ZeroPageIndexedX)},
        0xDE => {cpu.cycle+= 7; (DEC, AbsoluteIndexedX)},

        0xE6 => {cpu.cycle+= 5; (INC, ZeroPage)},
        0xEE => {cpu.cycle+= 6; (INC, Absolute)},
        0xF6 => {cpu.cycle+= 6; (INC, ZeroPageIndexedX)},
        0xFE => {cpu.cycle+= 7; (INC, AbsoluteIndexedX)},

        // The ever important nop
        // Observe all its majesty
        0xEA => {cpu.cycle+= 2; (NOP, Implied)},

        /******** END OF OFFICIAL OPCODES *********/
        _ => panic!("Unrecognized opcode: ${:02X}", op_code)
    }
}

/// Executes the instruction by calling a cpu function
pub fn execute(cpu: &mut Cpu,
               mem: &mut Memory,
               instr: (Instruction, AddressingMode))
{
    use self::Instruction::*;
    match instr.0 {
        Instruction::BRK => { cpu.BRK(mem); },
        PHP => { cpu.PHP(mem); },
        PLP => { cpu.PLP(mem); },
        PHA => { cpu.PHA(mem); },
        PLA => { cpu.PLA(mem); },
        TXS => { cpu.TXS(); },
        TSX => { cpu.TSX(); },
        BPL => { cpu.BPL(mem); },
        BMI => { cpu.BMI(mem); },
        BVC => { cpu.BVC(mem); },
        BVS => { cpu.BVS(mem); },
        BCC => { cpu.BCC(mem); },
        BCS => { cpu.BCS(mem); },
        BNE => { cpu.BNE(mem); },
        BEQ => { cpu.BEQ(mem); },
        CLC => { cpu.CLC(); },
        SEC => { cpu.SEC(); },
        CLI => { cpu.CLI(); },
        SEI => { cpu.SEI(); },
        CLV => { cpu.CLV(); },
        CLD => { cpu.CLD(); },
        SED => { cpu.SED(); },
        DEY => { cpu.DEY(); },
        DEX => { cpu.DEX(); },
        INX => { cpu.INX(); },
        INY => { cpu.INY(); },
        TAX => { cpu.TAX(); },
        TXA => { cpu.TXA(); },
        TAY => { cpu.TAY(); },
        TYA => { cpu.TYA(); },
        CPY => { cpu.CPY(mem, instr.1); },
        CPX => { cpu.CPX(mem, instr.1); },
        LDA => { cpu.LDA(mem, instr.1); },
        LDX => { cpu.LDX(mem, instr.1); },
        LDY => { cpu.LDY(mem, instr.1); },
        STA => { cpu.STA(mem, instr.1); },
        STX => { cpu.STX(mem, instr.1); },
        STY => { cpu.STY(mem, instr.1); },
        JSR => { cpu.JSR(mem); },
        JMP => { cpu.JMP(mem, instr.1); },
        RTI => { cpu.RTI(mem); },
        RTS => { cpu.RTS(mem); },
        BIT => { cpu.BIT(mem, instr.1); },
        ORA => { cpu.ORA(mem, instr.1); },
        AND => { cpu.AND(mem, instr.1); },
        EOR => { cpu.EOR(mem, instr.1); },
        ADC => { cpu.ADC(mem, instr.1); },
        CMP => { cpu.CMP(mem, instr.1); },
        SBC => { cpu.SBC(mem, instr.1); },
        ASL => { cpu.ASL(mem, instr.1); },
        LSR => { cpu.LSR(mem, instr.1); },
        ROL => { cpu.ROL(mem, instr.1); },
        ROR => { cpu.ROR(mem, instr.1); },
        DEC => { cpu.DEC(mem, instr.1); },
        INC => { cpu.INC(mem, instr.1); },
        NOP => { cpu.NOP(); },
    }
}
