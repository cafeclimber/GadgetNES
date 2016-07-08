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
    AbsoluteIndexed,
    ZeroPageIndexed,
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
    // TODO: Cycles
    pub fn run_instr(&mut self, interconnect: &mut Interconnect) {
        use enum_primitive::FromPrimitive;
        use instructions::Instruction::*;
        let raw_instr = interconnect.read_byte(self.pc);
        let instr = Instruction::from_u8(raw_instr).unwrap_or_else(|| {
            panic!("Unrecognized instruction: {:#x} Last Failure code: (02h): {:x} (03h): {:x}",
                   raw_instr, interconnect.read_byte(0x02), interconnect.read_byte(0x03));
        });
        println!("{:04X} {:?} \t A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X}",
                 self.pc, instr, self.a, self.x, self.y, self.status, self.stack_pointer);
        match instr {
            // TODO: Implement unofficial opcodes

            BRK => {self.bump_pc(2);
                    self.push_return_addr(interconnect);
                    let status = self.read_reg(CPURegister::Status);
                    self.set_flag(IRQ_FLAG);
                    self.push_byte_stack(interconnect, status);
                    let branch_target = interconnect.read_word(IRQBRK_VECTOR);
                    self.jmp(branch_target);
            },

            // Stack    
            PHP => {
                let byte = self.read_reg(CPURegister::Status);
                self.push_byte_stack(interconnect, byte);
                self.set_flag(BRK_FLAG); self.bump_pc(1);
            },
            PLP => {
                self.pull_byte_stack(interconnect, CPURegister::Status);
                self.bump_pc(1);
            },
            PHA => {
                let byte = self.read_reg(CPURegister::A);
                self.push_byte_stack(interconnect, byte);
                self.bump_pc(1);
            },
            PLA => {
                self.pull_byte_stack(interconnect, CPURegister::A);
                self.bump_pc(1);
                if self.read_reg(CPURegister::A) & 0b1000_0000 != 0 {
                    self.set_flag(NEGATIVE_FLAG);
                } else {
                    self.unset_flag(NEGATIVE_FLAG);
                }
                if self.read_reg(CPURegister::A) == 0 {
                    self.set_flag(ZERO_FLAG);
                } else {
                    self.unset_flag(ZERO_FLAG);
                }
            },
            TXS => {
                self.transfer(CPURegister::X, CPURegister::StackPointer);
                self.bump_pc(1);
            },
            TSX => {
                self.transfer(CPURegister::StackPointer, CPURegister::X);
                self.bump_pc(1);
            },

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
            DEY => {
                let mut val = self.read_reg(CPURegister::Y);
                val = self.decrement(val);
                self.write_to_reg(CPURegister::Y, val);
                self.bump_pc(1);
            },
            DEX => {
                let mut val = self.read_reg(CPURegister::X);
                val = self.decrement(val);
                self.write_to_reg(CPURegister::X, val);
                self.bump_pc(1);
            },
            INX => {
                let mut val = self.read_reg(CPURegister::X);
                val = self.increment(val);
                self.write_to_reg(CPURegister::X, val);
                self.bump_pc(1);
            },
            INY => {
                let mut val = self.read_reg(CPURegister::Y);
                val = self.increment(val);
                self.write_to_reg(CPURegister::Y, val);
                self.bump_pc(1);
            },
            TAX => {
                self.transfer(CPURegister::A, CPURegister::X);
                self.bump_pc(1);
            },
            TXA => {
                self.transfer(CPURegister::X, CPURegister::A);
                self.bump_pc(1);
            },
            TAY => {
                self.transfer(CPURegister::A, CPURegister::Y);
                self.bump_pc(1);
            },
            TYA => {
                self.transfer(CPURegister::Y, CPURegister::A);
                self.bump_pc(1);
            },

            // Compares
            CPYImm => {
                let val = self.immediate(interconnect);
                self.compare(CPURegister::Y, val);
                self.bump_pc(2);
            }, 
            CPYZPg=> {
                let val = self.zero_page(interconnect);
                self.compare(CPURegister::Y, val);
                self.bump_pc(2);
            }, 
            CPYAbs => {
                let val = self.absolute(interconnect);
                self.compare(CPURegister::Y, val);
                self.bump_pc(3);
            }, 
            CPXImm => {
                let val = self.immediate(interconnect);
                self.compare(CPURegister::X, val);
                self.bump_pc(2);
            }, 
            CPXZPg=> {
                let val = self.zero_page(interconnect);
                self.compare(CPURegister::X, val);
                self.bump_pc(2);
            }, 
            CPXAbs => {
                let val = self.absolute(interconnect);
                self.compare(CPURegister::X, val);
                self.bump_pc(3);
            }, 

            // Loads
            LDAInxX=> {
                let val = self.indexed_indirect(interconnect);
                self.load(CPURegister::A, val);
                self.bump_pc(2);
            }, 
            LDAZpg => {
                let val = self.zero_page(interconnect);
                self.load(CPURegister::A, val);
                self.bump_pc(2);
            }, 
            LDAImm => {
                let val = self.immediate(interconnect);
                self.load(CPURegister::A, val);
                self.bump_pc(2);
            }, 
            LDAAbs => {
                let val = self.absolute(interconnect);
                self.load(CPURegister::A, val);
                self.bump_pc(3);
            }, 
            LDAIndY => {
                let val = self.indirect_indexed(interconnect);
                self.load(CPURegister::A, val);
                self.bump_pc(2);
            }, 
            LDAZPgX   => {
                let val = self.z_page_indexed(interconnect, CPURegister::X);
                self.load(CPURegister::A, val);
                self.bump_pc(2);
            }, 
            LDAAx   => {
                let val = self.absolute_indexed(interconnect, CPURegister::X);
                self.load(CPURegister::A, val);
                self.bump_pc(3);
            }, 
            LDAAy   => {
                let val = self.absolute_indexed(interconnect, CPURegister::Y);
                self.load(CPURegister::A, val);
                self.bump_pc(3);
            }, 

            LDXImm => {
                let val = self.immediate(interconnect);
                self.load(CPURegister::X, val);
                self.bump_pc(2);
            },
            LDXZpg => {
                let val = self.zero_page(interconnect);
                self.load(CPURegister::X, val);
                self.bump_pc(2);
            },
            LDXAbs => {
                let val = self.absolute(interconnect);
                self.load(CPURegister::X, val);
                self.bump_pc(3);
            }, 
            LDXZPgY  => {
                let val = self.z_page_indexed(interconnect, CPURegister::Y);
                self.load(CPURegister::X, val);
                self.bump_pc(2);
            }, 
            LDXAy  => {
                let val = self.absolute_indexed(interconnect, CPURegister::Y);
                self.load(CPURegister::X, val);
                self.bump_pc(3);
            }, 

            LDYImm => {
                let val = self.immediate(interconnect);
                self.load(CPURegister::Y, val);
                self.bump_pc(2);
            },
            LDYZPg=> {
                let val = self.zero_page(interconnect);
                self.load(CPURegister::Y, val);
                self.bump_pc(2);
            }, 
            LDYAbs => {
                let val = self.absolute(interconnect);
                self.load(CPURegister::Y, val);
                self.bump_pc(3);
            }, 
            LDYZPgX  => {
                let val = self.z_page_indexed(interconnect, CPURegister::X);
                self.load(CPURegister::Y, val);
                self.bump_pc(2);
            }, 
            LDYAx  => {
                let val = self.absolute_indexed(interconnect, CPURegister::X);
                self.load(CPURegister::Y, val);
                self.bump_pc(3);
            }, 

            // Stores
            // PRETTIFYME: This is gross. Abstract, pls
            STAInxX => {
                let addr = interconnect.read_byte(self.pc + 1);
                let sum_addr = addr + self.read_reg(CPURegister::X);
                let full_addr = if sum_addr == 0xff {
                    (interconnect.read_byte(sum_addr as u16) as u16) |
                    ((interconnect.read_byte(0x0000) as u16) << 8)
                } else {
                    interconnect.read_word(sum_addr as u16)
                };
                self.store(interconnect, full_addr, CPURegister::A);
                self.bump_pc(2);
            }, 
            STAZpg => {
                let addr = interconnect.read_byte(self.pc + 1);
                self.store(interconnect, addr as u16, CPURegister::A);
                self.bump_pc(2);
            },
            STAAbs => {
                let addr = interconnect.read_word(self.pc + 1);
                self.store(interconnect, addr, CPURegister::A);
                self.bump_pc(3);
            },
            STAIndY => {
                let addr = self.indirect_indexed_addr(interconnect);
                self.store(interconnect, addr, CPURegister::A);
                self.bump_pc(2);
            },
            STAZPgX  => {
                let addr = self.z_page_indexed(interconnect, CPURegister::X);
                self.store(interconnect, addr, CPURegister::A);
                self.bump_pc(2);
            }, 
            STAAx  => {
                let addr = self.absolute_indexed_addr(interconnect, CPURegister::X);
                self.store(interconnect, addr, CPURegister::A);
                self.bump_pc(3);
            },
            STAAy  => {
                let addr = self.absolute_indexed_addr(interconnect, CPURegister::Y);
                self.store(interconnect, addr, CPURegister::A);
                self.bump_pc(3);
            }, 

            STXZpg => {
                let addr = interconnect.read_byte(self.pc + 1);
                self.store(interconnect, addr as u16, CPURegister::X);
                self.bump_pc(2);
            },
            STXAbs => {
                let addr = interconnect.read_word(self.pc + 1);
                self.store(interconnect, addr, CPURegister::X);
                self.bump_pc(3);
            },
            STXZPgY => {
                let addr = self.z_page_indexed(interconnect, CPURegister::Y);
                self.store(interconnect, addr, CPURegister::X);
                self.bump_pc(2);
            }, 
            STYZpg => {
                let addr = interconnect.read_byte(self.pc + 1);
                self.store(interconnect, addr as u16, CPURegister::Y);
                self.bump_pc(2);
            },
            STYAbs => {
                let addr = interconnect.read_word(self.pc + 1);
                self.store(interconnect, addr as u16, CPURegister::Y);
                self.bump_pc(3);
            }, 
            STYZPgX => {
                let addr = self.z_page_indexed(interconnect, CPURegister::X);
                self.store(interconnect, addr, CPURegister::Y);
                self.bump_pc(2);
            }, 

            // Jumps
            JSRAbs => {
                self.push_return_addr(interconnect);
                let addr = interconnect.read_word(self.pc + 1);
                self.jmp(addr);
            },
            JMPAbs => {
                let target_addr = interconnect.read_word(self.pc + 1);
                self.jmp(target_addr);
            },
            JMPInd => {
                let addr = interconnect.read_word(self.pc + 1);
                if addr & 0x00ff == 0xff {
                    let target_addr = interconnect.read_byte(addr) as u16 |
                    ((interconnect.read_byte(addr + 1 - 0x100) as u16) << 8);
                    self.jmp(target_addr);
                } else {
                    let target_addr = interconnect.read_word(addr);
                    self.jmp(target_addr);
                }}
            
            RTI     => {
                self.pull_byte_stack(interconnect, CPURegister::Status);
                let ret_addr = self.pull_return_addr(interconnect) - 1;
                self.jmp(ret_addr);
            }, 
            RTS => {
                let ret_addr = self.pull_return_addr(interconnect);
                self.jmp(ret_addr);
            },

            // Bit tests
            BITZpg => {
                let val = self.zero_page(interconnect);
                self.bit(val);
                self.bump_pc(2);
            },
            BITAbs => {
                let val = self.absolute(interconnect);
                self.bit(val);
                self.bump_pc(3);
            }, 

            // ALU operations
            ORAInxX => {
                let val = self.indexed_indirect(interconnect);
                self.ora(val);
                self.bump_pc(2);
            }, 
            ORAZPg=> {
                let val = self.zero_page(interconnect);
                self.ora(val);
                self.bump_pc(2);
            }, 
            ORAImm => {
                let imm = self.immediate(interconnect);
                self.ora(imm);
                self.bump_pc(2);
            },
            ORAAbs => {
                let val = self.absolute(interconnect);
                self.ora(val);
                self.bump_pc(3);
            }, 
            ORAIndY => {
                let val = self.indirect_indexed(interconnect);
                self.ora(val);
                self.bump_pc(2);
            }, 
            ORAZPgX  => {
                let val = self.z_page_indexed(interconnect, CPURegister::X);
                self.ora(val);
                self.bump_pc(2);
            }, 
            ORAAx  => {
                let val = self.absolute_indexed(interconnect, CPURegister::X);
                self.ora(val);
                self.bump_pc(3);
            }, 
            ORAAy  => {
                let val = self.absolute_indexed(interconnect, CPURegister::Y);
                self.ora(val);
                self.bump_pc(3);
            }, 

            ANDInxX => {
                let val = self.indexed_indirect(interconnect);
                self.and(val);
                self.bump_pc(2);
            }, 
            ANDZPg=> {
                let val = self.zero_page(interconnect);
                self.and(val);
                self.bump_pc(2);
            }, 
            ANDImm => {
                let val = self.immediate(interconnect);
                self.and(val);
                self.bump_pc(2);
            },
            ANDAbs => {
                let val = self.absolute(interconnect);
                self.and(val);
                self.bump_pc(3);
            }, 
            ANDIndY => {
                let val = self.indirect_indexed(interconnect);
                self.and(val);
                self.bump_pc(2);
            }, 
            ANDZPgX  => {
                let val = self.z_page_indexed(interconnect, CPURegister::X);
                self.and(val);
                self.bump_pc(2);
            }, 
            ANDAx  => {
                let val = self.absolute_indexed(interconnect, CPURegister::X);
                self.and(val);
                self.bump_pc(3);
            }, 
            ANDAy  => {
                let val = self.absolute_indexed(interconnect, CPURegister::Y);
                self.and(val);
                self.bump_pc(3);
            }, 

            EORInxX => {
                let val = self.indexed_indirect(interconnect);
                self.eor(val);
                self.bump_pc(2);
            }, 
            EORZPg=> {
                let val = self.zero_page(interconnect);
                self.eor(val);
                self.bump_pc(2);
            }, 
            EORImm => {
                let val = self.immediate(interconnect);
                self.eor(val);
                self.bump_pc(2);
            }, 
            EORAbs => {
                let val = self.absolute(interconnect);
                self.eor(val);
                self.bump_pc(3);
            }, 
            EORIndY => {
                let val = self.indirect_indexed(interconnect);
                self.eor(val);
                self.bump_pc(2);
            }, 
            EORZPgX  => {
                let val = self.z_page_indexed(interconnect, CPURegister::X);
                self.eor(val);
                self.bump_pc(2);
            }, 
            EORAx  => {
                let val = self.absolute_indexed(interconnect, CPURegister::X);
                self.eor(val);
                self.bump_pc(3);
            }, 
            EORAy  => {
                let val = self.absolute_indexed(interconnect, CPURegister::Y);
                self.eor(val);
                self.bump_pc(3);
            }, 

            ADCInxX => {
                let val = self.indexed_indirect(interconnect);
                self.add(val);
                self.bump_pc(2);
            }, 
            ADCZPg=> {
                let val = self.zero_page(interconnect);
                self.add(val);
                self.bump_pc(2);
            }, 
            ADCImm => {
                let val = self.immediate(interconnect);
                self.add(val);
                self.bump_pc(2);
            },
            ADCAbs => {
                let val = self.absolute(interconnect);
                self.add(val);
                self.bump_pc(3);
            }, 
            ADCIndY => {
                let val = self.indirect_indexed(interconnect);
                self.add(val);
                self.bump_pc(2);
            }, 
            ADCZPgX  => {
                let val = self.z_page_indexed(interconnect, CPURegister::X);
                self.add(val);
                self.bump_pc(2);
            }, 
            ADCAx  => {
                let val = self.absolute_indexed(interconnect, CPURegister::X);
                self.add(val);
                self.bump_pc(3);
            }, 
            ADCAy  => {
                let val = self.absolute_indexed(interconnect, CPURegister::Y);
                self.add(val);
                self.bump_pc(3);
            }, 

            CMPInxX => {
                let val = self.indexed_indirect(interconnect);
                self.compare(CPURegister::A, val);
                self.bump_pc(2);
            },
            CMPZPg=> {
                let val = self.zero_page(interconnect);
                self.compare(CPURegister::A, val);
                self.bump_pc(2);
            }, 
            CMPImm => {
                let val = self.immediate(interconnect);
                self.compare(CPURegister::A, val);
                self.bump_pc(2);
            }, 
            CMPAbs => {
                let val = self.absolute(interconnect);
                self.compare(CPURegister::A, val);
                self.bump_pc(3);
            }, 
            CMPIndY => {
                let val = self.indirect_indexed(interconnect);
                self.compare(CPURegister::A, val);
                self.bump_pc(2);
            },
            CMPZPgX  => {
                let val = self.z_page_indexed(interconnect, CPURegister::X);
                self.compare(CPURegister::A, val);
                self.bump_pc(2);
            }, 
            CMPAx  => {
                let val = self.absolute_indexed(interconnect, CPURegister::X);
                self.compare(CPURegister::A, val);
                self.bump_pc(3);
            }, 
            CMPAy  => {
                let val = self.absolute_indexed(interconnect, CPURegister::Y);
                self.compare(CPURegister::A, val);
                self.bump_pc(3);
            }, 

            SBCInxX=> {
                let val = self.indexed_indirect(interconnect);
                self.sub(val);
                self.bump_pc(2);
            }, 
            SBCZPg=> {
                let val = self.zero_page(interconnect);
                self.sub(val);
                self.bump_pc(2);
            }, 
            SBCImm => {
                let val = self.immediate(interconnect);
                self.sub(val);
                self.bump_pc(2);
            }, 
            SBCAbs => {
                let val = self.absolute(interconnect);
                self.sub(val);
                self.bump_pc(3);
            }, 
            SBCIndY => {
                let val = self.indirect_indexed(interconnect);
                self.sub(val);
                self.bump_pc(2);
            }, 
            SBCZPgX  => {
                let val = self.z_page_indexed(interconnect, CPURegister::X);
                self.sub(val);
                self.bump_pc(2);
            }, 
            SBCAx  => {
                let val = self.absolute_indexed(interconnect, CPURegister::X);
                self.sub(val);
                self.bump_pc(3);
            }, 
            SBCAy  => {
                let val = self.absolute_indexed(interconnect, CPURegister::Y);
                self.sub(val);
                self.bump_pc(3);
            }, 
                 
            ASLZPg => {
                let addr = interconnect.read_byte(self.pc + 1) as u16;
                let mut val = self.zero_page(interconnect);
                if val & (1 << 7) != 0 {
                    self.set_flag(CARRY_FLAG);
                } else {
                    self.unset_flag(CARRY_FLAG);
                }
                val = val << 1;
                if val  & 0b1000_0000 != 0 {
                    self.set_flag(NEGATIVE_FLAG);
                } else {
                    self.unset_flag(NEGATIVE_FLAG);
                };
                if val  == 0 {
                    self.set_flag(ZERO_FLAG);
                } else {
                    self.unset_flag(ZERO_FLAG);
                };
                interconnect.write_byte(addr, val);
                self.bump_pc(2);
            }
            ASL    => {
                let mut val = self.read_reg(CPURegister::A);
                if val & (1 << 7) != 0 {
                    self.set_flag(CARRY_FLAG);
                } else {
                    self.unset_flag(CARRY_FLAG);
                }
                val = val << 1;
                if val  & 0b1000_0000 != 0 {
                    self.set_flag(NEGATIVE_FLAG);
                } else {
                    self.unset_flag(NEGATIVE_FLAG);
                };
                if val  == 0 {self.set_flag(ZERO_FLAG);
                } else {
                    self.unset_flag(ZERO_FLAG);
                };
                self.write_to_reg(CPURegister::A, val);
                self.bump_pc(1);
            }, 
            ASLAbs => {
                let addr = interconnect.read_word(self.pc + 1);
                let mut val = self.absolute(interconnect);
                if val & (1 << 7) != 0 {
                    self.set_flag(CARRY_FLAG);
                } else {
                    self.unset_flag(CARRY_FLAG);
                }
                val = val << 1;
                if val  & 0b1000_0000 != 0 {
                    self.set_flag(NEGATIVE_FLAG);
                } else {
                    self.unset_flag(NEGATIVE_FLAG);
                };
                if val  == 0 {
                    self.set_flag(ZERO_FLAG);
                } else {
                    self.unset_flag(ZERO_FLAG);
                };
                interconnect.write_byte(addr, val);
                self.bump_pc(3);
            }, 
            ASLZPgX  => {
                let addr = self.z_page_indexed(interconnect, CPURegister::X);
                let mut val = self.z_page_indexed(interconnect, CPURegister::X);
                if val & (1 << 7) != 0 {self.set_flag(CARRY_FLAG);} else {self.unset_flag(CARRY_FLAG);}
                val = val << 1;
                if val  & 0b1000_0000 != 0 {
                    self.set_flag(NEGATIVE_FLAG);
                } else {
                    self.unset_flag(NEGATIVE_FLAG);
                };
                if val  == 0 {self.set_flag(ZERO_FLAG);} else {self.unset_flag(ZERO_FLAG);};
                interconnect.write_byte(addr, val);
                self.bump_pc(2);
            }, 
            ASLAx  => {
                let addr = self.absolute_indexed(interconnect, CPURegister::X);
                let mut val = self.absolute_indexed(interconnect, CPURegister::X);
                if val & (1 << 7) != 0 {
                    self.set_flag(CARRY_FLAG);
                } else {
                    self.unset_flag(CARRY_FLAG);
                }
                val = val << 1;
                if val  & 0b1000_0000 != 0 {
                    self.set_flag(NEGATIVE_FLAG);
                } else {
                    self.unset_flag(NEGATIVE_FLAG);
                };
                if val  == 0 {
                    self.set_flag(ZERO_FLAG);
                } else {
                    self.unset_flag(ZERO_FLAG);
                };
                interconnect.write_byte(addr, val);
                self.bump_pc(3);
            }, 
            
            LSRZPg => {
                let addr = interconnect.read_byte(self.pc + 1) as u16;
                let mut val = self.zero_page(interconnect);
                if val & (1 << 0) != 0 {
                    self.set_flag(CARRY_FLAG);
                } else {
                    self.unset_flag(CARRY_FLAG);
                }
                val = val >> 1;
                if val  & 0b1000_0000 != 0 {
                    self.set_flag(NEGATIVE_FLAG);
                } else {
                    self.unset_flag(NEGATIVE_FLAG);
                };
                if val  == 0 {
                    self.set_flag(ZERO_FLAG);
                } else {
                    self.unset_flag(ZERO_FLAG);
                };
                interconnect.write_byte(addr, val);
                self.bump_pc(2);
            }, 
            LSR    => {
                let mut val = self.read_reg(CPURegister::A);
                if val & (1 << 0) != 0 {
                    self.set_flag(CARRY_FLAG);
                } else {
                    self.unset_flag(CARRY_FLAG);
                }
                val = val >> 1;
                if val  & 0b1000_0000 != 0 {
                    self.set_flag(NEGATIVE_FLAG);
                } else {
                    self.unset_flag(NEGATIVE_FLAG);
                };
                if val  == 0 {
                    self.set_flag(ZERO_FLAG);
                } else {
                    self.unset_flag(ZERO_FLAG);
                };
                self.write_to_reg(CPURegister::A, val);
                self.bump_pc(1);}, 
            LSRAbs => {
                let addr = interconnect.read_word(self.pc + 1);
                let mut val = self.absolute(interconnect);
                if val & (1 << 0) != 0 {
                    self.set_flag(CARRY_FLAG);
                } else {
                    self.unset_flag(CARRY_FLAG);
                }
                val = val >> 1;
                if val  & 0b1000_0000 != 0 {
                    self.set_flag(NEGATIVE_FLAG);
                } else {
                    self.unset_flag(NEGATIVE_FLAG);
                };
                if val  == 0 {
                    self.set_flag(ZERO_FLAG);
                } else {
                    self.unset_flag(ZERO_FLAG);
                };
                interconnect.write_byte(addr, val);
                self.bump_pc(3);
            }, 
            LSRZPgX  => {
                let addr = self.z_page_indexed(interconnect, CPURegister::X);
                let mut val = self.z_page_indexed(interconnect, CPURegister::X);
                if val & (1 << 0) != 0 {
                    self.set_flag(CARRY_FLAG);
                } else {
                    self.unset_flag(CARRY_FLAG);
                }
                val = val >> 1;
                if val  & 0b1000_0000 != 0 {
                    self.set_flag(NEGATIVE_FLAG);
                } else {
                    self.unset_flag(NEGATIVE_FLAG);
                };
                if val  == 0 {
                    self.set_flag(ZERO_FLAG);
                } else {
                    self.unset_flag(ZERO_FLAG);
                };
                interconnect.write_byte(addr, val);
                self.bump_pc(2);
            }, 
            LSRAx  => {
                let addr = self.absolute_indexed(interconnect, CPURegister::X);
                let mut val = self.absolute_indexed(interconnect, CPURegister::X);
                if val & (1 << 0) != 0 {
                    self.set_flag(CARRY_FLAG);
                } else {
                    self.unset_flag(CARRY_FLAG);
                }
                val = val >> 1;
                if val  & 0b1000_0000 != 0 {
                    self.set_flag(NEGATIVE_FLAG);
                } else {
                             self.unset_flag(NEGATIVE_FLAG);
                };
                if val  == 0 {
                    self.set_flag(ZERO_FLAG);
                } else {
                    self.unset_flag(ZERO_FLAG);
                };
                interconnect.write_byte(addr, val);
                self.bump_pc(3);
            }, 
            
            // Rotates
            ROLZPg=> {
                let addr = interconnect.read_byte(self.pc + 1) as u16;
                let mut val = self.zero_page(interconnect);
                let carry = if self.check_flag(CARRY_FLAG) {1} else {0};
                if val & (1 << 7) != 0 {
                    self.set_flag(CARRY_FLAG);
                } else {
                    self.unset_flag(CARRY_FLAG);
                }
                val = (val << 1) | carry;
                if val  & 0b1000_0000 != 0 {
                    self.set_flag(NEGATIVE_FLAG);
                } else {
                    self.unset_flag(NEGATIVE_FLAG);
                };
                if val  == 0 {
                    self.set_flag(ZERO_FLAG);
                } else {
                    self.unset_flag(ZERO_FLAG);
                };
                interconnect.write_byte(addr, val);
                self.bump_pc(2);
            },
            ROL     => {
                let mut val = self.read_reg(CPURegister::A);
                let carry = if self.check_flag(CARRY_FLAG) {1} else {0};
                if val & (1 << 7) != 0 {
                    self.set_flag(CARRY_FLAG);
                } else {
                    self.unset_flag(CARRY_FLAG);
                }
                val = (val << 1) | carry;
                if val  & 0b1000_0000 != 0 {
                    self.set_flag(NEGATIVE_FLAG);
                } else {
                    self.unset_flag(NEGATIVE_FLAG);
                };
                if val  == 0 {
                    self.set_flag(ZERO_FLAG);
                } else {
                    self.unset_flag(ZERO_FLAG);
                };
                self.write_to_reg(CPURegister::A, val);
                self.bump_pc(1);
            }, 
            ROLAbs => {
                let addr = interconnect.read_word(self.pc + 1);
                let mut val = self.absolute(interconnect);
                let carry = if self.check_flag(CARRY_FLAG) {1} else {0};
                if val & (1 << 7) != 0 {
                    self.set_flag(CARRY_FLAG);
                } else {
                    self.unset_flag(CARRY_FLAG);
                }
                val = (val << 1) | carry;
                if val  & 0b1000_0000 != 0 {
                    self.set_flag(NEGATIVE_FLAG);
                } else {
                    self.unset_flag(NEGATIVE_FLAG);
                };
                if val  == 0 {
                    self.set_flag(ZERO_FLAG);
                } else {
                    self.unset_flag(ZERO_FLAG);
                };
                interconnect.write_byte(addr, val);
                self.bump_pc(3);
            }, 
            ROLZPgX  => {
                let addr = self.z_page_indexed(interconnect, CPURegister::X);
                let mut val = self.z_page_indexed(interconnect, CPURegister::X);
                let carry = if self.check_flag(CARRY_FLAG) {1} else {0};
                if val & (1 << 7) != 0 {self.set_flag(CARRY_FLAG);} else {self.unset_flag(CARRY_FLAG);}
                val = (val << 1) | carry;
                if val  & 0b1000_0000 != 0 {
                    self.set_flag(NEGATIVE_FLAG);
                } else {
                    self.unset_flag(NEGATIVE_FLAG);
                };
                if val  == 0 {self.set_flag(ZERO_FLAG);} else {self.unset_flag(ZERO_FLAG);};
                interconnect.write_byte(addr, val);
                self.bump_pc(2);}, 
            ROLAx  => {
                let addr = self.absolute_indexed(interconnect, CPURegister::X);
                let mut val = self.absolute_indexed(interconnect, CPURegister::X);
                let carry = if self.check_flag(CARRY_FLAG) {1} else {0};
                if val & (1 << 7) != 0 {
                    self.set_flag(CARRY_FLAG);
                } else {
                    self.unset_flag(CARRY_FLAG);
                }
                val = (val << 1) | carry;
                if val  & 0b1000_0000 != 0 {
                    self.set_flag(NEGATIVE_FLAG);
                } else {
                    self.unset_flag(NEGATIVE_FLAG);
                };
                if val  == 0 {
                    self.set_flag(ZERO_FLAG);
                } else {
                    self.unset_flag(ZERO_FLAG);
                };
                interconnect.write_byte(addr, val);
                self.bump_pc(3);
            }, 

            RORZPg=> {
                let addr = interconnect.read_byte(self.pc + 1) as u16;
                let mut val = self.zero_page(interconnect);
                let carry = if self.check_flag(CARRY_FLAG) {1 << 7} else {0};
                if val & (1 << 0) != 0 {
                    self.set_flag(CARRY_FLAG);
                } else {
                    self.unset_flag(CARRY_FLAG);
                }
                val = (val >> 1) | carry;
                if val  & 0b1000_0000 != 0 {
                    self.set_flag(NEGATIVE_FLAG);
                } else {
                    self.unset_flag(NEGATIVE_FLAG);
                };
                if val  == 0 {
                    self.set_flag(ZERO_FLAG);
                } else {
                    self.unset_flag(ZERO_FLAG);
                };
                interconnect.write_byte(addr, val);
                self.bump_pc(2);
            }, 
            ROR     => {
                let mut val = self.read_reg(CPURegister::A);
                let carry = if self.check_flag(CARRY_FLAG) {1 << 7} else {0};
                if val & (1 << 0) != 0 {
                    self.set_flag(CARRY_FLAG);
                } else {
                    self.unset_flag(CARRY_FLAG);
                }
                val = (val >> 1) | carry;
                if val  & 0b1000_0000 != 0 {
                    self.set_flag(NEGATIVE_FLAG);
                } else {
                    self.unset_flag(NEGATIVE_FLAG);
                };
                if val  == 0 {self.set_flag(ZERO_FLAG);} else {self.unset_flag(ZERO_FLAG);};
                self.write_to_reg(CPURegister::A, val);
                self.bump_pc(1);
            }, 
            RORAbs => {
                let addr = interconnect.read_word(self.pc + 1);
                let mut val = self.absolute(interconnect);
                let carry = if self.check_flag(CARRY_FLAG) {1 << 7} else {0};
                if val & (1 << 0) != 0 {
                    self.set_flag(CARRY_FLAG);
                } else {
                    self.unset_flag(CARRY_FLAG);
                }
                val = (val >> 1) | carry;
                if val  & 0b1000_0000 != 0 {
                    self.set_flag(NEGATIVE_FLAG);
                } else {
                    self.unset_flag(NEGATIVE_FLAG);
                };
                if val  == 0 {
                    self.set_flag(ZERO_FLAG);
                } else {
                    self.unset_flag(ZERO_FLAG);
                };
                interconnect.write_byte(addr, val);
                self.bump_pc(3);
            }, 
            RORZPgX  => {
                let addr = self.z_page_indexed(interconnect, CPURegister::X);
                let mut val = self.z_page_indexed(interconnect, CPURegister::X);
                let carry = if self.check_flag(CARRY_FLAG) {1 << 7} else {0};
                if val & (1 << 0) != 0 {
                    self.set_flag(CARRY_FLAG);
                } else {
                    self.unset_flag(CARRY_FLAG);
                }
                val = (val >> 1) | carry;
                if val  & 0b1000_0000 != 0 {
                    self.set_flag(NEGATIVE_FLAG);
                } else {
                    self.unset_flag(NEGATIVE_FLAG);
                };
                if val  == 0 {
                    self.set_flag(ZERO_FLAG);
                } else {
                    self.unset_flag(ZERO_FLAG);
                };
                interconnect.write_byte(addr, val);
                self.bump_pc(2);
            }, 
            RORAx  => {
                let addr = self.absolute_indexed(interconnect, CPURegister::X);
                let mut val = self.absolute_indexed(interconnect, CPURegister::X);
                let carry = if self.check_flag(CARRY_FLAG) {1 << 7} else {0};
                if val & (1 << 0) != 0 {
                    self.set_flag(CARRY_FLAG);
                } else {
                    self.unset_flag(CARRY_FLAG);
                }
                val = (val >> 1) | carry;
                if val  & 0b1000_0000 != 0 {
                    self.set_flag(NEGATIVE_FLAG);
                } else {
                    self.unset_flag(NEGATIVE_FLAG);
                };
                if val  == 0 {
                    self.set_flag(ZERO_FLAG);
                } else {
                    self.unset_flag(ZERO_FLAG);
                };
                interconnect.write_byte(addr, val);
                self.bump_pc(3);
            }, 

            // Increments
            DECZpg => {
                let addr = interconnect.read_byte(self.pc + 1);
                let temp_val = interconnect.read_byte(addr as u16);
                let val = temp_val.wrapping_sub(1);
                if val > temp_val {
                    self.set_flag(OVERFLOW_FLAG);
                } else {
                    self.unset_flag(OVERFLOW_FLAG);
                }
                if val  & 0b1000_0000 != 0 {
                    self.set_flag(NEGATIVE_FLAG);
                } else {
                    self.unset_flag(NEGATIVE_FLAG);
                };
                if val  == 0 {
                    self.set_flag(ZERO_FLAG);
                } else {
                    self.unset_flag(ZERO_FLAG);
                };
                interconnect.write_byte(addr as u16, val);
                self.bump_pc(2);
            },
            DECAbs => {
                let addr = interconnect.read_word(self.pc + 1);
                let temp_val = interconnect.read_byte(addr);
                let val = temp_val.wrapping_sub(1);
                if val > temp_val {
                    self.set_flag(OVERFLOW_FLAG);
                } else {
                    self.unset_flag(OVERFLOW_FLAG);
                }
                if val  & 0b1000_0000 != 0 {
                    self.set_flag(NEGATIVE_FLAG);
                } else {
                    self.unset_flag(NEGATIVE_FLAG);
                };
                if val  == 0 {
                    self.set_flag(ZERO_FLAG);
                } else {
                    self.unset_flag(ZERO_FLAG);
                };
                interconnect.write_byte(addr, val);
                self.bump_pc(3);
            }, 
            DECZPgX  => {
                let addr = self.z_page_indexed(interconnect, CPURegister::X);
                let temp_val = interconnect.read_byte(addr);
                let val = temp_val.wrapping_sub(1);
                if val > temp_val {
                    self.set_flag(OVERFLOW_FLAG);
                } else {
                    self.unset_flag(OVERFLOW_FLAG);
                }
                if val  & 0b1000_0000 != 0 {
                    self.set_flag(NEGATIVE_FLAG);
                } else {
                    self.unset_flag(NEGATIVE_FLAG);
                };
                if val  == 0 {
                    self.set_flag(ZERO_FLAG);
                } else {
                    self.unset_flag(ZERO_FLAG);
                };
                interconnect.write_byte(addr, val);
                self.bump_pc(2);
            }, 
            DECAx  => {
                let addr = self.absolute_indexed(interconnect, CPURegister::X);
                let temp_val = interconnect.read_byte(addr);
                let val = temp_val.wrapping_sub(1);
                if val > temp_val {
                    self.set_flag(OVERFLOW_FLAG);
                } else {
                    self.unset_flag(OVERFLOW_FLAG);
                }
                if val  & 0b1000_0000 != 0 {
                    self.set_flag(NEGATIVE_FLAG);
                } else {
                    self.unset_flag(NEGATIVE_FLAG);
                };
                if val  == 0 {
                    self.set_flag(ZERO_FLAG);
                } else {
                    self.unset_flag(ZERO_FLAG);
                };
                interconnect.write_byte(addr, val);
                self.bump_pc(3);
            }, 

            INCZPg=> {
                let addr = interconnect.read_byte(self.pc + 1);
                let temp_val = interconnect.read_byte(addr as u16);
                let val = temp_val.wrapping_add(1);
                if val < temp_val {
                    self.set_flag(OVERFLOW_FLAG);
                } else {
                    self.unset_flag(OVERFLOW_FLAG);
                }
                if val  & 0b1000_0000 != 0 {
                    self.set_flag(NEGATIVE_FLAG);
                } else {
                    self.unset_flag(NEGATIVE_FLAG);
                };
                if val  == 0 {
                    self.set_flag(ZERO_FLAG);
                } else {
                    self.unset_flag(ZERO_FLAG);
                };
                interconnect.write_byte(addr as u16, val);
                self.bump_pc(2);
            }, 
            INCAbs => {
                let addr = interconnect.read_word(self.pc + 1);
                let temp_val = interconnect.read_byte(addr);
                let val = temp_val.wrapping_add(1);
                if val < temp_val {
                    self.set_flag(OVERFLOW_FLAG);
                } else {
                    self.unset_flag(OVERFLOW_FLAG);
                }
                if val  & 0b1000_0000 != 0 {
                    self.set_flag(NEGATIVE_FLAG);
                } else {
                    self.unset_flag(NEGATIVE_FLAG);
                };
                if val  == 0 {
                    self.set_flag(ZERO_FLAG);
                } else {
                    self.unset_flag(ZERO_FLAG);
                };
                interconnect.write_byte(addr, val);
                self.bump_pc(3);
            }, 
            INCZPgX  => {
                let addr = self.z_page_indexed(interconnect, CPURegister::X);
                let temp_val = interconnect.read_byte(addr);
                let val = temp_val.wrapping_add(1);
                if val < temp_val {
                    self.set_flag(OVERFLOW_FLAG);
                } else {
                    self.unset_flag(OVERFLOW_FLAG);
                }
                if val  & 0b1000_0000 != 0 {
                    self.set_flag(NEGATIVE_FLAG);
                } else {
                    self.unset_flag(NEGATIVE_FLAG);
                };
                if val  == 0 {
                    self.set_flag(ZERO_FLAG);
                } else {
                    self.unset_flag(ZERO_FLAG);
                };
                interconnect.write_byte(addr, val);
                self.bump_pc(2);
            }, 
            INCAx  => {
                let addr = self.absolute_indexed(interconnect, CPURegister::X);
                let temp_val = interconnect.read_byte(addr);
                let val = temp_val.wrapping_add(1);
                if val < temp_val {
                    self.set_flag(OVERFLOW_FLAG);
                } else {
                    self.unset_flag(OVERFLOW_FLAG);
                }
                if val  & 0b1000_0000 != 0 {
                    self.set_flag(NEGATIVE_FLAG);
                } else {
                    self.unset_flag(NEGATIVE_FLAG);
                };
                if val  == 0 {
                    self.set_flag(ZERO_FLAG);
                } else {
                    self.unset_flag(ZERO_FLAG);
                };
                interconnect.write_byte(addr, val);
                self.bump_pc(3);
            }, 
            
            // The ever important nop
            // Observe all its majesty
            NOP => {self.bump_pc(1);},
        }
    }

    // Addressing modes
    fn immediate(&self, interconnect: &Interconnect) -> u8 {
        interconnect.read_byte(self.pc + 1)
    }

    fn absolute(&self, interconnect: &Interconnect) -> u16 {
        interconnect.read_word(self.pc + 1);
    }

    fn zero_page(&self, interconnect: &Interconnect) -> u16 {
        interconnect.read_byte(self.pc + 1) as u16;
    }

    // TODO: Check for CPURegister::{X or Y}
    fn absolute_indexed(&self, interconnect: &Interconnect, register: CPURegister) -> u16 {
        let addr = interconnect.read_word(self.pc + 1);
        addr.wrapping_add((self.read_reg(register) as u16))
    }

    // TODO: Check for CPURegister::{X or Y}
    fn z_page_indexed(&self, interconnect: &Interconnect, register: CPURegister) -> u16 {
        let addr = interconnect.read_byte(self.pc + 1);
        addr.wrapping_add(self.read_reg(register)) as u16
    }

    // PRETTIFYME
    fn indexed_indirect(&self, interconnect: &Interconnect) -> u16 {
        let addr = interconnect.read_byte(self.pc + 1);
        let sum_addr = addr.wrapping_add((self.read_reg(CPURegister::X))) as u16;
        if sum_addr == 0xff {
            (interconnect.read_byte(sum_addr) as u16) |
            ((interconnect.read_byte(0x0000) as u16) << 8)
        } else {
            interconnect.read_word(sum_addr as u16)
        }
    }

    fn indirect_indexed(&self, interconnect: &Interconnect) -> u16 {
        let temp_addr = interconnect.read_byte(self.pc + 1) as u16;
        if temp_addr == 0xff {
            let mut addr = interconnect.read_byte(temp_addr) as u16 |
            ((interconnect.read_byte(0x0000) as u16) << 8);
            addr.wrapping_add(self.read_reg(CPURegister::Y) as u16)
        } else {
            let mut addr = interconnect.read_word(temp_addr);
            addr.wrapping_add(self.read_reg(CPURegister::Y) as u16)
        }
    }

    // Instruction abstractions
    // PRETTIFYME: This is a kludge
    fn add(&mut self, arg: u8) {
        let a = self.read_reg(CPURegister::A) as u16;
        let mut sum = a + arg as u16;
        if self.check_flag(CARRY_FLAG) {sum += 1;};
        if sum > 255 {
            self.set_flag(CARRY_FLAG)} else {self.unset_flag(CARRY_FLAG);
        }
        if !((a as u8) ^ arg) & ((a as u8) ^ sum as u8) & 0x80 != 0 {
            self.set_flag(OVERFLOW_FLAG)} else {self.unset_flag(OVERFLOW_FLAG)}
        if (sum & 0xff) as u8 & (1 << 7) != 0 {
            self.set_flag(NEGATIVE_FLAG)} else {self.unset_flag(NEGATIVE_FLAG);
        }
        if ((sum & 0xff) as u8) == 0 {
            self.set_flag(ZERO_FLAG)} else {self.unset_flag(ZERO_FLAG);
        }
        self.write_to_reg(CPURegister::A, sum as u8);
    }

    fn and(&mut self, arg: u8) {
        let a = self.read_reg(CPURegister::A);
        self.write_to_reg(CPURegister::A, a & arg);
        if self.read_reg(CPURegister::A) & 0b1000_0000 != 0 {
            self.set_flag(NEGATIVE_FLAG);
        } else {
            self.unset_flag(NEGATIVE_FLAG);
        };
        if self.read_reg(CPURegister::A) == 0 {
            self.set_flag(ZERO_FLAG);
        } else {
            self.unset_flag(ZERO_FLAG);
        };
    }

    fn bit(&mut self, arg: u8) {
        if arg & self.read_reg(CPURegister::A) == 0 {
            self.set_flag(ZERO_FLAG);
        } else {
            self.unset_flag(ZERO_FLAG);
        }
        if arg & (1 << 6) != 0 {
            self.set_flag(OVERFLOW_FLAG)} else {self.unset_flag(OVERFLOW_FLAG);
        }
        if arg & (1 << 7) != 0 {
            self.set_flag(NEGATIVE_FLAG)} else {self.unset_flag(NEGATIVE_FLAG);
        }
    }

    fn branch(&mut self, interconnect: &Interconnect, branch_on: BranchOn) {
        let branch_target = interconnect.read_byte(self.pc + 1) as i8;
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
        if branch {
            self.pc = ((pc as i32) + (branch_target as i32)) as u16} else {self.bump_pc(2);
        }
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
        if eval & 0b1000_0000 != 0 {
            self.set_flag(NEGATIVE_FLAG);
        } else {
            self.unset_flag(NEGATIVE_FLAG);
        };
        if eval == 0 {
            self.set_flag(ZERO_FLAG);
        } else {
            self.unset_flag(ZERO_FLAG);
        };
        eval
    }
    fn eor(&mut self, arg: u8) {
        let a = self.read_reg(CPURegister::A);
        self.write_to_reg(CPURegister::A, a ^ arg);
        if self.read_reg(CPURegister::A) & 0b1000_0000 != 0 {
            self.set_flag(NEGATIVE_FLAG);
        } else {
            self.unset_flag(NEGATIVE_FLAG);
        };
        if self.read_reg(CPURegister::A) == 0 {
            self.set_flag(ZERO_FLAG);
        } else {
            self.unset_flag(ZERO_FLAG);
        };
    }

    fn increment(&mut self, arg: u8) -> u8 {
        let eval = arg.wrapping_add(1);
        if eval & 0b1000_0000 != 0 {
            self.set_flag(NEGATIVE_FLAG);
        } else {
            self.unset_flag(NEGATIVE_FLAG);
        };
        if eval == 0 {
            self.set_flag(ZERO_FLAG);
        } else {
            self.unset_flag(ZERO_FLAG);
        };
        eval
    }

    fn jmp(&mut self, jump_target: u16) {
        self.pc = jump_target;
    }

    fn load(&mut self, register: CPURegister, arg: u8) {
        self.write_to_reg(register, arg);
        if self.read_reg(register) & 0b1000_0000 != 0 {
            self.set_flag(NEGATIVE_FLAG);
        } else {
            self.unset_flag(NEGATIVE_FLAG);
        };
        if self.read_reg(register) == 0 {
            self.set_flag(ZERO_FLAG);
        } else {
            self.unset_flag(ZERO_FLAG);
        };
    }

    fn ora(&mut self, arg: u8) {
        let a = self.read_reg(CPURegister::A);
        self.write_to_reg(CPURegister::A, a | arg);
        if self.read_reg(CPURegister::A) & 0b1000_0000 != 0 {
            self.set_flag(NEGATIVE_FLAG);
        } else {
            self.unset_flag(NEGATIVE_FLAG);
        };
        if self.read_reg(CPURegister::A) == 0 {
            self.set_flag(ZERO_FLAG);
        } else {
            self.unset_flag(ZERO_FLAG);
        };
    }

    fn transfer(&mut self, from_reg: CPURegister, to_reg: CPURegister) {
        let val = self.read_reg(from_reg);
        self.write_to_reg(to_reg, val);
        if to_reg != CPURegister::StackPointer {
            if self.read_reg(to_reg) & 0b1000_0000 != 0 {
                self.set_flag(NEGATIVE_FLAG);
            } else {
                self.unset_flag(NEGATIVE_FLAG);
            };
            if self.read_reg(to_reg) == 0 {
                self.set_flag(ZERO_FLAG);
            } else {
                self.unset_flag(ZERO_FLAG);
            };
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

