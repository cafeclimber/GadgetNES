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
        self.pc = 0x8000;
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

    fn push_byte_stack(&mut self, interconnect: &mut Interconnect, byte: u8) {
        let addr = self.read_reg(CPURegister::StackPointer) as u16 + 0x100;
        interconnect.write_byte(addr, byte);
        self.stack_pointer -= 1;
    }

    fn pull_byte_stack(&mut self, interconnect: &Interconnect, register: CPURegister) {
        self.stack_pointer += 1;
        let addr = self.read_reg(CPURegister::StackPointer) as u16 + 0x100;
        self.write_to_reg(register, interconnect.read_byte(addr));
        self.set_flag(STACK_COPY);
    }

    fn push_return_addr(&mut self, interconnect: &mut Interconnect) {
        let pc = self.pc + 2;
        let pc_msb = ((pc & 0xff00) >> 8) as u8;
        let pc_lsb = (pc & 0x00ff) as u8;
        self.push_byte_stack(interconnect, pc_msb);
        self.push_byte_stack(interconnect, pc_lsb);
    }

    fn pull_return_addr(&mut self, interconnect: &mut Interconnect) -> u16 {
        self.stack_pointer += 1;
        let addr = self.read_reg(CPURegister::StackPointer) as u16 + 0x100;
        let pc_lsb = interconnect.read_byte(addr) as u16;
        self.stack_pointer += 1;
        let addr = self.read_reg(CPURegister::StackPointer) as u16 + 0x100;
        let pc_msb = (interconnect.read_byte(addr) as u16) << 8;
        let ret_addr = pc_msb | pc_lsb;
        ret_addr + 1
    }

    // TODO: Superfluous
    fn get_pc(&self) -> u16 {
        self.pc
    }

    fn bump_pc(&mut self, increment_by: u16) {
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
        println!("{:X} {:?} \t A:{:2X} X:{:2X} Y:{:2X} P:{:2X} SP:{:2X}", self.pc, instr, self.a, self.x, self.y, self.status, self.stack_pointer);
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
            PHP => {let byte = self.read_reg(CPURegister::Status); self.push_byte_stack(interconnect, byte); self.set_flag(BRK_FLAG); self.bump_pc(1);},
            PLP => {self.pull_byte_stack(interconnect, CPURegister::Status); self.bump_pc(1);},
            PHA => {let byte = self.read_reg(CPURegister::A); self.push_byte_stack(interconnect, byte); self.bump_pc(1);},
            PLA => {self.pull_byte_stack(interconnect, CPURegister::A);
                    self.bump_pc(1);
                    if self.read_reg(CPURegister::A) & 0b1000_0000 != 0 {self.set_flag(NEGATIVE_FLAG);} else {self.unset_flag(NEGATIVE_FLAG);};
                    if self.read_reg(CPURegister::A) == 0 {self.set_flag(ZERO_FLAG);} else {self.unset_flag(ZERO_FLAG);};}
            TXS => {self.transfer(CPURegister::X, CPURegister::StackPointer); self.bump_pc(1);},
            TSX => {self.transfer(CPURegister::StackPointer, CPURegister::X); self.bump_pc(1);},

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
            CLC => {self.unset_flag(CARRY_FLAG); self.bump_pc(1);},
            SEC => {self.set_flag(CARRY_FLAG); self.bump_pc(1);},
            // CLI => {},
            SEI => {self.set_flag(IRQ_FLAG); self.bump_pc(1);},
            CLV => {self.unset_flag(OVERFLOW_FLAG); self.bump_pc(1)},
            CLD => {self.unset_flag(DECIMAL_FLAG); self.bump_pc(1);},
            SED => {self.set_flag(DECIMAL_FLAG); self.bump_pc(1);},

            // Register instructions
            DEY => {let mut val = self.read_reg(CPURegister::Y);
                    val = self.decrement(val);
                    self.write_to_reg(CPURegister::Y, val);
                    self.bump_pc(1);},
            DEX => {let mut val = self.read_reg(CPURegister::X);
                    val = self.decrement(val);
                    self.write_to_reg(CPURegister::X, val);
                    self.bump_pc(1);},
            INX => {let mut val = self.read_reg(CPURegister::X);
                    val = self.increment(val);
                    self.write_to_reg(CPURegister::X, val);
                    self.bump_pc(1);},
            INY => {let mut val = self.read_reg(CPURegister::Y);
                    val = self.increment(val);
                    self.write_to_reg(CPURegister::Y, val);
                    self.bump_pc(1);},
            TAX => {self.transfer(CPURegister::A, CPURegister::X); self.bump_pc(1);},
            TXA => {self.transfer(CPURegister::X, CPURegister::A); self.bump_pc(1);},
            TAY => {self.transfer(CPURegister::A, CPURegister::Y); self.bump_pc(1);},
            TYA => {self.transfer(CPURegister::Y, CPURegister::A); self.bump_pc(1);},

            // Compares
            CPYImm => {let val = self.immediate(interconnect); self.compare(CPURegister::Y, val); self.bump_pc(2);}, 
            CPYZPg=> {let val = self.zero_page(interconnect); self.compare(CPURegister::Y, val); self.bump_pc(2);}, 
            // CPY_abs => {}, 
            CPXImm => {let val = self.immediate(interconnect); self.compare(CPURegister::X, val); self.bump_pc(2);}, 
            CPXZPg=> {let val = self.zero_page(interconnect); self.compare(CPURegister::X, val); self.bump_pc(2);}, 
            // CPX_abs => {}, 

            // Loads
            LDAInxX=> {let val = self.indexed_indirect(interconnect);
                       self.load(CPURegister::A, val);
                       self.bump_pc(2);}, 
            LDAZpg => {let val = self.zero_page(interconnect);
                       self.load(CPURegister::A, val);
                       self.bump_pc(2);}, 
            LDAImm => {let val = self.immediate(interconnect);
                       self.load(CPURegister::A, val);
                       self.bump_pc(2);}, 
            LDAAbs => {let val = self.absolute(interconnect);
                       self.load(CPURegister::A, val);
                       self.bump_pc(3);}, 
            LDAIndY => {let val = self.indirect_indexed(interconnect);
                        self.load(CPURegister::A, val);
                        self.bump_pc(2);}, 
            // LDA_dx   => {}, 
            // LDA_ax   => {}, 
            // LDA_ay   => {}, 

            LDXImm => {let val = self.immediate(interconnect);
                       self.load(CPURegister::X, val);
                       self.bump_pc(2);},
            LDXZpg => {let val = self.zero_page(interconnect);
                       self.load(CPURegister::X, val);
                       self.bump_pc(2);},
            LDXAbs => {let val = self.absolute(interconnect);
                       self.load(CPURegister::X, val);
                       self.bump_pc(3);}, 
            // LDX_dy  => {}, 
            // LDX_ay  => {}, 

            LDYImm => {let val = self.immediate(interconnect);
                       self.load(CPURegister::Y, val);
                       self.bump_pc(2);},
            LDYZPg=> {let val = self.zero_page(interconnect);
                       self.load(CPURegister::Y, val);
                       self.bump_pc(2);}, 
            // LDY_abs => {}, 
            // LDY_dx  => {}, 
            // LDY_ax  => {}, 

            // Stores
            // PRETTIFYME: This is gross. Abstract, pls
            STAInxX => {let addr = interconnect.read_byte(self.get_pc() + 1);
                        let sum_addr = addr + self.read_reg(CPURegister::X);
                        let full_addr = if sum_addr == 0xff {
                            (interconnect.read_byte(sum_addr as u16) as u16) |
                            ((interconnect.read_byte(0x0000) as u16) << 8)
                        } else {
                            interconnect.read_word(sum_addr as u16)
                        };
                        self.store(interconnect, full_addr, CPURegister::A);
                        self.bump_pc(2);}, 
            STAZpg => {let addr = interconnect.read_byte(self.get_pc() + 1);
                       self.store(interconnect, addr as u16, CPURegister::A);
                       self.bump_pc(2);},
            STAAbs => {let addr = interconnect.read_word(self.get_pc() + 1);
                       self.store(interconnect, addr, CPURegister::A);
                       self.bump_pc(3);},
            /*STAIndY => {let addr = interconnect.read_byte(self.get_pc() + 1);
                        let sum_addr = addr + self.read_reg(CPURegister::X);
                        let addr = interconnect.read_word(sum_addr as u16);
                        self.store(interconnect, addr, CPURegister::A);
                        self.bump_pc(2);},*/
            // STA_dx  => {}, 
            // STA_ax  => {}, 
            // STA_ay  => {}, 

            STXZpg => {let addr = interconnect.read_byte(self.get_pc() + 1);
                       self.store(interconnect, addr as u16, CPURegister::X);
                       self.bump_pc(2);},
            STXAbs => {let addr = interconnect.read_word(self.get_pc() + 1);
                       self.store(interconnect, addr, CPURegister::X);
                       self.bump_pc(3);},
            // STX_dy => {}, 
            STYZpg => {let addr = interconnect.read_byte(self.get_pc() + 1);
                       self.store(interconnect, addr as u16, CPURegister::Y);
                       self.bump_pc(2);},
            // STY_abs => {}, 
            // STY_dx  => {}, 

            // Jumps
            JSRAbs => {self.push_return_addr(interconnect);
                       let addr = interconnect.read_word(self.get_pc() + 1);
                       self.jmp(addr);},
            JMPAbs => {let target_addr = interconnect.read_word(self.get_pc() + 1);
                       self.jmp(target_addr);},
            // JMP_ind => {}, 

            RTI     => {self.pull_byte_stack(interconnect, CPURegister::Status);
                        let ret_addr = self.pull_return_addr(interconnect) - 1; // TODO: Correct?
                        self.jmp(ret_addr);}, 
            RTS => {let ret_addr = self.pull_return_addr(interconnect); self.jmp(ret_addr);},

            // Bit tests
            BITZpg => {let val = self.zero_page(interconnect); self.bit(val); self.bump_pc(2);},
            // BIT_abs => {}, 

            // ALU operations
            ORAInxX => {let val = self.indexed_indirect(interconnect); self.ora(val); self.bump_pc(2);}, 
            ORAZPg=> {let val = self.zero_page(interconnect); self.ora(val); self.bump_pc(2);}, 
            ORAImm => {let imm = self.immediate(interconnect); self.ora(imm); self.bump_pc(2);},
            // ORA_abs => {}, 
            // ORA_ind_=> {}, 
            // ORA_dx  => {}, 
            // ORA_ax  => {}, 
            // ORA_ay  => {}, 

            ANDInxX => {let val = self.indexed_indirect(interconnect); self.and(val); self.bump_pc(2);}, 
            ANDZPg=> {let val = self.zero_page(interconnect); self.and(val); self.bump_pc(2);}, 
            ANDImm => {let val = self.immediate(interconnect); self.and(val); self.bump_pc(2);},
            // AND_abs => {}, 
            // AND_ind_=> {}, 
            ANDZPgX  => {let val = self.z_page_indexed(interconnect, CPURegister::X); self.and(val); self.bump_pc(2);}, 
            // AND_ax  => {}, 
            // AND_ay  => {}, 

            EORInxX => {let val = self.indexed_indirect(interconnect); self.eor(val); self.bump_pc(2);}, 
            EORZPg=> {let val = self.zero_page(interconnect); self.eor(val); self.bump_pc(2);}, 
            EORImm => {let val = self.immediate(interconnect); self.eor(val); self.bump_pc(2);}, 
            // EOR_abs => {}, 
            // EOR_ind_=> {}, 
            // EOR_dx  => {}, 
            // EOR_ax  => {}, 
            // EOR_ay  => {}, 

            ADCInxX => {let val = self.indexed_indirect(interconnect); self.add(val); self.bump_pc(2);}, 
            ADCZPg=> {let val = self.zero_page(interconnect); self.add(val); self.bump_pc(2);}, 
            ADCImm => {let val = self.immediate(interconnect); self.add(val); self.bump_pc(2);},
            // ADC_abs => {}, 
            // ADC_ind_=> {}, 
            // ADC_dx  => {}, 
            // ADC_ax  => {}, 
            // ADC_ay  => {}, 

            CMPInxX => {let val = self.indexed_indirect(interconnect); self.compare(CPURegister::A, val); self.bump_pc(2);},
            CMPZPg=> {let val = self.zero_page(interconnect); self.compare(CPURegister::A, val); self.bump_pc(2);}, 
            CMPImm => {let val = self.immediate(interconnect); self.compare(CPURegister::A, val); self.bump_pc(2);}, 
            // CMP_abs => {}, 
            // CMP_ind_=> {}, 
            // CMP_dx  => {}, 
            // CMP_ax  => {}, 
            // CMP_ay  => {}, 

            SBCInxX=> {let val = self.indexed_indirect(interconnect); self.sub(val); self.bump_pc(2);}, 
            SBCZPg=> {let val = self.zero_page(interconnect); self.sub(val); self.bump_pc(2);}, 
            SBCImm => {let val = self.immediate(interconnect); self.sub(val); self.bump_pc(2);}, 
            // SBC_abs => {}, 
            // SBC_ind_=> {}, 
            // SBC_dx  => {}, 
            // SBC_ax  => {}, 
            // SBC_ay  => {}, 
                 
            ASLZPg=> {let mut addr = self.get_pc() + 1;
                      addr = interconnect.read_byte(addr) as u16;
                      let val = self.zero_page(interconnect);
                      let eval = self.asl(val);
                      interconnect.write_byte(addr, eval);
                      self.bump_pc(2);}, 
            ASL     => {let val = self.read_reg(CPURegister::A);
                        let eval = self.asl(val);
                        self.write_to_reg(CPURegister::A, eval);
                        self.bump_pc(1);}, 
            // ASL_abs => {}, 
            // ASL_dx  => {}, 
            // ASL_ax  => {}, 

            LSRZPg=> {let mut addr = self.get_pc() + 1;
                      addr = interconnect.read_byte(addr) as u16;
                      let val = self.zero_page(interconnect);
                      let eval = self.lsr(val);
                      interconnect.write_byte(addr, eval);
                      self.bump_pc(2);}, 
            LSR     => {let val = self.read_reg(CPURegister::A);
                        let eval = self.lsr(val);
                        self.write_to_reg(CPURegister::A, eval);
                        self.bump_pc(1);}, 
            // LSR_abs => {}, 
            // LSR_dx  => {}, 
            // LSR_ax  => {}, 

            // Rotates
            ROLZPg=> {let mut addr = self.get_pc() + 1;
                      addr = interconnect.read_byte(addr) as u16;
                      let val = self.zero_page(interconnect);
                      let eval = self.rol(val);
                      interconnect.write_byte(addr, eval);
                      self.bump_pc(2);},
            ROL     => {let val = self.read_reg(CPURegister::A);
                        let eval = self.rol(val);
                        self.write_to_reg(CPURegister::A, eval);
                        self.bump_pc(1);}, 
            // ROL_abs => {}, 
            // ROL_dx  => {}, 
            // ROL_ax  => {}, 

            RORZPg=> {let mut addr = self.get_pc() + 1;
                      addr = interconnect.read_byte(addr) as u16;
                      let val = self.zero_page(interconnect);
                      let eval = self.ror(val);
                      interconnect.write_byte(addr, eval);
                      self.bump_pc(2);}, 
            ROR     => {let val = self.read_reg(CPURegister::A);
                        let eval = self.ror(val);
                        self.write_to_reg(CPURegister::A, eval);
                        self.bump_pc(1);}, 
            // ROR_abs => {}, 
            // ROR_dx  => {}, 
            // ROR_ax  => {}, 

            // Increments
            DECZpg => {let addr = interconnect.read_byte(self.get_pc() + 1);
                       let val = interconnect.read_byte(addr as u16);
                       val.wrapping_sub(1);
                       interconnect.write_byte(addr as u16, val);
                       self.bump_pc(2);},
            // DEC_abs => {}, 
            // DEC_dx  => {}, 
            // DEC_ax  => {}, 

            INCZPg=> {let addr = interconnect.read_byte(self.get_pc() + 1);
                       let val = interconnect.read_byte(addr as u16);
                       val.wrapping_add(1);
                       interconnect.write_byte(addr as u16, val);
                       self.bump_pc(2);}, 
            // INC_abs => {}, 
            // INC_dx  => {}, 
            // INC_ax  => {}, 

            // The ever important nop
            // Observe all its majesty
            NOP => {self.bump_pc(1);},
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
        let sum_addr = addr.wrapping_add(self.read_reg(register));
        interconnect.read_byte(sum_addr as u16)
    }

    // PRETTIFYME
    fn indexed_indirect(&self, interconnect: &Interconnect) -> u8 {
        let addr = interconnect.read_byte(self.get_pc() + 1);
        let sum_addr = addr.wrapping_add((self.read_reg(CPURegister::X))) as u16;
        if sum_addr == 0xff {
            let fetch_addr = (interconnect.read_byte(sum_addr as u16) as u16) | ((interconnect.read_byte(0x0000) as u16) << 8);
            interconnect.read_byte(fetch_addr)
        } else {
            let fetch_addr = interconnect.read_word(sum_addr as u16);
            interconnect.read_byte(fetch_addr)
        }
    }

    fn indirect_indexed(&self, interconnect: &Interconnect) -> u8 {
        let mut addr = interconnect.read_word(self.get_pc() + 1);
        addr += self.read_reg(CPURegister::Y) as u16;
        interconnect.read_byte(addr)
    }

    // Instruction abstractions
    // PRETTIFYME: This is a kludge
    fn add(&mut self, arg: u8) {
        let a = self.read_reg(CPURegister::A) as u16;
        let mut sum = a + arg as u16;
        if self.check_flag(CARRY_FLAG) {sum += 1;};
        if sum > 255 {self.set_flag(CARRY_FLAG)} else {self.unset_flag(CARRY_FLAG);}
        if !((a as u8) ^ arg) & ((a as u8) ^ sum as u8) & 0x80 != 0 {self.set_flag(OVERFLOW_FLAG)} else {self.unset_flag(OVERFLOW_FLAG)}
        if (sum & 0xff) as u8 & (1 << 7) != 0 {self.set_flag(NEGATIVE_FLAG)} else {self.unset_flag(NEGATIVE_FLAG);}
        if ((sum & 0xff) as u8) == 0 {self.set_flag(ZERO_FLAG)} else {self.unset_flag(ZERO_FLAG);}
        self.write_to_reg(CPURegister::A, sum as u8);
    }

    fn and(&mut self, arg: u8) {
        let a = self.read_reg(CPURegister::A);
        self.write_to_reg(CPURegister::A, a & arg);
        if self.read_reg(CPURegister::A) & 0b1000_0000 != 0 {self.set_flag(NEGATIVE_FLAG);} else {self.unset_flag(NEGATIVE_FLAG);};
        if self.read_reg(CPURegister::A) == 0 {self.set_flag(ZERO_FLAG);} else {self.unset_flag(ZERO_FLAG);};
    }

    fn asl(&mut self, arg: u8) -> u8 {
        if arg & (1 << 7) != 0 {self.set_flag(CARRY_FLAG);} else {self.unset_flag(CARRY_FLAG);}
        let eval = ((arg as i8) << 1) as u8; // cast to ensure arithmetic vs. logical shift
        if eval & 0b1000_0000 != 0 {self.set_flag(NEGATIVE_FLAG);} else {self.unset_flag(NEGATIVE_FLAG);};
        if eval == 0 {self.set_flag(ZERO_FLAG);} else {self.unset_flag(ZERO_FLAG);};
        eval
    }

    fn bit(&mut self, arg: u8) {
        if arg & self.read_reg(CPURegister::A) == 0 {self.set_flag(ZERO_FLAG);} else {self.unset_flag(ZERO_FLAG);}
        if arg & (1 << 6) != 0 {self.set_flag(OVERFLOW_FLAG)} else {self.unset_flag(OVERFLOW_FLAG);}
        if arg & (1 << 7) != 0 {self.set_flag(NEGATIVE_FLAG)} else {self.unset_flag(NEGATIVE_FLAG);}
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
        if branch {self.pc = ((pc as i32) + (branch_target as i32)) as u16} else {self.bump_pc(2);}
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

    fn decrement(&mut self, arg: u8) -> u8 {
        let eval = arg.wrapping_sub(1);
        if eval & 0b1000_0000 != 0 {self.set_flag(NEGATIVE_FLAG);} else {self.unset_flag(NEGATIVE_FLAG);};
        if eval == 0 {self.set_flag(ZERO_FLAG);} else {self.unset_flag(ZERO_FLAG);};
        eval
    }

    fn eor(&mut self, arg: u8) {
        let a = self.read_reg(CPURegister::A);
        self.write_to_reg(CPURegister::A, a ^ arg);
        if self.read_reg(CPURegister::A) & 0b1000_0000 != 0 {self.set_flag(NEGATIVE_FLAG);} else {self.unset_flag(NEGATIVE_FLAG);};
        if self.read_reg(CPURegister::A) == 0 {self.set_flag(ZERO_FLAG);} else {self.unset_flag(ZERO_FLAG);};
    }

    fn increment(&mut self, arg: u8) -> u8 {
        let eval = arg.wrapping_add(1);
        if eval & 0b1000_0000 != 0 {self.set_flag(NEGATIVE_FLAG);} else {self.unset_flag(NEGATIVE_FLAG);};
        if eval == 0 {self.set_flag(ZERO_FLAG);} else {self.unset_flag(ZERO_FLAG);};
        eval
    }

    fn jmp(&mut self, jump_target: u16) {
        self.pc = jump_target;
    }

    fn load(&mut self, register: CPURegister, arg: u8) {
        self.write_to_reg(register, arg);
        if self.read_reg(register) & 0b1000_0000 != 0 {self.set_flag(NEGATIVE_FLAG);} else {self.unset_flag(NEGATIVE_FLAG);};
        if self.read_reg(register) == 0 {self.set_flag(ZERO_FLAG);} else {self.unset_flag(ZERO_FLAG);};
    }

    fn lsr(&mut self, arg: u8) -> u8 {
        if arg & (1 << 0) != 0 {self.set_flag(CARRY_FLAG);} else {self.unset_flag(CARRY_FLAG);}
        let eval = arg >> 1;
        if eval & 0b1000_0000 != 0 {self.set_flag(NEGATIVE_FLAG);} else {self.unset_flag(NEGATIVE_FLAG);};
        if eval == 0 {self.set_flag(ZERO_FLAG);} else {self.unset_flag(ZERO_FLAG);};
        eval
    }

    fn ora(&mut self, arg: u8) {
        let a = self.read_reg(CPURegister::A);
        self.write_to_reg(CPURegister::A, a | arg);
        if self.read_reg(CPURegister::A) & 0b1000_0000 != 0 {self.set_flag(NEGATIVE_FLAG);} else {self.unset_flag(NEGATIVE_FLAG);};
        if self.read_reg(CPURegister::A) == 0 {self.set_flag(ZERO_FLAG);} else {self.unset_flag(ZERO_FLAG);};
    }

    fn rol(&mut self, arg: u8) -> u8 {
        let carry = if self.check_flag(CARRY_FLAG) {1 << 0} else {0};
        if arg & (1 << 7) != 0 {self.set_flag(CARRY_FLAG);} else {self.unset_flag(CARRY_FLAG);}
        let eval = (arg << 1) | carry;
        if eval & 0b1000_0000 != 0 {self.set_flag(NEGATIVE_FLAG);} else {self.unset_flag(NEGATIVE_FLAG);};
        if eval == 0 {self.set_flag(ZERO_FLAG);} else {self.unset_flag(ZERO_FLAG);};
        eval
    }

    fn ror(&mut self, arg: u8) -> u8 {
        let carry = if self.check_flag(CARRY_FLAG) {1 << 7} else {0};
        if arg & (1 << 0) != 0 {self.set_flag(CARRY_FLAG);} else {self.unset_flag(CARRY_FLAG);}
        let eval = (arg >> 1) | carry;
        if eval & 0b1000_0000 != 0 {self.set_flag(NEGATIVE_FLAG);} else {self.unset_flag(NEGATIVE_FLAG);};
        if eval == 0 {self.set_flag(ZERO_FLAG);} else {self.unset_flag(ZERO_FLAG);};
        eval
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

    fn sub(&mut self, arg: u8) {
        self.add(!arg);
    }
}

// TODO: Move this to a propper debugger
impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#x} a:0x{:x} x:0x{:x} y:0x{:x} stack_pointer:0x{:x} status:0b{:#b}",
               self.pc, self.a, self.x, self.y,  self.stack_pointer, self.status)
    }
}

