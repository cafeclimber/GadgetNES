use std::fmt;
use super::interconnect::Interconnect;
use super::instructions::Instruction;

// PRETTIFYME: Make an enum?
const NEGATIVE_FLAG:  u8 = 1 << 7;
const OVERFLOW_FLAG:  u8 = 1 << 6;
const STACK_COPY:     u8 = 1 << 5;
const BRK_FLAG:       u8 = 1 << 4;
const DECIMAL_FLAG:   u8 = 1 << 3;
const IRQ_FLAG:       u8 = 1 << 2;
const ZERO_FLAG:      u8 = 1 << 1;
const CARRY_FLAG:     u8 = 1 << 0;

const NMI_VECTOR: u16 = 0xfffa;
const RESET_VECTOR: u16 = 0xfffc;
const IRQBRK_VECTOR: u16 = 0xfffe;

#[derive(Default)]
pub struct Cpu {
    a: u8, // Accumulator

    x: u8, // x-Index
    y: u8, // y-index

    pc: u16, // Program counter
    
    stack_pointer: u8, // Stack pointer

    status: u8, // Status register
}

/*#[derive(Debug)]
enum AddressingMode {
    Accumulator,
    Implied,
    Immediate,
    Absolute,
    ZeroPage,
    Relative,
    AbsoluteIndexed(CPURegister),
    ZeroPageIndexed(CPURegister),
    IndexedIndirect,
    IndirectIndexed,
}*/

#[derive(Debug)]
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

#[derive(Debug, Clone, Copy, PartialEq)]
enum CPURegister {
    A,
    X,
    Y,
    StackPointer,
    Status,
}

// TODO Has Registers trait
impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            a: 0,

            x: 0,
            y: 0,

            pc: 0, // TODO make this a result of header information

            stack_pointer: 0,

            status: 0,
        } 
    }

    pub fn power_up(&mut self) {
        self.status = 0x24;
        self.stack_pointer = 0xfd;
    }

    fn check_flag(&mut self, flag: u8) -> bool {
        self.read_reg(CPURegister::Status) & flag != 0
    }
    
    fn set_flag(&mut self, flag: u8) {
        self.status = self.status | flag;
    }

    fn unset_flag(&mut self, flag: u8) {
        self.status = self.status & !flag;
    }

    fn push_byte_stack(&mut self, interconnect: &mut Interconnect, value: u8) {
        let addr = self.read_reg(CPURegister::StackPointer) as u16 + 0x100;
        interconnect.write_byte(addr, value);
        self.stack_pointer -= 1;
    }

    fn pull_byte_stack(&mut self, interconnect: &Interconnect, register: CPURegister) {
        self.stack_pointer += 1;
        let addr = self.read_reg(CPURegister::StackPointer) as u16 + 0x100;
        self.write_to_reg(register, interconnect.read_byte(addr));
        self.set_flag(STACK_COPY);
    }

    fn push_return_addr(&mut self, interconnect: &mut Interconnect) {
        let addr = (self.read_reg(CPURegister::StackPointer) as u16) + 0x100;
        interconnect.write_word(addr, self.pc + 2);
        self.stack_pointer -= 2;
    }

    fn pull_return_addr(&mut self, interconnect: &mut Interconnect) -> u16 {
        self.stack_pointer += 2;
        let addr = self.read_reg(CPURegister::StackPointer) as u16 + 0x100;
        let ret_addr = interconnect.read_word(addr) + 1;
        ret_addr
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
            CPURegister::StackPointer => self.stack_pointer,
            CPURegister::Status => self.status,
        }
    }

    fn write_to_reg(&mut self, register: CPURegister, val: u8) {
        match register {
            CPURegister::A => {self.a = val},
            CPURegister::X => {self.x = val},
            CPURegister::Y => {self.y = val},
            CPURegister::StackPointer => {self.stack_pointer = val},
            CPURegister::Status => {self.status = val},
        }
    }

    // IDEA: Make each instruction return tuple of status regs to set? to call after match arm? closure perhaps to form u8?
    // TODO: Fetch instruction, then run it. Should make printing better
    // TODO: Compare with logfile.
    // TODO: Cycles
    pub fn run_instr(&mut self, interconnect: &mut Interconnect) {
        use enum_primitive::FromPrimitive;
        use instructions::Instruction::*;
        let raw_instr = interconnect.read_byte(self.get_pc());
        let instr = Instruction::from_u8(raw_instr).unwrap_or_else(|| {
            panic!("Unrecognized instruction: {:#x} Last Failure code: (02h): {:x} (03h): {:x}", raw_instr, interconnect.read_byte(0x02), interconnect.read_byte(0x03));
        });
        println!("{:X} {:?} \t A:{:2X} X:{:2X} Y:{:2X} P:{:2X} SP:{:2X}", self.pc+0x8000, instr, self.a, self.x, self.y, self.status, self.stack_pointer);
        match instr {
            // TODO: Implement unofficial opcodes

            // FIXME: ...wat
            /*BRK => {let ret_addr = self.get_pc();
                     self.push_return_addr(interconnect, ret_addr);
                     let flags = self.read_reg(CPURegister::StackPointer);
                     self.push_byte_stack(interconnect, flags);
                     self.set_flag(IRQ_FLAG);
                     let jmp_addr = interconnect.read_word(IRQBRK_VECTOR);
                     self.jmp(jmp_addr);}, // BRK*/

            // Stack    
            PHP => {let val = self.read_reg(CPURegister::Status); self.push_byte_stack(interconnect, val); self.set_flag(BRK_FLAG); self.increment_pc(1);},
            PLP => {self.pull_byte_stack(interconnect, CPURegister::Status); self.increment_pc(1);},
            PHA => {let val = self.read_reg(CPURegister::A); self.push_byte_stack(interconnect, val); self.increment_pc(1);},
            PLA => {self.pull_byte_stack(interconnect, CPURegister::A);
                    self.increment_pc(1);
                    if self.read_reg(CPURegister::A) & 0b1000_0000 != 0 {self.set_flag(NEGATIVE_FLAG);} else {self.unset_flag(NEGATIVE_FLAG);};
                    if self.read_reg(CPURegister::A) == 0 {self.set_flag(ZERO_FLAG);} else {self.unset_flag(ZERO_FLAG);};}
            TXS => {self.transfer(CPURegister::X, CPURegister::StackPointer); self.increment_pc(1);},
            TSX => {self.transfer(CPURegister::StackPointer, CPURegister::X); self.increment_pc(1);},

            // Branch   
            BPL => {self.branch(interconnect, BranchOn::Plus);},
            BMI => {self.branch(interconnect, BranchOn::Minus)},
            BVC => {self.branch(interconnect, BranchOn::OverflowClear);},
            BVS => {self.branch(interconnect, BranchOn::OverflowSet);},
            BCC => {self.branch(interconnect, BranchOn::CarryClear);},
            BCS => {self.branch(interconnect, BranchOn::CarrySet);},
            BNE => {self.branch(interconnect, BranchOn::NotEqual);},
            BEQ => {self.branch(interconnect, BranchOn::Equal);},

            // Flag instructions
            CLC => {self.unset_flag(CARRY_FLAG); self.increment_pc(1);},
            SEC => {self.set_flag(CARRY_FLAG); self.increment_pc(1);},
            // CLI => {},
            SEI => {self.set_flag(IRQ_FLAG); self.increment_pc(1);},
            CLV => {self.unset_flag(OVERFLOW_FLAG); self.increment_pc(1)},
            CLD => {self.unset_flag(DECIMAL_FLAG); self.increment_pc(1);},
            SED => {self.set_flag(DECIMAL_FLAG); self.increment_pc(1);},

            // Register instructions
            DEY => {self.decrement(CPURegister::Y); self.increment_pc(1);},
            DEX => {self.decrement(CPURegister::X); self.increment_pc(1);},
            INX => {self.increment(CPURegister::X); self.increment_pc(1);},
            INY => {self.increment(CPURegister::Y); self.increment_pc(1);},
            TAX => {self.transfer(CPURegister::A, CPURegister::X); self.increment_pc(1);},
            TXA => {self.transfer(CPURegister::X, CPURegister::A); self.increment_pc(1);},
            TAY => {self.transfer(CPURegister::A, CPURegister::Y); self.increment_pc(1);},
            TYA => {self.transfer(CPURegister::Y, CPURegister::A); self.increment_pc(1);},

            // Compares
            CPYImm => {let val = self.immediate(interconnect); self.compare(CPURegister::Y, val); self.increment_pc(2);}, 
            // CPY_z_pg=> {}, 
            // CPY_abs => {}, 
            CPXImm => {let val = self.immediate(interconnect); self.compare(CPURegister::X, val); self.increment_pc(2);}, 
            // CPX_z_pg=> {}, 
            // CPX_abs => {}, 

            // Loads
            // LDA_inx_x=> {}, 
            LDAZpg => {let val = self.zero_page(interconnect);
                       self.load(CPURegister::A, val);
                       self.increment_pc(2);}, 
            LDAImm => {let val = self.immediate(interconnect);
                       self.load(CPURegister::A, val);
                       self.increment_pc(2);}, 
            LDAAbs => {let val = self.absolute(interconnect);
                       self.load(CPURegister::A, val);
                       self.increment_pc(3);}, 
            LDAIndY => {let val = self.indirect_indexed(interconnect);
                        self.load(CPURegister::A, val);
                        self.increment_pc(2);}, 
            // LDA_dx   => {}, 
            // LDA_ax   => {}, 
            // LDA_ay   => {}, 

            LDXImm => {let val = self.immediate(interconnect);
                       self.load(CPURegister::X, val);
                       self.increment_pc(2);},
            LDXZpg => {let val = self.zero_page(interconnect);
                       self.load(CPURegister::X, val);
                       self.increment_pc(2);},
            LDXAbs => {let val = self.absolute(interconnect);
                       self.load(CPURegister::X, val);
                       self.increment_pc(3);}, 
            // LDX_dy  => {}, 
            // LDX_ay  => {}, 

            LDYImm => {let val = self.immediate(interconnect);
                       self.load(CPURegister::Y, val);
                       self.increment_pc(2);},
            // LDY_z_pg=> {}, 
            // LDY_abs => {}, 
            // LDY_dx  => {}, 
            // LDY_ax  => {}, 

            // Stores
            // STA_inx_=> {}, 
            STAZpg => {let addr = interconnect.read_byte(self.get_pc() + 1);
                       self.store(interconnect, addr as u16, CPURegister::A);
                       self.increment_pc(2);},
            STAAbs => {let addr = interconnect.read_word(self.get_pc() + 1);
                       self.store(interconnect, addr, CPURegister::A);
                       self.increment_pc(3);},
            STAIndY => {let addr = interconnect.read_byte(self.get_pc() + 1);
                        let sum_addr = addr + self.read_reg(CPURegister::X);
                        let addr = interconnect.read_word(sum_addr as u16);
                        self.store(interconnect, addr, CPURegister::A);
                        self.increment_pc(2);},
            // STA_dx  => {}, 
            // STA_ax  => {}, 
            // STA_ay  => {}, 

            STXZpg => {let addr = interconnect.read_byte(self.get_pc() + 1);
                       self.store(interconnect, addr as u16, CPURegister::X);
                       self.increment_pc(2);},
            STXAbs => {let addr = interconnect.read_word(self.get_pc() + 1);
                       self.store(interconnect, addr, CPURegister::X);
                       self.increment_pc(3);},
            // STX_dy => {}, 
            STYZpg => {let temp_addr = interconnect.read_byte(self.get_pc() + 1);
                       let addr = interconnect.read_word(temp_addr as u16);
                       self.store(interconnect, addr, CPURegister::Y);
                       self.increment_pc(2);},
            // STY_abs => {}, 
            // STY_dx  => {}, 

            // Jumps
            JSRAbs => {self.push_return_addr(interconnect);
                       let addr = interconnect.read_word(self.get_pc() + 1);
                       self.jmp(addr - 0x8000);},
            JMPAbs => {let target_addr = interconnect.read_word(self.get_pc() + 1);
                       self.jmp(target_addr - 0x8000);},
            // JMP_ind => {}, 

            // RTI     => {}, 
            RTS => {let ret_addr = self.pull_return_addr(interconnect); self.jmp(ret_addr);},

            // Bit tests
            BITZpg => {let val = self.zero_page(interconnect); self.bit(val); self.increment_pc(2);},
            // BIT_abs => {}, 

            // ALU operations
            // ORA_inx_=> {}, 
            // ORA_z_pg=> {}, 
            ORAImm => {let imm = self.immediate(interconnect); self.ora(imm); self.increment_pc(2);},
            // ORA_abs => {}, 
            // ORA_ind_=> {}, 
            // ORA_dx  => {}, 
            // ORA_ax  => {}, 
            // ORA_ay  => {}, 

            // AND_inx_=> {}, 
            // AND_z_pg=> {}, 
            ANDImm => {let val = self.immediate(interconnect); self.and(val); self.increment_pc(2);},
            // AND_abs => {}, 
            // AND_ind_=> {}, 
            // AND_dx  => {}, 
            // AND_ax  => {}, 
            // AND_ay  => {}, 

            // EOR_inx_=> {}, 
            // EOR_z_pg=> {}, 
            EORImm => {let val = self.immediate(interconnect); self.eor(val); self.increment_pc(2);}, 
            // EOR_abs => {}, 
            // EOR_ind_=> {}, 
            // EOR_dx  => {}, 
            // EOR_ax  => {}, 
            // EOR_ay  => {}, 

            // ADC_inx_=> {}, 
            // ADC_z_pg=> {}, 
            ADCImm => {let val = self.immediate(interconnect); self.add(val); self.increment_pc(2);},
            // ADC_abs => {}, 
            // ADC_ind_=> {}, 
            // ADC_dx  => {}, 
            // ADC_ax  => {}, 
            // ADC_ay  => {}, 

            // CMP_inx_=> {}, 
            // CMP_z_pg=> {}, 
            CMPImm => {let val = self.immediate(interconnect); self.compare(CPURegister::A, val); self.increment_pc(2);}, 
            // CMP_abs => {}, 
            // CMP_ind_=> {}, 
            // CMP_dx  => {}, 
            // CMP_ax  => {}, 
            // CMP_ay  => {}, 

            // SBC_inx_=> {}, 
            // SBC_z_pg=> {}, 
            SBCImm => {let val = self.immediate(interconnect); self.sub(val); self.increment_pc(2);}, 
            // SBC_abs => {}, 
            // SBC_ind_=> {}, 
            // SBC_dx  => {}, 
            // SBC_ax  => {}, 
            // SBC_ay  => {}, 
                 
            // ASL_z_pg=> {}, 
            // ASL     => {}, 
            // ASL_abs => {}, 
            // ASL_dx  => {}, 
            // ASL_ax  => {}, 

            // LSR_z_pg=> {}, 
            LSR     => {let val = self.read_reg(CPURegister::A) >> 1; self.write_to_reg(CPURegister::A, val); self.increment_pc(1);}, 
            // LSR_abs => {}, 
            // LSR_dx  => {}, 
            // LSR_ax  => {}, 

            // Rotates
            // ROL_z_pg=> {}, 
            // ROL     => {}, 
            // ROL_abs => {}, 
            // ROL_dx  => {}, 
            // ROL_ax  => {}, 

            // ROR_z_pg=> {}, 
            // ROR     => {}, 
            // ROR_abs => {}, 
            // ROR_dx  => {}, 
            // ROR_ax  => {}, 

            // Increments
            DECZpg => {let addr = interconnect.read_byte(self.get_pc() + 1);
                     let val = interconnect.read_byte(addr as u16);
                     val.wrapping_sub(1);
                     interconnect.write_byte(addr as u16, val);
                     self.increment_pc(2);},
            // DEC_abs => {}, 
            // DEC_dx  => {}, 
            // DEC_ax  => {}, 

            // INC_z_pg=> {}, 
            // INC_abs => {}, 
            // INC_dx  => {}, 
            // INC_ax  => {}, 

            // The ever important nop
            // Observe all its majesty
            NOP => {self.increment_pc(1);},
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
    // PRETTIFYME: This is a kludge
    fn add(&mut self, val: u8) {
        let a = self.read_reg(CPURegister::A) as u16;
        let mut sum = a + val as u16;
        if self.check_flag(CARRY_FLAG) {sum += 1;};
        if sum > 255 {self.set_flag(CARRY_FLAG)} else {self.unset_flag(CARRY_FLAG);}
        if !((a as u8) ^ val) & ((a as u8) ^ sum as u8) & 0x80 != 0 {self.set_flag(OVERFLOW_FLAG)} else {self.unset_flag(OVERFLOW_FLAG)}
        if (sum & 0xff) as u8 & (1 << 7) != 0 {self.set_flag(NEGATIVE_FLAG)} else {self.unset_flag(NEGATIVE_FLAG);}
        if ((sum & 0xff) as u8) == 0 {self.set_flag(ZERO_FLAG)} else {self.unset_flag(ZERO_FLAG);}
        self.write_to_reg(CPURegister::A, sum as u8);
    }

    fn and(&mut self, val: u8) {
        let a = self.read_reg(CPURegister::A);
        self.write_to_reg(CPURegister::A, a & val);
        if self.read_reg(CPURegister::A) & 0b1000_0000 != 0 {self.set_flag(NEGATIVE_FLAG);} else {self.unset_flag(NEGATIVE_FLAG);};
        if self.read_reg(CPURegister::A) == 0 {self.set_flag(ZERO_FLAG);} else {self.unset_flag(ZERO_FLAG);};
    }

    fn bit(&mut self, val: u8) {
        if val & self.read_reg(CPURegister::A) == 0 {self.set_flag(ZERO_FLAG);} else {self.unset_flag(ZERO_FLAG);}
        if val & (1 << 6) != 0 {self.set_flag(OVERFLOW_FLAG)} else {self.unset_flag(OVERFLOW_FLAG);}
        if val & (1 << 7) != 0 {self.set_flag(NEGATIVE_FLAG)} else {self.unset_flag(NEGATIVE_FLAG);}
    }

    fn branch(&mut self, interconnect: &Interconnect, branch_on: BranchOn) {
        let branch_target = interconnect.read_byte(self.get_pc() + 1) as i8;
        let pc = self.pc + 2;
        let branch = match branch_on {
            BranchOn::Plus => {!self.check_flag(NEGATIVE_FLAG)},
            BranchOn::Minus => {self.check_flag(NEGATIVE_FLAG)},
            BranchOn::OverflowClear => {!self.check_flag(OVERFLOW_FLAG)},
            BranchOn::OverflowSet => {self.check_flag(OVERFLOW_FLAG)},
            BranchOn::CarryClear => {!self.check_flag(CARRY_FLAG)},
            BranchOn::CarrySet => {self.check_flag(CARRY_FLAG)},
            BranchOn::NotEqual => {!self.check_flag(ZERO_FLAG)},
            BranchOn::Equal => {self.check_flag(ZERO_FLAG)},
        };
        if branch {self.pc = ((pc as i32) + (branch_target as i32)) as u16} else {self.increment_pc(2);}
    }

    // FIXME: This works...well enough
    fn compare(&mut self, register: CPURegister, mem: u8) {
        let reg = self.read_reg(register);
        if reg == mem {
            self.unset_flag(NEGATIVE_FLAG);
            self.set_flag(ZERO_FLAG);
            self.set_flag(CARRY_FLAG);
        } else if reg < mem {
            self.set_flag(NEGATIVE_FLAG);
            self.unset_flag(ZERO_FLAG);
            self.unset_flag(CARRY_FLAG);
        } else if reg > mem {
            self.unset_flag(NEGATIVE_FLAG);
            self.unset_flag(ZERO_FLAG);
            self.set_flag(CARRY_FLAG);
            if (reg - mem) & (1 << 7) !=0 {self.set_flag(NEGATIVE_FLAG);}
        }
    }

    fn decrement(&mut self, register: CPURegister) {
        let val = self.read_reg(register);
        self.write_to_reg(register, val.wrapping_sub(1));
        if self.read_reg(register) & 0b1000_0000 != 0 {self.set_flag(NEGATIVE_FLAG);} else {self.unset_flag(NEGATIVE_FLAG);};
        if self.read_reg(register) == 0 {self.set_flag(ZERO_FLAG);} else {self.unset_flag(ZERO_FLAG);};
    }

    fn eor(&mut self, val: u8) {
        let a = self.read_reg(CPURegister::A);
        self.write_to_reg(CPURegister::A, a ^ val);
        if self.read_reg(CPURegister::A) & 0b1000_0000 != 0 {self.set_flag(NEGATIVE_FLAG);} else {self.unset_flag(NEGATIVE_FLAG);};
        if self.read_reg(CPURegister::A) == 0 {self.set_flag(ZERO_FLAG);} else {self.unset_flag(ZERO_FLAG);};
    }

    fn increment(&mut self, register: CPURegister) {
        let val = self.read_reg(register);
        self.write_to_reg(register, val.wrapping_add(1));
        if self.read_reg(register) & 0b1000_0000 != 0 {self.set_flag(NEGATIVE_FLAG);} else {self.unset_flag(NEGATIVE_FLAG);};
        if self.read_reg(register) == 0 {self.set_flag(ZERO_FLAG);} else {self.unset_flag(ZERO_FLAG);};
    }

    fn jmp(&mut self, jump_target: u16) {
        self.pc = jump_target;
    }

    fn load(&mut self, register: CPURegister, val: u8) {
        self.write_to_reg(register, val);
        if self.read_reg(register) & 0b1000_0000 != 0 {self.set_flag(NEGATIVE_FLAG);} else {self.unset_flag(NEGATIVE_FLAG);};
        if self.read_reg(register) == 0 {self.set_flag(ZERO_FLAG);} else {self.unset_flag(ZERO_FLAG);};
    }

    fn ora(&mut self, val: u8) {
        let a = self.read_reg(CPURegister::A);
        self.write_to_reg(CPURegister::A, a | val);
        if self.read_reg(CPURegister::A) & 0b1000_0000 != 0 {self.set_flag(NEGATIVE_FLAG);} else {self.unset_flag(NEGATIVE_FLAG);};
        if self.read_reg(CPURegister::A) == 0 {self.set_flag(ZERO_FLAG);} else {self.unset_flag(ZERO_FLAG);};
    }

    fn transfer(&mut self, from_reg: CPURegister, to_reg: CPURegister) {
        let val = self.read_reg(from_reg);
        self.write_to_reg(to_reg, val);
        if to_reg != CPURegister::StackPointer {
            if self.read_reg(to_reg) & 0b1000_0000 != 0 {self.set_flag(NEGATIVE_FLAG);} else {self.unset_flag(NEGATIVE_FLAG);};
            if self.read_reg(to_reg) == 0 {self.set_flag(ZERO_FLAG);} else {self.unset_flag(ZERO_FLAG);};
        }
    }

    fn store(&mut self, interconnect: &mut Interconnect, addr: u16, register: CPURegister) {
        let val = self.read_reg(register);
        interconnect.write_byte(addr, val);
    }

    fn sub(&mut self, val: u8) {
        self.add(!val);
    }
}

// TODO: Move this to a propper debugger
impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#x} a:0x{:x} x:0x{:x} y:0x{:x} stack_pointer:0x{:x} status:0b{:#b}",
               self.pc, self.a, self.x, self.y,  self.stack_pointer, self.status)
    }
}

