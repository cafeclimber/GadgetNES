use std::fmt;
use num::FromPrimitive;
use super::apu::Apu;
use super::ppu::Ppu;
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
        match instr {
            // TODO: Implement unofficial opcodes
            // 0x00 => {}, // BRK       

            // Stack    
            // 0x08 => {}, // PHP       
            // 0x28 => {}, // PLP       
            // 0x48 => {}, // PHA       
            // 0x68 => {}, // PLA       
            // 0x9a => {}, // TXS       
            // 0xba => {}, // TSX       

            // Branch   
            // 0x10 => {}, // BPL       
            // 0x30 => {}, // BMI       
            // 0x50 => {}, // BVC       
            // 0x70 => {}, // BVS       
            // 0x90 => {}, // BCC       
            // 0xb0 => {}, // BCS       
            // 0xd0 => {}, // BNE       
            // 0xf0 => {}, // BEQ       

            // Flag instructions
            // 0x18 => {}, // CLC      
            // 0x38 => {}, // SEC      
            // 0x58 => {}, // CLI      
            // 0x78 => {}, // SEI      
            // 0xb8 => {}, // CLV      
            // 0xd8 => {}, // CLD      
            // 0xf8 => {}, // SED      

            // Register instructions
            // 0x88 => {}, // DEY       
            // 0xca => {}, // DEX       
            // 0xe8 => {}, // INX       
            // 0xc8 => {}, // INY       
            // 0xaa => {}, // TAX       
            // 0x8a => {}, // TXA       
            // 0xa8 => {}, // TAY       
            // 0x98 => {}, // TYA       

            // Compares
            // 0xc0 => {}, // CPY_imm  
            // 0xc4 => {}, // CPY_z_pg 
            // 0xcc => {}, // CPY_abs  
            // 0xe0 => {}, // CPX_imm  
            // 0xe4 => {}, // CPX_z_pg 
            // 0xec => {}, // CPX_abs  

            // Loads
            // 0xa1 => {}, // LDA_inx_x 
            // 0xa5 => {}, // LDA_z_pg  
            // 0xa9 => {}, // LDA_imm   
            // 0xad => {}, // LDA_abs   
            // 0xb1 => {}, // LDA_ind_y 
            // 0xb5 => {}, // LDA_dx    
            // 0xbd => {}, // LDA_ax    
            // 0xb9 => {}, // LDA_ay    

            // 0xa2 => {}, // LDX_imm  
            // 0xa6 => {}, // LDX_z_pg 
            // 0xae => {}, // LDX_abs  
            // 0xb6 => {}, // LDX_dy   
            // 0xbe => {}, // LDX_ay   

            // 0xa0 => {}, // LDY_imm  
            // 0xa4 => {}, // LDY_z_pg 
            // 0xac => {}, // LDY_abs  
            // 0xb4 => {}, // LDY_dx   
            // 0xbc => {}, // LDY_ax   

            // Stores
            // 0x81 => {}, // STA_inx_x
            // 0x85 => {}, // STA_z_pg 
            // 0x8d => {}, // STA_abs  
            // 0x91 => {}, // STA_ind_y
            // 0x95 => {}, // STA_dx   
            // 0x9d => {}, // STA_ax   
            // 0x99 => {}, // STA_ay   

            // 0x86 => {}, // STX_z_pg 
            // 0x8e => {}, // STX_abs  
            // 0x96 => {}, // STX_dy   

            // 0x84 => {}, // STY_z_pg 
            // 0x8c => {}, // STY_abs  
            // 0x94 => {}, // STY_dx   

            // Jumps
            // 0x20 => {}, // JSR_abs  
            // 0x4c => {}, // JMP_abs  
            // 0x6c => {}, // JMP_ind  

            // 0x40 => {}, // RTI      
            // 0x60 => {}, // RTS      

            // Bit tests
            // 0x24 => {}, // BIT_z_pg 
            // 0x2c => {}, // BIT_abs  

            // ALU operations
            // 0x01 => {}, // ORA_inx_x
            // 0x05 => {}, // ORA_z_pg 
            // 0x09 => {}, // ORA_imm  
            // 0x0d => {}, // ORA_abs  
            // 0x11 => {}, // ORA_ind_y
            // 0x15 => {}, // ORA_dx   
            // 0x19 => {}, // ORA_ax   
            // 0x1d => {}, // ORA_ay   

            // 0x21 => {}, // AND_inx_x
            // 0x25 => {}, // AND_z_pg 
            // 0x29 => {}, // AND_imm  
            // 0x2d => {}, // AND_abs  
            // 0x31 => {}, // AND_ind_y
            // 0x35 => {}, // AND_dx   
            // 0x39 => {}, // AND_ax   
            // 0x3d => {}, // AND_ay   

            // 0x41 => {}, // EOR_inx_x
            // 0x45 => {}, // EOR_z_pg 
            // 0x49 => {}, // EOR_imm  
            // 0x4d => {}, // EOR_abs  
            // 0x51 => {}, // EOR_ind_y
            // 0x55 => {}, // EOR_dx   
            // 0x59 => {}, // EOR_ax   
            // 0x5d => {}, // EOR_ay   

            // 0x61 => {}, // ADC_inx_x
            // 0x65 => {}, // ADC_z_pg 
            // 0x69 => {}, // ADC_imm  
            // 0x6d => {}, // ADC_abs  
            // 0x71 => {}, // ADC_ind_y
            // 0x75 => {}, // ADC_dx   
            // 0x79 => {}, // ADC_ax   
            // 0x7d => {}, // ADC_ay   

            // 0xc1 => {}, // CMP_inx_x
            // 0xc5 => {}, // CMP_z_pg 
            // 0xc9 => {}, // CMP_imm  
            // 0xcd => {}, // CMP_abs  
            // 0xd1 => {}, // CMP_ind_y
            // 0xd5 => {}, // CMP_dx   
            // 0xd9 => {}, // CMP_ax   
            // 0xdd => {}, // CMP_ay   

            // 0xe1 => {}, // SBC_inx_x
            // 0xe5 => {}, // SBC_z_pg 
            // 0xe9 => {}, // SBC_imm  
            // 0xed => {}, // SBC_abs  
            // 0xf1 => {}, // SBC_ind_y
            // 0xf5 => {}, // SBC_dx   
            // 0xf9 => {}, // SBC_ax   
            // 0xfd => {}, // SBC_ay   
                 
            // 0x06 => {}, // ASL_z_pg 
            // 0x0a => {}, // ASL      
            // 0x0e => {}, // ASL_abs  
            // 0x16 => {}, // ASL_dx   
            // 0x1e => {}, // ASL_ax   

            // 0x46 => {}, // LSR_z_pg 
            // 0x4a => {}, // LSR      
            // 0x4e => {}, // LSR_abs  
            // 0x56 => {}, // LSR_dx   
            // 0x5e => {}, // LSR_ax   

            // Rotates
            // 0x26 => {}, // ROL_z_pg 
            // 0x2a => {}, // ROL      
            // 0x2e => {}, // ROL_abs  
            // 0x36 => {}, // ROL_dx   
            // 0x3e => {}, // ROL_ax   

            // 0x66 => {}, // ROR_z_pg 
            // 0x6a => {}, // ROR      
            // 0x6e => {}, // ROR_abs  
            // 0x76 => {}, // ROR_dx   
            // 0x7e => {}, // ROR_ax   

            // Increments
            // 0xc6 => {}, // DEC_z_pg 
            // 0xce => {}, // DEC_abs  
            // 0xd6 => {}, // DEC_dx   
            // 0xde => {}, // DEC_ax   

            // 0xe6 => {}, // INC_z_pg 
            // 0xee => {}, // INC_abs  
            // 0xf6 => {}, // INC_dx   
            // 0xfe => {, // INC_ax   

            // The ever important nop
            // Observe all its majesty
            // 0xea => {}, // NOP
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

