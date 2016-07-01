use std::fmt;
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

enum BranchOn {
    Plus,
    Minus,
    OverflowClear,
    OverflowSet,
    CarryClear,
    CarrySet,
    NotEqual,
    Equal,
}

#[derive(Debug, Clone, Copy)]
enum CPURegister {
    A,
    X,
    Y,
    Pc,
    S,
    P,
}

// TODO Has Registers trait
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
        if self.p & flag != 0 {true} else {false}
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

    fn get_pc(&self) -> u16 {
        self.pc + 0x8000
    }

    fn increment_pc(&mut self, increment_by: u16) {
        self.pc += increment_by;
    }

    fn read_reg(&self, register: CPURegister) -> u8 {
        match register {
            CPURegister::A => self.a,
            CPURegister::X => self.x,
            CPURegister::Y => self.y,
            CPURegister::S => self.s,
            CPURegister::P => self.p,
            _ => panic!("Attemped to interact with special register: {:?}. Use specific helper methods instead", register),
        }
    }

    fn write_to_reg(&mut self, register: CPURegister, val: u8) {
        match register {
            CPURegister::A => {self.a = val},
            CPURegister::X => {self.x = val},
            CPURegister::Y => {self.y = val},
            CPURegister::S => {self.s = val},
            _ => panic!{"Attempt to write to unsupported register: {:?}", register},
        }
    }

    // IDEA Make each instruction return tuple of status regs to set? to call after match arm? closure perhaps to form u8?
    // TODO: Make load function
    // TODO: Make functions to get operands based on addressing modes
    pub fn run_instr(&mut self, interconnect: &mut Interconnect) {
        let instr = interconnect.read_byte(self.get_pc());
        println!("instr: {:#x}", instr);
        match instr {
            // TODO: Implement unofficial opcodes
            // 0x00 => {}, // BRK       

            // Stack    
            // 0x08 => {}, // PHP       
            // 0x28 => {}, // PLP       
            // 0x48 => {}, // PHA       
            // 0x68 => {}, // PLA       
            0x9a => {self.transfer(CPURegister::X, CPURegister::S); self.increment_pc(1);}, // TXS       
            // 0xba => {}, // TSX       

            // Branch   
            0x10 => {let branch_target = interconnect.read_byte(self.get_pc() + 1);
                     self.branch(BranchOn::Plus, branch_target);}, // BPL       
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
            0x78 => {self.set_flag(INTERUPT_FLAG); self.increment_pc(1)}, // SEI      
            // 0xb8 => {}, // CLV      
            0xd8 => {self.unset_flag(DECIMAL_FLAG); self.increment_pc(1)}, // CLD      
            // 0xf8 => {}, // SED      

            // Register instructions
            // 0x88 => {}, // DEY       
            0xca => {self.decrement(CPURegister::X); self.increment_pc(1)}, // DEX       
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
            0xa9 => {let val = self.immediate(interconnect);
                     self.load(CPURegister::A, val);
                     self.increment_pc(2);}, // LDA_imm   
            0xad => {let val = self.absolute(interconnect);
                     self.load(CPURegister::A, val);
                     self.increment_pc(3);}, // LDA_abs   
            // 0xb1 => {}, // LDA_ind_y 
            // 0xb5 => {}, // LDA_dx    
            // 0xbd => {}, // LDA_ax    
            // 0xb9 => {}, // LDA_ay    

            0xa2 => {let val = self.immediate(interconnect);
                     self.load(CPURegister::X, val);
                     self.increment_pc(2);}, // LDX_imm  
            0xa6 => {let val = self.zero_page(interconnect);
                     self.load(CPURegister::X, val);
                     self.increment_pc(2);}, // LDX_z_pg 
            // 0xae => {}, // LDX_abs  
            // 0xb6 => {}, // LDX_dy   
            // 0xbe => {}, // LDX_ay   

            0xa0 => {let val = self.immediate(interconnect);
                     self.load(CPURegister::Y, val);
                     self.increment_pc(2);}, // LDY_imm  
            // 0xa4 => {}, // LDY_z_pg 
            // 0xac => {}, // LDY_abs  
            // 0xb4 => {}, // LDY_dx   
            // 0xbc => {}, // LDY_ax   

            // Stores
            // 0x81 => {}, // STA_inx_x
            // 0x85 => {}, // STA_z_pg 
            // 0x8d => {}, // STA_abs  
            //0x91 => {}, // STA_ind_y
            // 0x95 => {}, // STA_dx   
            // 0x9d => {}, // STA_ax   
            // 0x99 => {}, // STA_ay   

            // 0x86 => {}, // STX_z_pg 
            0x8e => {let addr = interconnect.read_word(self.get_pc() + 1);
                     self.store(interconnect, addr, CPURegister::X);
                     self.increment_pc(3);}, // STX_abs  
            // 0x96 => {}, // STX_dy   

            0x84 => {let temp_addr = interconnect.read_byte(self.get_pc() + 1);
                     let addr = interconnect.read_word(temp_addr as u16);
                     self.store(interconnect, addr, CPURegister::Y);
                     self.increment_pc(2);}, // STY_z_pg 
            // 0x8c => {}, // STY_abs  
            // 0x94 => {}, // STY_dx   

            // Jumps
            // 0x20 => {}, // JSR_abs  
            0x4c => {let target_addr = interconnect.read_word(self.get_pc() + 1);
                     self.jmp(target_addr);}, // JMP_abs  
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

    // Addressing modes
    fn immediate(&self, interconnect: &Interconnect) -> u8 {interconnect.read_byte(self.get_pc() + 1)}

    fn absolute(&self, interconnect: &Interconnect) -> u8 {
        let addr = interconnect.read_word(self.get_pc() + 1);
        interconnect.read_byte(addr)
    }

    fn zero_page(&self, interconnect: &Interconnect) -> u8 {
        let addr = interconnect.read_byte(self.get_pc() + 1);
        interconnect.read_byte(addr as u16)
    }

    /// Should only take CPURegister::X or CPURegister::Y as an argument
    fn absolute_indexed(&self, interconnect: &Interconnect, register: CPURegister) -> u8 {
        let addr = interconnect.read_word(self.get_pc() + 1);
        let sum_addr = addr + (self.read_reg(register) as u16);
        interconnect.read_byte(sum_addr)
    }

    fn z_page_indexed(&self, interconnect: &Interconnect, register: CPURegister) -> u8 {
        let addr = interconnect.read_byte(self.get_pc() + 1);
        let sum_addr = addr + self.read_reg(register);
        interconnect.read_byte(sum_addr as u16)
    }

    fn indexed_indirect(&self, interconnect: &Interconnect) -> u8 {
        let addr = interconnect.read_byte(self.get_pc() + 1);
        let sum_addr = addr + self.read_reg(CPURegister::X);
        let addr = interconnect.read_word(sum_addr as u16);
        interconnect.read_byte(addr as u16)
    }

    fn indirect_indexed(&self, interconnect: &Interconnect) -> u8 {
        let mut addr = interconnect.read_word(self.get_pc() + 1);
        addr += self.read_reg(CPURegister::Y) as u16;
        interconnect.read_byte(addr)
    }

    // Instruction abstractions
    fn compare(&mut self, reg_val: u8, comp_val: u8) {
    }

    fn eor(&mut self, val: u8) {
    }

    fn jmp(&mut self, jump_target: u16) {
        self.pc = jump_target;
    }

    fn transfer(&mut self, from_reg: CPURegister, to_reg: CPURegister) {
        let val = self.read_reg(from_reg);
        self.write_to_reg(to_reg, val);
        if self.read_reg(to_reg) & 0b1000_0000 != 0 {self.set_flag(NEGATIVE_FLAG)};
        if self.read_reg(to_reg) == 0 {self.set_flag(ZERO_FLAG)};
    }

    fn load(&mut self, register: CPURegister, val: u8) {
        self.write_to_reg(register, val);
        if self.read_reg(register) & 0b1000_0000 != 0 {self.set_flag(NEGATIVE_FLAG)};
        if self.read_reg(register) == 0 {self.set_flag(ZERO_FLAG)};
    }

    fn store(&mut self, interconnect: &mut Interconnect, addr: u16, register: CPURegister) {
        let val = self.read_reg(register);
        interconnect.write_byte(addr, val);
    }

    fn branch(&mut self, branch_on: BranchOn, branch_target: u8) {
        match branch_on {
            BranchOn::Plus => {if self.check_flag(NEGATIVE_FLAG) {self.increment_pc(2)} else {self.pc = ((self.pc as i32) + (branch_target as i32)) as u16}},
            BranchOn::Minus => {},
            BranchOn::OverflowClear => {},
            BranchOn::OverflowSet => {},
            BranchOn::CarryClear => {},
            BranchOn::CarrySet => {},
            BranchOn::NotEqual => {},
            BranchOn::Equal => {},
        }
    }

    fn decrement(&mut self, register: CPURegister) {
        let val = self.read_reg(register);
        val.wrapping_sub(1);
        if self.read_reg(register) & 0b1000_0000 != 0 {self.set_flag(NEGATIVE_FLAG)};
        if self.read_reg(register) == 0 {self.set_flag(ZERO_FLAG)};
    }
}

// TODO: Move this to a propper debugger
impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CPU: a:0x{:x} x:0x{:x} y:0x{:x} pc:0x{:x} s:0x{:x} p:0x{:x}",
               self.a, self.x, self.y, self.pc, self.s, self.p)
    }
}

