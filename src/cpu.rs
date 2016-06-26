use std::fmt;
use super::apu::Apu;
use super::cart::Cartridge;
use num::FromPrimitive;
use super::instruction::Instruction;

const RAM_SIZE: usize = 2048;

#[derive(Default)]
pub struct Cpu {
    a: u8, // Accumulator

    x: u8, // x-Index
    y: u8, // y-index

    pub pc: u16, // Program counter
    
    s: u8, // Stack pointer

    p: u8, // Status register

    ram: Box<[u8]>, // RAM

    // Because all instructions are first run by the cpu,
    // it is easiest to let it own both the APU and the PPU
    apu: Apu,
    // ppu: Ppu,

    cart: Cartridge,
}

impl Cpu {
    pub fn new() -> Cpu{
        Cpu {
            a: 0,

            x: 0,
            y: 0,

            pc: 0,

            s: 0,

            p: 0,

            ram: vec![0u8; RAM_SIZE].into_boxed_slice(),

            apu: Apu::default(),

            cart: Cartridge::default(),
        } 
    }
    pub fn power_up(&mut self) {
        self.p = 0x34;
        self.s = 0xfd;
    }

    pub fn read_instr(&self) -> Instruction {
        let raw_instr = self.cart.read_rom(self.pc as usize);
        Instruction::from_u8(raw_instr).unwrap_or_else(|| {
            panic!("Unrecognized instruction: {:?}", raw_instr)
        })
    }

    pub fn run_instr(&mut self, instr: Instruction) {
        use super::instruction::Instruction::*;
        match instr {
            // TODO: Implement unofficial opcodes
            BRK       => {},

            // Stack    
            PHP       => {},
            PLP       => {},
            PHA       => {},
            PLA       => {},
            TXS       => {},
            TSX       => {},

            // Branch   
            BPL       => {},
            BMI       => {},
            BVC       => {},
            BVS       => {},
            BCC       => {},
            BCS       => {},
            BNE       => {},
            BEQ       => {},

            // Flag instructions
            CLC       => {},
            SEC       => {},
            CLI       => {},
            SEI       => {},
            CLV       => {},
            CLD       => {},
            SED       => {},

            // Register instructions
            DEY       => {},
            DEX       => {},
            INX       => {},
            INY       => {},
            TAX       => {},
            TXA       => {},
            TAY       => {},
            TYA       => {},

            // Compares
            CPY_imm   => {},
            CPY_z_pg  => {},
            CPY_abs   => {},
            CPX_imm   => {},
            CPX_z_pg  => {},
            CPX_abs   => {},

            // Loads
            LDA_inx_x => {},
            LDA_z_pg  => {},
            LDA_imm   => {},
            LDA_abs   => {},
            LDA_ind_y => {},
            LDA_dx    => {},
            LDA_ax    => {},
            LDA_ay    => {},

            LDX_imm   => {},
            LDX_z_pg  => {},
            LDX_abs   => {},
            LDX_dy    => {},
            LDX_ay    => {},

            LDY_imm   => {},
            LDY_z_pg  => {},
            LDY_abs   => {},
            LDY_dx    => {},
            LDY_ax    => {},

            // Stores
            STA_inx_x => {},
            STA_z_pg  => {},
            STA_abs   => {},
            STA_ind_y => {},
            STA_dx    => {},
            STA_ax    => {},
            STA_ay    => {},

            STX_z_pg  => {},
            STX_abs   => {},
            STX_dy    => {},

            STY_z_pg  => {},
            STY_abs   => {},
            STY_dx    => {},

            // Jumps
            JSR_abs   => {},
            JMP_abs   => {},
            JMP_ind   => {},

            RTI       => {},
            RTS       => {},

            // Bit tests
            BIT_z_pg  => {},
            BIT_abs   => {},

            // ALU operations
            ORA_inx_x => {},
            ORA_z_pg  => {},
            ORA_imm   => {},
            ORA_abs   => {},
            ORA_ind_y => {},
            ORA_dx    => {},
            ORA_ax    => {},
            ORA_ay    => {},

            AND_inx_x => {},
            AND_z_pg  => {},
            AND_imm   => {},
            AND_abs   => {},
            AND_ind_y => {},
            AND_dx    => {},
            AND_ax    => {},
            AND_ay    => {},

            EOR_inx_x => {},
            EOR_z_pg  => {},
            EOR_imm   => {},
            EOR_abs   => {},
            EOR_ind_y => {},
            EOR_dx    => {},
            EOR_ax    => {},
            EOR_ay    => {},

            ADC_inx_x => {},
            ADC_z_pg  => {},
            ADC_imm   => {},
            ADC_abs   => {},
            ADC_ind_y => {},
            ADC_dx    => {},
            ADC_ax    => {},
            ADC_ay    => {},

            CMP_inx_x => {},
            CMP_z_pg  => {},
            CMP_imm   => {},
            CMP_abs   => {},
            CMP_ind_y => {},
            CMP_dx    => {},
            CMP_ax    => {},
            CMP_ay    => {},

            SBC_inx_x => {},
            SBC_z_pg  => {},
            SBC_imm   => {},
            SBC_abs   => {},
            SBC_ind_y => {},
            SBC_dx    => {},
            SBC_ax    => {},
            SBC_ay    => {},
            
            ASL_z_pg  => {},
            ASL       => {},
            ASL_abs   => {},
            ASL_dx    => {},
            ASL_ax    => {},

            LSR_z_pg  => {},
            LSR       => {},
            LSR_abs   => {},
            LSR_dx    => {},
            LSR_ax    => {},

            // Rotates
            ROL_z_pg  => {},
            ROL       => {},
            ROL_abs   => {},
            ROL_dx    => {},
            ROL_ax    => {},

            ROR_z_pg  => {},
            ROR       => {},
            ROR_abs   => {},
            ROR_dx    => {},
            ROR_ax    => {},

            // Increments
            DEC_z_pg  => {},
            DEC_abs   => {},
            DEC_dx    => {},
            DEC_ax    => {},

            INC_z_pg  => {},
            INC_abs   => {},
            INC_dx    => {},
            INC_ax    => {},

            // The ever important nop
            // Observe all its majesty
            NOP       => {},
        }
    }
}

// TODO: Move this to a propper debugger
impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CPU: a:0x{:x} x:0x{:x} y:0x{:x} pc:0x{:x} s:0x{:x} p:0x{:x}",
               self.a, self.x, self.y, self.pc, self.s, self.p)
    }
}
