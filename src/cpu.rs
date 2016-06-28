use std::fmt;
use num::FromPrimitive;
use super::apu::Apu;
use super::ppu::Ppu;
use super::instruction::Instruction;
use super::mem::Memory;

const NEGATIVE_FLAG:u8 = 1 << 7;
const OVERFLOW_FLAG:u8 = 1 << 6;
const IRQ_FLAG:     u8 = 1 << 4;
const DECIMAL_FLAG: u8 = 1 << 3;
const INTERUPT_FLAG:u8 = 1 << 2;
const ZERO_FLAG:    u8 = 1 << 1;
const CARRY_FLAG:   u8 = 1 << 0;

// TODO: Put this somewhere else?
/* ==== Memory Map ===== */
const RAM_BEG: u16 = 0x0000;
const RAM_SIZE: usize = 0x0800;
const RAM_END: u16 = 0x07ff;

const RAM_MIRROR_ONE_BEG: u16 = 0x0800;
const RAM_MIRROR_ONE_SIZE: u16 = 0x0800;
const RAM_MIRROR_ONE_END: u16 = 0x0fff;

const RAM_MIRROR_TWO_BEG: u16 = 0x1000;
const RAM_MIRROR_TWO_SIZE: u16 = 0x0800;
const RAM_MIRROR_TWO_END: u16 = 0x17ff;

const RAM_MIRROR_THREE_BEG: u16 = 0x1800;
const RAM_MIRROR_THREE_SIZE: u16 = 0x0800;
const RAM_MIRROR_THREE_END: u16 = 0x1fff;

const PPU_REGS_BEG: u16 = 0x2000;
const PPU_REGS_SIZE: u16 = 0x0008;
const PPU_REGS_END: u16 = 0x2007;

const PPU_MIRRORS_BEG: u16 = 0x2008;
const PPU_MIRRORS_SIZE: u16 = 0x1ff8;
const PPU_MIRRORS_END: u16 = 0x3fff;

const APU_REGS_BEG: u16 = 0x4000;
const APU_REGS_SIZE: u16 = 0x0020;
const APU_REGS_END: u16 = 0x401f;

const CARTRIDGE_SPACE_BEG: u16 = 0x4020;
const CARTRIDGE_SPACE_SIZE: u16 = 0xBFE0;
const CARTRIDGE_SPACE_END: u16 = 0xffff;


#[derive(Default)]
pub struct Cpu {
    a: u8, // Accumulator

    x: u8, // x-Index
    y: u8, // y-index

    pub pc: u16, // Program counter
    
    s: u8, // Stack pointer

    p: u8, // Status register

    /* Because instructions are all handled by the 
       CPU, which then gives commands to the APU, 
       and the PPU, it's easiest if it owns both. */
    apu: Apu,
    ppu: Ppu,

    memory: Memory,
}

pub enum AddressingMode {
    Absolute(u16),
    Immediate(u16),
    ZeroPage(u16),
    Relative(u16),
    AbsX(u16),
    AbsY(u16),
    ZPageX(u16),
    ZPageY(u16),
    IndexedIndirect(u16),
    IndirectIndexed(u16),
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

            apu: Apu::default(),
            ppu: Ppu::default(),

            memory: Memory::default(),
        } 
    }

    pub fn power_up(&mut self, cart_rom: Vec<u8>) {
        self.p = 0x34;
        self.s = 0xfd;

        self.memory.load_cartridge(cart_rom);
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

    fn map_mem(&self, addr: u16) -> u8 {
        match addr {
            RAM_BEG ... RAM_END => {self.memory.read_ram(addr)},
            RAM_MIRROR_ONE_BEG ... RAM_MIRROR_ONE_END => {self.memory.read_ram(addr - 0x800)},
            RAM_MIRROR_TWO_BEG ... RAM_MIRROR_TWO_END => {self.memory.read_ram(addr - 2*0x800)},
            RAM_MIRROR_THREE_BEG ... RAM_MIRROR_THREE_END => {self.memory.read_ram(addr - 3*0x800)},
            PPU_REGS_BEG ... PPU_REGS_END => {self.ppu.read_reg(addr)}
            PPU_MIRRORS_BEG ... PPU_MIRRORS_END => {self.ppu.read_reg(addr)}
            APU_REGS_BEG ... APU_REGS_END => {self.apu.read_reg(addr)},
            CARTRIDGE_SPACE_BEG ... CARTRIDGE_SPACE_END => {self.memory.rom_byte(addr)},
            _ => panic!("Unrecognized virtual address: {:#x}", addr),
        }
    }

    // TODO: Double check these are correct
    pub fn read_mem(&self, addressing_mode: AddressingMode) -> u8 {
        use self::AddressingMode::*;
        match addressing_mode {
            Absolute(pc)      => {self.map_mem(self.memory.rom_word(pc+1))},
            Immediate(pc)     => {self.memory.rom_byte(pc+1)}
            ZeroPage(pc)      => {self.map_mem((0x0000 | self.memory.rom_byte(pc+1) as u16))},
            Relative(pc)      => {self.map_mem((pc + self.memory.rom_byte(pc+1) as u16))},
            AbsX(pc)   => {self.map_mem((self.x as u16 + self.memory.rom_word(pc+1)))},
            AbsY(pc)   => {self.map_mem((self.y as u16 + self.memory.rom_word(pc+1)))},
            ZPageX(pc) => {self.map_mem(((self.x + self.memory.rom_byte(pc+1)) as u16))},
            ZPageY(pc) => {self.map_mem(((self.y + (self.memory.rom_byte(pc+1))) as u16))},
            IndexedIndirect(pc) => {self.map_mem((self.memory.rom_word((self.memory.rom_byte(pc+1) + self.x) as u16)))},
            IndirectIndexed(pc) => {self.map_mem((self.memory.rom_word(((self.memory.rom_byte((pc+1) as u16)) + self.y) as u16)))},
        }
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
        use self::AddressingMode as AM;
        println!("CPU STATE: {:?}", self);
        println!("INSTR: {:?}", instr);
        match instr {
            // TODO: Implement unofficial opcodes
            // BRK       => {},

            // Stack    
            PHP       => {let status = self.p; self.pushs(status); self.pc += 1;},
            // PLP       => {},
            // PHA       => {},
            // PLA       => {},
            // TXS       => {},
            // TSX       => {},

            // Branch   
            BPL       => {if self.check_flag(NEGATIVE_FLAG) == true {branch(self.relative(pc+1))} // FIXME
                          else {self.pc += 2;}},
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
            CPY_imm   => { let imm = self.read_mem(AM::Immediate(self.pc));
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
            LDA_abs   => {self.a = self.read_mem(AM::Absolute(self.pc)); self.pc += 3},
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
            JSR_abs   => {let pc_l = (self.pc+2 & 0b11111111) as u8;
                          let pc_h = ((self.pc+2 & 0b1111111100000000) >> 8) as u8;
                          self.pushs(pc_l); self.pushs(pc_h);
                          let jump_target = self.memory.rom_word(self.pc+1);
                          self.jmp(jump_target)},
            JMP_abs   => {let jump_target = self.memory.rom_word(self.pc+1);
                          self.jmp(jump_target)},
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
            EOR_z_pg  => {let val = self.read_mem(AM::ZeroPage(self.pc));
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

    fn jmp(&mut self, jump_target: u16) {
        self.pc = jump_target - 0x8000; // Uhhh memory mappers....
    }
}


// TODO: Move this to a propper debugger
impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CPU: a:0x{:x} x:0x{:x} y:0x{:x} pc:0x{:x} s:0x{:x} p:0x{:x}",
               self.a, self.x, self.y, self.pc, self.s, self.p)
    }
}
