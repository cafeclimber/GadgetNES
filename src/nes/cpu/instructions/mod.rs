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
mod unofficial;

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
#[allow(non_camel_case_types)]
/// All 6502 instructions
pub enum Instruction {
    BRK, PHP, PLP, PHA, PLA, TXS, TSX, BPL,
    BMI, BVC, BVS, BCC, BCS, BNE, BEQ, CLC,
    SEC, CLI, SEI, CLV, CLD, SED, DEY, DEX,
    INX, INY, TAX, TXA, TAY, TYA, CPY, CPX,
    LDA, LDX, LDY, STA, STX, STY, JSR, JMP,
    RTI, RTS, BIT, ORA, AND, EOR, ADC, CMP,
    SBC, ASL, LSR, ROL, ROR, DEC, INC, NOP,

    // Below are unofficial opcodes denoted by an appended _u
    NOP_u, LAX_u, SAX_u, SBC_u, DCP_u, ISC_u,
    SLO_u, RLA_u, SRE_u, RRA_u, ALR_u, ANC_u,
    ARR_u, AXS_u, AXA_u, XAS_u, SYA_u, SXA_u,
    ATX_u, LAR_u,
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
        0x00 => {(BRK,Implied)},
        
        // Stack
        0x08 => {(PHP, Implied)},
        0x28 => {(PLP, Implied)},
        0x48 => {(PHA, Implied)},
        0x68 => {(PLA, Implied)},
        0x9A => {(TXS, Implied)},
        0xBA => {(TSX, Implied)},

        // Branch
        0x10 => {(BPL, Relative)},
        0x30 => {(BMI, Relative)},
        0x50 => {(BVC, Relative)},
        0x70 => {(BVS, Relative)},
        0x90 => {(BCC, Relative)},
        0xB0 => {(BCS, Relative)},
        0xD0 => {(BNE, Relative)},
        0xF0 => {(BEQ, Relative)},

        // Flag instructions
        0x18 => {(CLC, Implied)},
        0x38 => {(SEC, Implied)},
        0x58 => {(CLI, Implied)},
        0x78 => {(SEI, Implied)},
        0xB8 => {(CLV, Implied)},
        0xD8 => {(CLD, Implied)},
        0xF8 => {(SED, Implied)},

        // Register instructions
        0x88 => {(DEY, Implied)},
        0xCA => {(DEX, Implied)},
        0xE8 => {(INX, Implied)},
        0xC8 => {(INY, Implied)},
        0xAA => {(TAX, Implied)},
        0x8A => {(TXA, Implied)},
        0xA8 => {(TAY, Implied)},
        0x98 => {(TYA, Implied)},

        // Compares
        0xC0 => {(CPY, Immediate)},
        0xC4 => {(CPY, ZeroPage)},
        0xCC => {(CPY, Absolute)},
        0xE0 => {(CPX, Immediate)},
        0xE4 => {(CPX, ZeroPage)},
        0xEC => {(CPX, Absolute)},

        // Loads
        0xA1 => {(LDA, IndexedIndirect)},
        0xA5 => {(LDA, ZeroPage)},
        0xA9 => {(LDA, Immediate)},
        0xAD => {(LDA, Absolute)},
        0xB1 => {(LDA, IndirectIndexed)},
        0xB5 => {(LDA, ZeroPageIndexedX)},
        0xBD => {(LDA, AbsoluteIndexedX)},
        0xB9 => {(LDA, AbsoluteIndexedY)},

        0xA2 => {(LDX, Immediate)},
        0xA6 => {(LDX, ZeroPage)},
        0xAE => {(LDX, Absolute)},
        0xB6 => {(LDX, ZeroPageIndexedY)},
        0xBE => {(LDX, AbsoluteIndexedY)},

        0xA0 => {(LDY, Immediate)},
        0xA4 => {(LDY, ZeroPage)},
        0xAC => {(LDY, Absolute)},
        0xB4 => {(LDY, ZeroPageIndexedX)},
        0xBC => {(LDY, AbsoluteIndexedX)},

        // Stores
        0x81 => {(STA, IndexedIndirect)},
        0x85 => {(STA, ZeroPage)},
        0x8D => {(STA, Absolute)},
        0x91 => {(STA, IndirectIndexed)},
        0x95 => {(STA, ZeroPageIndexedX)},
        0x9D => {(STA, AbsoluteIndexedX)},
        0x99 => {(STA, AbsoluteIndexedY)},

        0x86 => {(STX, ZeroPage)},
        0x8E => {(STX, Absolute)},
        0x96 => {(STX, ZeroPageIndexedY)},

        0x84 => {(STY, ZeroPage)},
        0x8C => {(STY, Absolute)},
        0x94 => {(STY, ZeroPageIndexedX)},

        // Jumps
        0x20 => {(JSR, Absolute)},
        0x4C => {(JMP, Absolute)},
        0x6C => {(JMP, Indirect)},

        0x40 => {(RTI, Implied)},
        0x60 => {(RTS, Implied)},

        // Bit tests
        0x24 => {(BIT, ZeroPage)},
        0x2C => {(BIT, Absolute)},

        // ALU operations
        0x01 => {(ORA, IndexedIndirect)},
        0x05 => {(ORA, ZeroPage)},
        0x09 => {(ORA, Immediate)},
        0x0D => {(ORA, Absolute)},
        0x11 => {(ORA, IndirectIndexed)},
        0x15 => {(ORA, ZeroPageIndexedX)},
        0x1D => {(ORA, AbsoluteIndexedX)},
        0x19 => {(ORA, AbsoluteIndexedY)},

        0x21 => {(AND, IndexedIndirect)},
        0x25 => {(AND, ZeroPage)},
        0x29 => {(AND, Immediate)},
        0x2D => {(AND, Absolute)},
        0x31 => {(AND, IndirectIndexed)},
        0x35 => {(AND, ZeroPageIndexedX)},
        0x3D => {(AND, AbsoluteIndexedX)},
        0x39 => {(AND, AbsoluteIndexedY)},

        0x41 => {(EOR, IndexedIndirect)},
        0x45 => {(EOR, ZeroPage)},
        0x49 => {(EOR, Immediate)},
        0x4D => {(EOR, Absolute)},
        0x51 => {(EOR, IndirectIndexed)},
        0x55 => {(EOR, ZeroPageIndexedX)},
        0x5D => {(EOR, AbsoluteIndexedX)},
        0x59 => {(EOR, AbsoluteIndexedY)},

        0x61 => {(ADC, IndexedIndirect)},
        0x65 => {(ADC, ZeroPage)},
        0x69 => {(ADC, Immediate)},
        0x6D => {(ADC, Absolute)},
        0x71 => {(ADC, IndirectIndexed)},
        0x75 => {(ADC, ZeroPageIndexedX)},
        0x7D => {(ADC, AbsoluteIndexedX)},
        0x79 => {(ADC, AbsoluteIndexedY)},

        0xC1 => {(CMP, IndexedIndirect)},
        0xC5 => {(CMP, ZeroPage)},
        0xC9 => {(CMP, Immediate)},
        0xCD => {(CMP, Absolute)},
        0xD1 => {(CMP, IndirectIndexed)},
        0xD5 => {(CMP, ZeroPageIndexedX)},
        0xDD => {(CMP, AbsoluteIndexedX)},
        0xD9 => {(CMP, AbsoluteIndexedY)},

        0xE1 => {(SBC, IndexedIndirect)},
        0xE5 => {(SBC, ZeroPage)},
        0xE9 => {(SBC, Immediate)},
        0xED => {(SBC, Absolute)},
        0xF1 => {(SBC, IndirectIndexed)},
        0xF5 => {(SBC, ZeroPageIndexedX)},
        0xFD => {(SBC, AbsoluteIndexedX)},
        0xF9 => {(SBC, AbsoluteIndexedY)},

        0x06 => {(ASL, ZeroPage)},
        0x0A => {(ASL, Accumulator)},
        0x0E => {(ASL, Absolute)},
        0x16 => {(ASL, ZeroPageIndexedX)},
        0x1E => {(ASL, AbsoluteIndexedX)},

        0x46 => {(LSR, ZeroPage)},
        0x4A => {(LSR, Accumulator)},
        0x4E => {(LSR, Absolute)},
        0x56 => {(LSR, ZeroPageIndexedX)},
        0x5E => {(LSR, AbsoluteIndexedX)},

        // Rotates
        0x26 => {(ROL, ZeroPage)},
        0x2A => {(ROL, Accumulator)},
        0x2E => {(ROL, Absolute)},
        0x36 => {(ROL, ZeroPageIndexedX)},
        0x3E => {(ROL, AbsoluteIndexedX)},

        0x66 => {(ROR, ZeroPage)},
        0x6A => {(ROR, Accumulator)},
        0x6E => {(ROR, Absolute)},
        0x76 => {(ROR, ZeroPageIndexedX)},
        0x7E => {(ROR, AbsoluteIndexedX)},

        // Increments
        0xC6 => {(DEC, ZeroPage)},
        0xCE => {(DEC, Absolute)},
        0xD6 => {(DEC, ZeroPageIndexedX)},
        0xDE => {(DEC, AbsoluteIndexedX)},

        0xE6 => {(INC, ZeroPage)},
        0xEE => {(INC, Absolute)},
        0xF6 => {(INC, ZeroPageIndexedX)},
        0xFE => {(INC, AbsoluteIndexedX)},

        // The ever important nop
        // Observe all its majesty
        0xEA => {cpu.cycle+= 2; (NOP, Implied)},

        /******** END OF OFFICIAL OPCODES *********/
        /*********** UNOFFICIAL OPCODES ***********/
        0x04 => {(NOP_u, ZeroPage)},
        0x44 => {(NOP_u, ZeroPage)},
        0x64 => {(NOP_u, ZeroPage)},

        0x0C => {(NOP_u, Absolute)},

        0x14 => {(NOP_u, ZeroPageIndexedX)},
        0x34 => {(NOP_u, ZeroPageIndexedX)},
        0x54 => {(NOP_u, ZeroPageIndexedX)},
        0x74 => {(NOP_u, ZeroPageIndexedX)},
        0xD4 => {(NOP_u, ZeroPageIndexedX)},
        0xF4 => {(NOP_u, ZeroPageIndexedX)},

        0x1A => {(NOP_u, Implied)},
        0x3A => {(NOP_u, Implied)},
        0x5A => {(NOP_u, Implied)},
        0x7A => {(NOP_u, Implied)},
        0xDA => {(NOP_u, Implied)},
        0xFA => {(NOP_u, Implied)},

        0x80 => {(NOP_u, Immediate)},
        0x82 => {(NOP_u, Immediate)},
        0x89 => {(NOP_u, Immediate)},
        0xC2 => {(NOP_u, Immediate)},
        0xE2 => {(NOP_u, Immediate)},

        0x1C => {(NOP_u, AbsoluteIndexedX)},
        0x3C => {(NOP_u, AbsoluteIndexedX)},
        0x5C => {(NOP_u, AbsoluteIndexedX)},
        0x7C => {(NOP_u, AbsoluteIndexedX)},
        0xDC => {(NOP_u, AbsoluteIndexedX)},
        0xFC => {(NOP_u, AbsoluteIndexedX)},

        0xA7 => {(LAX_u, ZeroPage)},
        0xB7 => {(LAX_u, ZeroPageIndexedY)},
        0xAF => {(LAX_u, Absolute)},
        0xBF => {(LAX_u, AbsoluteIndexedY)},
        0xA3 => {(LAX_u, IndexedIndirect)},
        0xB3 => {(LAX_u, IndirectIndexed)},

        0x87 => {(SAX_u, ZeroPage)},
        0x97 => {(SAX_u, ZeroPageIndexedY)},
        0x83 => {(SAX_u, IndexedIndirect)},
        0x8F => {(SAX_u, Absolute)},

        0xEB => {(SBC_u, Immediate)},

        0xC3 => {(DCP_u, IndexedIndirect)},
        0xC7 => {(DCP_u, ZeroPage)},
        0xCF => {(DCP_u, Absolute)},
        0xD3 => {(DCP_u, IndirectIndexed)},
        0xD7 => {(DCP_u, ZeroPageIndexedX)},
        0xDB => {(DCP_u, AbsoluteIndexedY)},
        0xDF => {(DCP_u, AbsoluteIndexedX)},

        0xE3 => {(ISC_u, IndexedIndirect)},
        0xE7 => {(ISC_u, ZeroPage)},
        0xEF => {(ISC_u, Absolute)},
        0xF3 => {(ISC_u, IndirectIndexed)},
        0xF7 => {(ISC_u, ZeroPageIndexedX)},
        0xFB => {(ISC_u, AbsoluteIndexedY)},
        0xFF => {(ISC_u, AbsoluteIndexedX)},

        0x03 => {(SLO_u, IndexedIndirect)},
        0x07 => {(SLO_u, ZeroPage)},
        0x0F => {(SLO_u, Absolute)},
        0x13 => {(SLO_u, IndirectIndexed)},
        0x17 => {(SLO_u, ZeroPageIndexedX)},
        0x1B => {(SLO_u, AbsoluteIndexedY)},
        0x1F => {(SLO_u, AbsoluteIndexedX)},

        0x23 => {(RLA_u, IndexedIndirect)},
        0x27 => {(RLA_u, ZeroPage)},
        0x2F => {(RLA_u, Absolute)},
        0x33 => {(RLA_u, IndirectIndexed)},
        0x37 => {(RLA_u, ZeroPageIndexedX)},
        0x3B => {(RLA_u, AbsoluteIndexedY)},
        0x3F => {(RLA_u, AbsoluteIndexedX)},

        0x43 => {(SRE_u, IndexedIndirect)},
        0x47 => {(SRE_u, ZeroPage)},
        0x4F => {(SRE_u, Absolute)},
        0x53 => {(SRE_u, IndirectIndexed)},
        0x57 => {(SRE_u, ZeroPageIndexedX)},
        0x5B => {(SRE_u, AbsoluteIndexedY)},
        0x5F => {(SRE_u, AbsoluteIndexedX)},

        0x63 => {(RRA_u, IndexedIndirect)},
        0x67 => {(RRA_u, ZeroPage)},
        0x6F => {(RRA_u, Absolute)},
        0x73 => {(RRA_u, IndirectIndexed)},
        0x77 => {(RRA_u, ZeroPageIndexedX)},
        0x7B => {(RRA_u, AbsoluteIndexedY)},
        0x7F => {(RRA_u, AbsoluteIndexedX)},

        0x4B => {(ALR_u, Immediate)},

        0x0B => {(ANC_u, Immediate)},
        0x2B => {(ANC_u, Immediate)},

        0x6B => {(ARR_u, Immediate)},

        0xCB => {(AXS_u, Immediate)},

        0x93 => {(AXA_u, AbsoluteIndexedY)},
        0x9F => {(AXA_u, IndirectIndexed)},

        0x9B => {(XAS_u, AbsoluteIndexedY)},

        0x9C => {(SYA_u, AbsoluteIndexedX)},

        0x9E => {(SXA_u, AbsoluteIndexedY)},

        0xAB => {(ATX_u, Immediate)},

        0xBB => {(LAR_u, AbsoluteIndexedY)},

        // Apparently these codes just lock the processor requiring reboot
        0x02 => panic!("KIL Opcode. Execution Halted"),
        0x12 => panic!("KIL Opcode. Execution Halted"),
        0x22 => panic!("KIL Opcode. Execution Halted"),
        0x32 => panic!("KIL Opcode. Execution Halted"),
        0x42 => panic!("KIL Opcode. Execution Halted"),
        0x52 => panic!("KIL Opcode. Execution Halted"),
        0x62 => panic!("KIL Opcode. Execution Halted"),
        0x72 => panic!("KIL Opcode. Execution Halted"),
        0x92 => panic!("KIL Opcode. Execution Halted"),
        0xB2 => panic!("KIL Opcode. Execution Halted"),
        0xD2 => panic!("KIL Opcode. Execution Halted"),
        0xF2 => panic!("KIL Opcode. Execution Halted"),

        // TODO: Implement this?
        0x8B => panic!("XAA Opcode. Execution Unpredictable"),
        
        _ => unreachable!()
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

        // Unofficial opcodes
        NOP_u => { cpu.NOP_u(); },
        SBC_u => { cpu.SBC(mem, instr.1); },
        LAX_u => { cpu.LAX_u(mem, instr.1); },
        SAX_u => { cpu.SAX_u(mem, instr.1); },
        DCP_u => { cpu.DCP_u(mem, instr.1); },
        ISC_u => { cpu.ISC_u(mem, instr.1); },
        SLO_u => { cpu.SLO_u(mem, instr.1); },
        RLA_u => { cpu.RLA_u(mem, instr.1); },
        SRE_u => { cpu.SRE_u(mem, instr.1); },
        RRA_u => { cpu.RRA_u(mem, instr.1); },
        ALR_u => { cpu.ALR_u(mem, instr.1); },
        ANC_u => { cpu.ANC_u(mem, instr.1); },
        ARR_u => { cpu.ARR_u(mem, instr.1); },
        AXS_u => { cpu.AXS_u(mem, instr.1); },
        AXA_u => { cpu.AXA_u(mem, instr.1); },
        XAS_u => { cpu.XAS_u(mem, instr.1); },
        SYA_u => { cpu.SYA_u(mem, instr.1); },
        SXA_u => { cpu.SXA_u(mem, instr.1); },
        ATX_u => { cpu.ATX_u(mem, instr.1); },
        LAR_u => { cpu.LAR_u(mem, instr.1); },
    }
}
