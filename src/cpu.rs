use std::fmt;
use num::FromPrimitive;
use super::apu::Apu;
use super::ppu::Ppu;
use super::instruction::Instruction;
use super::interconnect::Interconnect;

const NEGATIVE_FLAG:u8 = 1 << 7;
const OVERFLOW_FLAG:u8 = 1 << 6;
const IRQ_FLAG:     u8 = 1 << 4;
const DECIMAL_FLAG: u8 = 1 << 3;
const INTERUPT_FLAG:u8 = 1 << 2;
const ZERO_FLAG:    u8 = 1 << 1;
const CARRY_FLAG:   u8 = 1 << 0;

#[derive(Default)]
pub struct Cpu {
    a: u8, // Accumulator

    x: u8, // x-Index
    y: u8, // y-index

    pc: u16, // Program counter
    
    s: u8, // Stack pointer

    p: u8, // Status register
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            a: 0,

            x: 0,
            y: 0,

            pc: 0x10, // TODO make this a result of header information

            s: 0,

            p: 0,
        } 
    }

    pub fn power_up(&mut self) {
        self.p = 0x34;
        self.s = 0xfd;
    }

    fn check_flag(&mut self, flag: u8) -> bool{
        if self.p & flag > 0 {true} else {false}
    }
    
    fn set_flag(&mut self, flag: u8) {
        self.p = self.p | flag;
    }

    fn unset_flag(&mut self, flag: u8) {
        self.p = self.p & !flag;
    }

    // Stack helpers TODO
    fn pushs(&mut self, value: u8) {
        // TODO self.ram[(0x100 + self.s as u16) as usize] = value;
        self.s -= 1;
    }

    pub fn run_instr(&self, interconnect: &mut Interconnect) {
        let instr = interconnect.read_byte(self.pc + 0x8000);
        match Instruction::from_u8(instr) {
            // TODO: Implement unofficial opcodes
            // BRK       => {},

            // Stack    
            // PHP       => {},
            // PLP       => {},
            // PHA       => {},
            // PLA       => {},
            // TXS       => {},
            // TSX       => {},

            // Branch   
            // BPL       => {},
            // BMI       => {},
            // BVC       => {},
            // BVS       => {},
            // BCC       => {},
            // BCS       => {},
            // BNE       => {},
            // BEQ       => {},

            // Flag instructions
            // CLC       => {},
            // SEC       => {},
            // CLI       => {},
            // SEI       => {},
            // CLV       => {},
            // CLD       => {},
            // SED       => {},

            // Register instructions
            // DEY       => {},
            // DEX       => {},
            // INX       => {},
            // INY       => {},
            // TAX       => {},
            // TXA       => {},
            // TAY       => {},
            // TYA       => {},

            // Compares
            // CPY_imm   => {}
            // CPY_z_pg  => {},
            // CPY_abs   => {},
            // CPX_imm   => {},
            // CPX_z_pg  => {},
            // CPX_abs   => {},

            // Loads
            // LDA_inx_x => {},
            // LDA_z_pg  => {},
            // LDA_imm   => {},
            // LDA_abs   => {},
            // LDA_ind_y => {},
            // LDA_dx    => {},
            // LDA_ax    => {},
            // LDA_ay    => {},

            // LDX_imm   => {},
            // LDX_z_pg  => {},
            // LDX_abs   => {},
            // LDX_dy    => {},
            // LDX_ay    => {},

            // LDY_imm   => {},
            // LDY_z_pg  => {},
            // LDY_abs   => {},
            // LDY_dx    => {},
            // LDY_ax    => {},

            // Stores
            // STA_inx_x => {},
            // STA_z_pg  => {},
            // STA_abs   => {},
            // STA_ind_y => {},
            // STA_dx    => {},
            // STA_ax    => {},
            // STA_ay    => {},

            // STX_z_pg  => {},
            // STX_abs   => {},
            // STX_dy    => {},

            // STY_z_pg  => {},
            // STY_abs   => {},
            // STY_dx    => {},

            // Jumps
            // JSR_abs   => {},
            // JMP_abs   => {},
            // JMP_ind   => {},

            // RTI       => {},
            // RTS       => {},

            // Bit tests
            // BIT_z_pg  => {},
            // BIT_abs   => {},

            // ALU operations
            // ORA_inx_x => {},
            // ORA_z_pg  => {},
            // ORA_imm   => {},
            // ORA_abs   => {},
            // ORA_ind_y => {},
            // ORA_dx    => {},
            // ORA_ax    => {},
            // ORA_ay    => {},

            // AND_inx_x => {},
            // AND_z_pg  => {},
            // AND_imm   => {},
            // AND_abs   => {},
            // AND_ind_y => {},
            // AND_dx    => {},
            // AND_ax    => {},
            // AND_ay    => {},

            // EOR_inx_x => {},
            // EOR_z_pg  => {},
            // EOR_imm   => {},
            // EOR_abs   => {},
            // EOR_ind_y => {},
            // EOR_dx    => {},
            // EOR_ax    => {},
            // EOR_ay    => {},

            // ADC_inx_x => {},
            // ADC_z_pg  => {},
            // ADC_imm   => {},
            // ADC_abs   => {},
            // ADC_ind_y => {},
            // ADC_dx    => {},
            // ADC_ax    => {},
            // ADC_ay    => {},

            // CMP_inx_x => {},
            // CMP_z_pg  => {},
            // CMP_imm   => {},
            // CMP_abs   => {},
            // CMP_ind_y => {},
            // CMP_dx    => {},
            // CMP_ax    => {},
            // CMP_ay    => {},

            // SBC_inx_x => {},
            // SBC_z_pg  => {},
            // SBC_imm   => {},
            // SBC_abs   => {},
            // SBC_ind_y => {},
            // SBC_dx    => {},
            // SBC_ax    => {},
            // SBC_ay    => {},
            
            // ASL_z_pg  => {},
            // ASL       => {},
            // ASL_abs   => {},
            // ASL_dx    => {},
            // ASL_ax    => {},

            // LSR_z_pg  => {},
            // LSR       => {},
            // LSR_abs   => {},
            // LSR_dx    => {},
            // LSR_ax    => {},

            // Rotates
            // ROL_z_pg  => {},
            // ROL       => {},
            // ROL_abs   => {},
            // ROL_dx    => {},
            // ROL_ax    => {},

            // ROR_z_pg  => {},
            // ROR       => {},
            // ROR_abs   => {},
            // ROR_dx    => {},
            // ROR_ax    => {},

            // Increments
            // DEC_z_pg  => {},
            // DEC_abs   => {},
            // DEC_dx    => {},
            // DEC_ax    => {},

            // INC_z_pg  => {},
            // INC_abs   => {},
            // INC_dx    => {},
            // INC_ax    => {},

            // The ever important nop
            // Observe all its majesty
            // NOP       => {},
            _ => panic!("Unrecognized instruction: {:#x}", instr),
        }
    }

    // Functions
    fn compare(&mut self, reg_val: u8, comp_val: u8) {
    }

    fn eor(&mut self, val: u8) {
    }

    fn jmp(&mut self, jump_target: u16) {
    }
}

// TODO: Move this to a propper debugger
impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CPU: a:0x{:x} x:0x{:x} y:0x{:x} pc:0x{:x} s:0x{:x} p:0x{:x}",
               self.a, self.x, self.y, self.pc, self.s, self.p)
    }
}
