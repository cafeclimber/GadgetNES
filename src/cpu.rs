use std::fmt;
use num::FromPrimitive;
use super::apu::Apu;
use super::instruction::Instruction;
use super::mem::{Memory, AddressingMode};

const RAM_SIZE: usize = 0x800;

#[derive(Default)]
pub struct Cpu {
    a: u8, // Accumulator

    x: u8, // x-Index
    y: u8, // y-index

    pub pc: u16, // Program counter
    
    s: u8, // Stack pointer

    p: u8, // Status register

    // Because all instructions are first run by the cpu,
    // it is easiest to let it own both the APU, the PPU,
    // and the cartridge
    apu: Apu,
    // ppu: Ppu,

    memory: Memory,
}

const NEGATIVE_FLAG:u8 = 1 << 7;
const OVERFLOW_FLAG:u8 = 1 << 6;
const IRQ_FLAG:     u8 = 1 << 4;
const DECIMAL_FLAG: u8 = 1 << 3;
const INTERUPT_FLAG:u8 = 1 << 2;
const ZERO_FLAG:    u8 = 1 << 1;
const CARRY_FLAG:   u8 = 1 << 0;

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            a: 0,

            x: 0,
            y: 0,

            pc: 0x10, // TODO make this a result of header information

            s: 0,

            p: 0,

            apu: Apu::default(),

            memory: Memory::default(),
        } 
    }

    pub fn power_up(&mut self, cart_rom: Vec<u8>) {
        self.p = 0x34;
        self.s = 0xfd;

        self.memory.load_cartridge(cart_rom);
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


    pub fn read_instr(&self) -> Instruction {
        let raw_instr = self.memory.read_instr(self.pc);
        Instruction::from_u8(raw_instr).unwrap_or_else(|| {
            panic!("Unrecognized instruction: {:#x}", raw_instr)
        })
    }

    // TODO implement memory map instead of raw access?
    pub fn run_instr(&mut self, instr: Instruction) {
        use super::instruction::Instruction::*;
        use mem::AddressingMode as AM;
        println!("CPU STATE: {:?}", self);
        println!("INSTR: {:?}", instr);
        match instr {
            // TODO: Implement unofficial opcodes
            // BRK       => {},

            // Stack    
            PHP       => { let status = self.p; self.pushs(status); self.pc += 1 },
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
            SEI       => {self.set_flag(INTERUPT_FLAG); self.pc += 1},
            // CLV       => {},
            CLD       => {self.unset_flag(DECIMAL_FLAG); self.pc += 1},
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
            CPY_imm   => { let imm = self.memory.read_mem(AM::Immediate(self.pc));
                           let y = self.y; self.compare(y, imm);
                           self.pc += 2}
            // CPY_z_pg  => {},
            // CPY_abs   => {},
            // CPX_imm   => {},
            // CPX_z_pg  => {},
            // CPX_abs   => {},

            // Loads
            // LDA_inx_x => {},
            // LDA_z_pg  => {},
            // LDA_imm   => {},
            LDA_abs   => {self.a = self.memory.read_mem(AM::Absolute(self.pc)); self.pc += 2},
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
            // JMP_abs   => {self.pc = self.cart.read_rom_word((self.pc + 1) as usize)},
            // JMP_ind   => {},

            // RTI       => {},
            // RTS       => {self.pc = self.pc + 1},

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
            EOR_z_pg  => {let val = self.memory.read_mem(AM::ZeroPage(self.pc));
                          self.eor(val);
                          self.pc += 2},
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
        }
    }
    // Functions
    fn compare(&mut self, reg_val: u8, comp_val: u8) {
        println!("reg_val: {} comp_val {}", reg_val, comp_val);
        let comparison = reg_val as i16 - comp_val as i16;
        if comparison & 0x100 == 0 {self.set_flag(CARRY_FLAG)} else {self.unset_flag(CARRY_FLAG)};
    }

    fn eor(&mut self, val: u8) {
        self.a = self.a ^ val;
        if (self.a as i8) < 0 {self.set_flag(NEGATIVE_FLAG)} else {self.unset_flag(NEGATIVE_FLAG)};
        if self.a == 0 {self.set_flag(ZERO_FLAG)} else {self.unset_flag(ZERO_FLAG)};
    }
}

// TODO: Move this to a propper debugger
impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CPU: a:0x{:x} x:0x{:x} y:0x{:x} pc:0x{:x} s:0x{:x} p:0x{:x}",
               self.a, self.x, self.y, self.pc, self.s, self.p)
    }
}
