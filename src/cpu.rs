use std::fmt;
use super::interconnect::Interconnect;
use super::instructions::Instruction;
use super::ppu::ppu::Ppu;

const NEGATIVE_FLAG:    u8 = 1 << 7;
const OVERFLOW_FLAG:    u8 = 1 << 6;
const STACK_COPY:       u8 = 1 << 5;
const BRK_FLAG:         u8 = 1 << 4;
const DECIMAL_FLAG:     u8 = 1 << 3;
const IRQ_INHIBIT_FLAG: u8 = 1 << 2;
const ZERO_FLAG:        u8 = 1 << 1;
const CARRY_FLAG:       u8 = 1 << 0;

const NMI_VECTOR: u16 = 0xfffa;
const RESET_VECTOR: u16 = 0xfffc;
const IRQBRK_VECTOR: u16 = 0xfffe;

pub enum Interrupt {
    NMI,
    RESET,
    IRQ,
    BRK,
}

#[derive(Default)]
pub struct Cpu {
    a: u8, // Accumulator

    x: u8, // x-Index
    y: u8, // y-index

    pc: u16, // Program counter
    
    stack_pointer: u8, // Stack pointer

    status: u8, // Status register

    pub cycles: u64,
}

#[derive(Debug)]
#[allow(dead_code)]
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

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            a: 0,

            x: 0,
            y: 0,

            pc: 0,

            stack_pointer: 0,

            status: 0,

            cycles: 0,
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

    fn push_byte_stack(&mut self, interconnect: &mut Interconnect, ppu: &mut Ppu, byte: u8) {
        let addr = self.read_reg(CPURegister::StackPointer) as u16 + 0x100;
        interconnect.write_byte(ppu, addr, byte);
        self.stack_pointer -= 1;
    }

    fn pull_byte_stack(&mut self, interconnect: &Interconnect, ppu: &Ppu, register: CPURegister) {
        self.stack_pointer += 1;
        let addr = self.read_reg(CPURegister::StackPointer) as u16 + 0x100;
        self.write_to_reg(register, interconnect.read_byte(ppu, addr));
        self.set_flag(STACK_COPY);
    }

    fn push_return_addr(&mut self, interconnect: &mut Interconnect, ppu: &mut Ppu) {
        let pc = self.pc + 2;
        let pc_msb = ((pc & 0xff00) >> 8) as u8;
        let pc_lsb = (pc & 0x00ff) as u8;
        self.push_byte_stack(interconnect, ppu, pc_msb);
        self.push_byte_stack(interconnect, ppu, pc_lsb);
    }

    fn pull_return_addr(&mut self, interconnect: &mut Interconnect, ppu: &Ppu) -> u16 {
        self.stack_pointer += 1;
        let addr = self.read_reg(CPURegister::StackPointer) as u16 + 0x100;
        let pc_lsb = interconnect.read_byte(ppu, addr) as u16;
        self.stack_pointer += 1;
        let addr = self.read_reg(CPURegister::StackPointer) as u16 + 0x100;
        let pc_msb = (interconnect.read_byte(ppu, addr) as u16) << 8;
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

    pub fn run_instr(&mut self, interconnect: &mut Interconnect, ppu: &mut Ppu) {
        use enum_primitive::FromPrimitive;
        use instructions::Instruction::*;
        let raw_instr = interconnect.read_byte(ppu, self.pc);
        let instr = Instruction::from_u8(raw_instr).unwrap_or_else(|| {
            panic!("Unrecognized instruction: {:#x}", raw_instr)
        });
        println!("{:04X} {:?} \t A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X} CYCLE: {:?}",
                 self.pc, instr, self.a, self.x, self.y, self.status, self.stack_pointer, self.cycles);
        self.cycles += match instr {
            // TODO: Implement unofficial opcodes

            BRK => {
                self.interrupt(interconnect, ppu, Interrupt::BRK);
                7
            },

            // Stack    
            PHP => {
                let byte = self.read_reg(CPURegister::Status);
                self.push_byte_stack(interconnect, ppu, byte);
                self.set_flag(BRK_FLAG); self.bump_pc(1);
                3
            },
            PLP => {
                self.pull_byte_stack(interconnect, ppu, CPURegister::Status);
                self.bump_pc(1);
                4
            },
            PHA => {
                let byte = self.read_reg(CPURegister::A);
                self.push_byte_stack(interconnect, ppu, byte);
                self.bump_pc(1);
                3
            },
            PLA => {
                self.pull_byte_stack(interconnect, ppu, CPURegister::A);
                self.bump_pc(1);
                if self.read_reg(CPURegister::A) & (1 << 7) != 0 {
                    self.set_flag(NEGATIVE_FLAG);
                } else {
                    self.unset_flag(NEGATIVE_FLAG);
                }
                if self.read_reg(CPURegister::A) == 0 {
                    self.set_flag(ZERO_FLAG);
                } else {
                    self.unset_flag(ZERO_FLAG);
                }
                4
            },
            TXS => {
                self.transfer(CPURegister::X, CPURegister::StackPointer);
                self.bump_pc(1);
                2
            },
            TSX => {
                self.transfer(CPURegister::StackPointer, CPURegister::X);
                self.bump_pc(1);
                2
            },

            // Branch   
            BPL => {self.branch(interconnect, ppu, BranchOn::Plus)},
            BMI => {self.branch(interconnect, ppu, BranchOn::Minus)},
            BVC => {self.branch(interconnect, ppu, BranchOn::OverflowClear)},
            BVS => {self.branch(interconnect, ppu, BranchOn::OverflowSet)},
            BCC => {self.branch(interconnect, ppu, BranchOn::CarryClear)},
            BCS => {self.branch(interconnect, ppu, BranchOn::CarrySet)},
            BNE => {self.branch(interconnect, ppu, BranchOn::NotEqual)},
            BEQ => {self.branch(interconnect, ppu, BranchOn::Equal)},

            // Flag instructions
            CLC => {self.unset_flag(CARRY_FLAG); self.bump_pc(1);2},
            SEC => {self.set_flag(CARRY_FLAG); self.bump_pc(1);2},
            // CLI => {},
            SEI => {self.set_flag(IRQ_INHIBIT_FLAG); self.bump_pc(1);2},
            CLV => {self.unset_flag(OVERFLOW_FLAG); self.bump_pc(1);2},
            CLD => {self.unset_flag(DECIMAL_FLAG); self.bump_pc(1);2},
            SED => {self.set_flag(DECIMAL_FLAG); self.bump_pc(1);2},

            // Register instructions
            DEY => {
                let val = self.read_reg(CPURegister::Y);
                let eval = val.wrapping_sub(1);
                self.write_to_reg(CPURegister::Y, eval);
                if eval  & (1 << 7) != 0 {
                    self.set_flag(NEGATIVE_FLAG);
                } else {
                    self.unset_flag(NEGATIVE_FLAG);
                };
                if eval  == 0 {
                    self.set_flag(ZERO_FLAG);
                } else {
                    self.unset_flag(ZERO_FLAG);
                };
                self.bump_pc(1);
                2
            },
            DEX => {
                let val = self.read_reg(CPURegister::X);
                let eval = val.wrapping_sub(1);
                self.write_to_reg(CPURegister::X, eval);
                if eval  & (1 << 7) != 0 {
                    self.set_flag(NEGATIVE_FLAG);
                } else {
                    self.unset_flag(NEGATIVE_FLAG);
                };
                if eval  == 0 {
                    self.set_flag(ZERO_FLAG);
                } else {
                    self.unset_flag(ZERO_FLAG);
                };
                self.bump_pc(1);
                2
            },
            INX => {
                let val = self.read_reg(CPURegister::X);
                let eval = val.wrapping_add(1);
                self.write_to_reg(CPURegister::X, eval);
                if eval  & (1 << 7) != 0 {
                    self.set_flag(NEGATIVE_FLAG);
                } else {
                    self.unset_flag(NEGATIVE_FLAG);
                };
                if eval  == 0 {
                    self.set_flag(ZERO_FLAG);
                } else {
                    self.unset_flag(ZERO_FLAG);
                };
                self.bump_pc(1);
                2
            },
            INY => {
                let val = self.read_reg(CPURegister::Y);
                let eval = val.wrapping_add(1);
                self.write_to_reg(CPURegister::Y, eval);
                if eval  & (1 << 7) != 0 {
                    self.set_flag(NEGATIVE_FLAG);
                } else {
                    self.unset_flag(NEGATIVE_FLAG);
                };
                if eval  == 0 {
                    self.set_flag(ZERO_FLAG);
                } else {
                    self.unset_flag(ZERO_FLAG);
                };
                self.bump_pc(1);
                2
            },
            TAX => {
                self.transfer(CPURegister::A, CPURegister::X);
                self.bump_pc(1);
                2
            },
            TXA => {
                self.transfer(CPURegister::X, CPURegister::A);
                self.bump_pc(1);
                2
            },
            TAY => {
                self.transfer(CPURegister::A, CPURegister::Y);
                self.bump_pc(1);
                2
            },
            TYA => {
                self.transfer(CPURegister::Y, CPURegister::A);
                self.bump_pc(1);
                2
            },

            // Compares
            CPYImm => {
                let val = self.immediate(interconnect, ppu);
                self.compare(CPURegister::Y, val);
                self.bump_pc(2);
                2
            }, 
            CPYZPg=> {
                let addr = self.zero_page(interconnect, ppu);
                let val = interconnect.read_byte(ppu, addr);
                self.compare(CPURegister::Y, val);
                self.bump_pc(2);
                3
            }, 
            CPYAbs => {
                let addr = self.absolute(interconnect);
                let val = interconnect.read_byte(ppu, addr);
                self.compare(CPURegister::Y, val);
                self.bump_pc(3);
                4
            }, 
            CPXImm => {
                let val = self.immediate(interconnect, ppu);
                self.compare(CPURegister::X, val);
                self.bump_pc(2);
                2
            }, 
            CPXZPg=> {
                let addr = self.zero_page(interconnect, ppu);
                let val = interconnect.read_byte(ppu, addr);
                self.compare(CPURegister::X, val);
                self.bump_pc(2);
                3
            }, 
            CPXAbs => {
                let addr = self.absolute(interconnect);
                let val = interconnect.read_byte(ppu, addr);
                self.compare(CPURegister::X, val);
                self.bump_pc(3);
                4
            }, 

            // Loads
            LDAInxX=> {
                let addr = self.indexed_indirect(interconnect, ppu);
                let val = interconnect.read_byte(ppu, addr);
                self.load(CPURegister::A, val);
                self.bump_pc(2);
                6
            }, 
            LDAZpg => {
                let addr = self.zero_page(interconnect, ppu);
                let val = interconnect.read_byte(ppu, addr);
                self.load(CPURegister::A, val);
                self.bump_pc(2);
                3
            }, 
            LDAImm => {
                let val = self.immediate(interconnect, ppu);
                self.load(CPURegister::A, val);
                self.bump_pc(2);
                2
            }, 
            LDAAbs => {
                let addr = self.absolute(interconnect);
                let val = interconnect.read_byte(ppu, addr);
                self.load(CPURegister::A, val);
                self.bump_pc(3);
                4
            }, 
            LDAIndY => {
                let addr = self.indirect_indexed(interconnect, ppu);
                let val = interconnect.read_byte(ppu, addr);
                self.load(CPURegister::A, val);
                self.bump_pc(2);
                5
            }, 
            LDAZPgX   => {
                let addr = self.z_page_indexed(interconnect, ppu, CPURegister::X);
                let val = interconnect.read_byte(ppu, addr);
                self.load(CPURegister::A, val);
                self.bump_pc(2);
                4
            }, 
            LDAAx   => {
                let addr = self.absolute_indexed(interconnect, CPURegister::X);
                let val = interconnect.read_byte(ppu, addr);
                self.load(CPURegister::A, val);
                self.bump_pc(3);
                4
            }, 
            LDAAy   => {
                let addr = self.absolute_indexed(interconnect, CPURegister::Y);
                let val = interconnect.read_byte(ppu, addr);
                self.load(CPURegister::A, val);
                self.bump_pc(3);
                4
            }, 

            LDXImm => {
                let val = self.immediate(interconnect, ppu);
                self.load(CPURegister::X, val);
                self.bump_pc(2);
                2
            },
            LDXZpg => {
                let addr = self.zero_page(interconnect, ppu);
                let val = interconnect.read_byte(ppu, addr);
                self.load(CPURegister::X, val);
                self.bump_pc(2);
                3
            },
            LDXAbs => {
                let addr = self.absolute(interconnect);
                let val = interconnect.read_byte(ppu, addr);
                self.load(CPURegister::X, val);
                self.bump_pc(3);
                4
            }, 
            LDXZPgY  => {
                let addr = self.z_page_indexed(interconnect, ppu, CPURegister::Y);
                let val = interconnect.read_byte(ppu, addr);
                self.load(CPURegister::X, val);
                self.bump_pc(2);
                4
            }, 
            LDXAy  => {
                let addr = self.absolute_indexed(interconnect, CPURegister::Y);
                let val = interconnect.read_byte(ppu, addr);
                self.load(CPURegister::X, val);
                self.bump_pc(3);
                4
            }, 

            LDYImm => {
                let val = self.immediate(interconnect, ppu);
                self.load(CPURegister::Y, val);
                self.bump_pc(2);
                2
            },
            LDYZPg=> {
                let addr = self.zero_page(interconnect, ppu);
                let val = interconnect.read_byte(ppu, addr);
                self.load(CPURegister::Y, val);
                self.bump_pc(2);
                3
            }, 
            LDYAbs => {
                let addr = self.absolute(interconnect);
                let val = interconnect.read_byte(ppu, addr);
                self.load(CPURegister::Y, val);
                self.bump_pc(3);
                4
            }, 
            LDYZPgX  => {
                let addr = self.z_page_indexed(interconnect, ppu, CPURegister::X);
                let val = interconnect.read_byte(ppu, addr);
                self.load(CPURegister::Y, val);
                self.bump_pc(2);
                4
            }, 
            LDYAx  => {
                let addr = self.absolute_indexed(interconnect, CPURegister::X);
                let val = interconnect.read_byte(ppu, addr);
                self.load(CPURegister::Y, val);
                self.bump_pc(3);
                4
            }, 

            // Stores
            STAInxX => {
                let addr = self.indexed_indirect(interconnect, ppu);
                self.store(interconnect, ppu, addr, CPURegister::A);
                self.bump_pc(2);
                6
            }, 
            STAZpg => {
                let addr = self.zero_page(interconnect, ppu);
                self.store(interconnect, ppu, addr as u16, CPURegister::A);
                self.bump_pc(2);
                3
            },
            STAAbs => {
                let addr = self.absolute(interconnect);
                self.store(interconnect, ppu, addr, CPURegister::A);
                self.bump_pc(3);
                4
            },
            STAIndY => {
                let addr = self.indirect_indexed(interconnect, ppu);
                self.store(interconnect, ppu, addr, CPURegister::A);
                self.bump_pc(2);
                6
            },
            STAZPgX  => {
                let addr = self.z_page_indexed(interconnect, ppu, CPURegister::X);
                self.store(interconnect, ppu, addr, CPURegister::A);
                self.bump_pc(2);
                4
            }, 
            STAAx  => {
                let addr = self.absolute_indexed(interconnect, CPURegister::X);
                self.store(interconnect, ppu, addr, CPURegister::A);
                self.bump_pc(3);
                5
            },
            STAAy  => {
                let addr = self.absolute_indexed(interconnect, CPURegister::Y);
                self.store(interconnect, ppu, addr, CPURegister::A);
                self.bump_pc(3);
                5
            }, 

            STXZpg => {
                let addr = interconnect.read_byte(ppu, self.pc + 1);
                self.store(interconnect, ppu, addr as u16, CPURegister::X);
                self.bump_pc(2);
                3
            },
            STXAbs => {
                let addr = interconnect.read_word(self.pc + 1);
                self.store(interconnect, ppu, addr, CPURegister::X);
                self.bump_pc(3);
                4
            },
            STXZPgY => {
                let addr = self.z_page_indexed(interconnect, ppu, CPURegister::Y);
                self.store(interconnect, ppu, addr, CPURegister::X);
                self.bump_pc(2);
                4
            }, 
            STYZpg => {
                let addr = interconnect.read_byte(ppu, self.pc + 1);
                self.store(interconnect, ppu, addr as u16, CPURegister::Y);
                self.bump_pc(2);
                3
            },
            STYAbs => {
                let addr = interconnect.read_word(self.pc + 1);
                self.store(interconnect, ppu, addr as u16, CPURegister::Y);
                self.bump_pc(3);
                4
            }, 
            STYZPgX => {
                let addr = self.z_page_indexed(interconnect, ppu, CPURegister::X);
                self.store(interconnect, ppu, addr, CPURegister::Y);
                self.bump_pc(2);
                4
            }, 

            // Jumps
            JSRAbs => {
                self.push_return_addr(interconnect, ppu);
                let addr = interconnect.read_word(self.pc + 1);
                self.jmp(addr);
                6
            },
            JMPAbs => {
                let target_addr = self.absolute(interconnect);
                self.jmp(target_addr);
                3
            },
            JMPInd => {
                let addr = interconnect.read_word(self.pc + 1);
                if addr & 0x00ff == 0xff {
                    let target_addr = interconnect.read_byte(ppu, addr) as u16 |
                    ((interconnect.read_byte(ppu, (addr + 1 - 0x100)) as u16) << 8);
                    self.jmp(target_addr);
                } else {
                    let target_addr = interconnect.read_word(addr);
                    self.jmp(target_addr);
                }
                5
            },
            
            RTI     => {
                self.pull_byte_stack(interconnect, ppu, CPURegister::Status);
                let ret_addr = self.pull_return_addr(interconnect, ppu) - 1;
                self.jmp(ret_addr);
                6
            }, 
            RTS => {
                let ret_addr = self.pull_return_addr(interconnect, ppu);
                self.jmp(ret_addr);
                6
            },

            // Bit tests
            BITZpg => {
                let addr = self.zero_page(interconnect, ppu);
                let val = interconnect.read_byte(ppu, addr);
                self.bit(val);
                self.bump_pc(2);
                3
            },
            BITAbs => {
                let addr = self.absolute(interconnect);
                let val = interconnect.read_byte(ppu, addr);
                self.bit(val);
                self.bump_pc(3);
                4
            }, 

            // ALU operations
            ORAInxX => {
                let addr = self.indexed_indirect(interconnect, ppu);
                let val = interconnect.read_byte(ppu, addr);
                self.logic_op(val, |val, accumulator|{val | accumulator});
                self.bump_pc(2);
                6
            }, 
            ORAZPg=> {
                let addr = self.zero_page(interconnect, ppu);
                let val = interconnect.read_byte(ppu, addr);
                self.logic_op(val, |val, accumulator|{val | accumulator});
                self.bump_pc(2);
                3
            }, 
            ORAImm => {
                let val = self.immediate(interconnect, ppu);
                self.logic_op(val, |val, accumulator|{val | accumulator});
                self.bump_pc(2);
                2
            },
            ORAAbs => {
                let addr = self.absolute(interconnect);
                let val = interconnect.read_byte(ppu, addr);
                self.logic_op(val, |val, accumulator|{val | accumulator});
                self.bump_pc(3);
                4
            }, 
            ORAIndY => {
                let addr = self.indirect_indexed(interconnect, ppu);
                let val = interconnect.read_byte(ppu, addr);
                self.logic_op(val, |val, accumulator|{val | accumulator});
                self.bump_pc(2);
                5
            }, 
            ORAZPgX  => {
                let addr = self.z_page_indexed(interconnect, ppu, CPURegister::X);
                let val = interconnect.read_byte(ppu, addr);
                self.logic_op(val, |val, accumulator|{val | accumulator});
                self.bump_pc(2);
                4
            }, 
            ORAAx  => {
                let addr = self.absolute_indexed(interconnect, CPURegister::X);
                let val = interconnect.read_byte(ppu, addr);
                self.logic_op(val, |val, accumulator|{val | accumulator});
                self.bump_pc(3);
                4
            }, 
            ORAAy  => {
                let addr = self.absolute_indexed(interconnect, CPURegister::Y);
                let val = interconnect.read_byte(ppu, addr);
                self.logic_op(val, |val, accumulator|{val | accumulator});
                self.bump_pc(3);
                4
            }, 

            ANDInxX => {
                let addr = self.indexed_indirect(interconnect, ppu);
                let val = interconnect.read_byte(ppu, addr);
                self.logic_op(val, |val, accumulator|{val & accumulator});
                self.bump_pc(2);
                6
            }, 
            ANDZPg=> {
                let addr = self.zero_page(interconnect, ppu);
                let val = interconnect.read_byte(ppu, addr);
                self.logic_op(val, |val, accumulator|{val & accumulator});
                self.bump_pc(2);
                3
            }, 
            ANDImm => {
                let val = self.immediate(interconnect, ppu);
                self.logic_op(val, |val, accumulator|{val & accumulator});
                self.bump_pc(2);
                2
            },
            ANDAbs => {
                let addr = self.absolute(interconnect);
                let val = interconnect.read_byte(ppu, addr);
                self.logic_op(val, |val, accumulator|{val & accumulator});
                self.bump_pc(3);
                4
            }, 
            ANDIndY => {
                let addr = self.indirect_indexed(interconnect, ppu);
                let val = interconnect.read_byte(ppu, addr);
                self.logic_op(val, |val, accumulator|{val & accumulator});
                self.bump_pc(2);
                5
            }, 
            ANDZPgX  => {
                let addr = self.z_page_indexed(interconnect, ppu, CPURegister::X);
                let val = interconnect.read_byte(ppu, addr);
                self.logic_op(val, |val, accumulator|{val & accumulator});
                self.bump_pc(2);
                4
            }, 
            ANDAx  => {
                let addr = self.absolute_indexed(interconnect, CPURegister::X);
                let val = interconnect.read_byte(ppu, addr);
                self.logic_op(val, |val, accumulator|{val & accumulator});
                self.bump_pc(3);
                4
            }, 
            ANDAy  => {
                let addr = self.absolute_indexed(interconnect, CPURegister::Y);
                let val = interconnect.read_byte(ppu, addr);
                self.logic_op(val, |val, accumulator|{val & accumulator});
                self.bump_pc(3);
                4
            }, 

            EORInxX => {
                let addr = self.indexed_indirect(interconnect, ppu);
                let val = interconnect.read_byte(ppu, addr);
                self.logic_op(val, |val, accumulator|{val ^ accumulator});
                self.bump_pc(2);
                6
            }, 
            EORZPg=> {
                let addr = self.zero_page(interconnect, ppu);
                let val = interconnect.read_byte(ppu, addr);
                self.logic_op(val, |val, accumulator|{val ^ accumulator});
                self.bump_pc(2);
                3
            }, 
            EORImm => {
                let val = self.immediate(interconnect, ppu);
                self.logic_op(val, |val, accumulator|{val ^ accumulator});
                self.bump_pc(2);
                2
            }, 
            EORAbs => {
                let addr = self.absolute(interconnect);
                let val = interconnect.read_byte(ppu, addr);
                self.logic_op(val, |val, accumulator|{val ^ accumulator});
                self.bump_pc(3);
                4
            }, 
            EORIndY => {
                let addr = self.indirect_indexed(interconnect, ppu);
                let val = interconnect.read_byte(ppu, addr);
                self.logic_op(val, |val, accumulator|{val ^ accumulator});
                self.bump_pc(2);
                5
            }, 
            EORZPgX  => {
                let addr = self.z_page_indexed(interconnect, ppu, CPURegister::X);
                let val = interconnect.read_byte(ppu, addr);
                self.logic_op(val, |val, accumulator|{val ^ accumulator});
                self.bump_pc(2);
                4
            }, 
            EORAx  => {
                let addr = self.absolute_indexed(interconnect, CPURegister::X);
                let val = interconnect.read_byte(ppu, addr);
                self.logic_op(val, |val, accumulator|{val ^ accumulator});
                self.bump_pc(3);
                4
            }, 
            EORAy  => {
                let addr = self.absolute_indexed(interconnect, CPURegister::Y);
                let val = interconnect.read_byte(ppu, addr);
                self.logic_op(val, |val, accumulator|{val ^ accumulator});
                self.bump_pc(3);
                4
            }, 

            ADCInxX => {
                let addr = self.indexed_indirect(interconnect, ppu);
                let val = interconnect.read_byte(ppu, addr);
                self.add(val);
                self.bump_pc(2);
                6
            }, 
            ADCZPg=> {
                let addr = self.zero_page(interconnect, ppu);
                let val = interconnect.read_byte(ppu, addr);
                self.add(val);
                self.bump_pc(2);
                3
            }, 
            ADCImm => {
                let val = self.immediate(interconnect, ppu);
                self.add(val);
                self.bump_pc(2);
                2
            },
            ADCAbs => {
                let addr = self.absolute(interconnect);
                let val = interconnect.read_byte(ppu, addr);
                self.add(val);
                self.bump_pc(3);
                4
            }, 
            ADCIndY => {
                let addr = self.indirect_indexed(interconnect, ppu);
                let val = interconnect.read_byte(ppu, addr);
                self.add(val);
                self.bump_pc(2);
                5
            }, 
            ADCZPgX  => {
                let addr = self.z_page_indexed(interconnect, ppu, CPURegister::X);
                let val = interconnect.read_byte(ppu, addr);
                self.add(val);
                self.bump_pc(2);
                4
            }, 
            ADCAx  => {
                let addr = self.absolute_indexed(interconnect, CPURegister::X);
                let val = interconnect.read_byte(ppu, addr);
                self.add(val);
                self.bump_pc(3);
                4
            }, 
            ADCAy  => {
                let addr = self.absolute_indexed(interconnect, CPURegister::Y);
                let val = interconnect.read_byte(ppu, addr);
                self.add(val);
                self.bump_pc(3);
                4
            }, 

            CMPInxX => {
                let addr = self.indexed_indirect(interconnect, ppu);
                let val = interconnect.read_byte(ppu, addr);
                self.compare(CPURegister::A, val);
                self.bump_pc(2);
                6
            },
            CMPZPg=> {
                let addr = self.zero_page(interconnect, ppu);
                let val = interconnect.read_byte(ppu, addr);
                self.compare(CPURegister::A, val);
                self.bump_pc(2);
                3
            }, 
            CMPImm => {
                let val = self.immediate(interconnect, ppu);
                self.compare(CPURegister::A, val);
                self.bump_pc(2);
                2
            }, 
            CMPAbs => {
                let addr = self.absolute(interconnect);
                let val = interconnect.read_byte(ppu, addr);
                self.compare(CPURegister::A, val);
                self.bump_pc(3);
                4
            }, 
            CMPIndY => {
                let addr = self.indirect_indexed(interconnect, ppu);
                let val = interconnect.read_byte(ppu, addr);
                self.compare(CPURegister::A, val);
                self.bump_pc(2);
                5
            },
            CMPZPgX  => {
                let addr = self.z_page_indexed(interconnect, ppu, CPURegister::X);
                let val = interconnect.read_byte(ppu, addr);
                self.compare(CPURegister::A, val);
                self.bump_pc(2);
                4
            }, 
            CMPAx  => {
                let addr = self.absolute_indexed(interconnect, CPURegister::X);
                let val = interconnect.read_byte(ppu, addr);
                self.compare(CPURegister::A, val);
                self.bump_pc(3);
                4
            }, 
            CMPAy  => {
                let addr = self.absolute_indexed(interconnect, CPURegister::Y);
                let val = interconnect.read_byte(ppu, addr);
                self.compare(CPURegister::A, val);
                self.bump_pc(3);
                4
            }, 

            SBCInxX=> {
                let addr = self.indexed_indirect(interconnect, ppu);
                let val = interconnect.read_byte(ppu, addr);
                self.sub(val);
                self.bump_pc(2);
                6
            }, 
            SBCZPg=> {
                let addr = self.zero_page(interconnect, ppu);
                let val = interconnect.read_byte(ppu, addr);
                self.sub(val);
                self.bump_pc(2);
                3
            }, 
            SBCImm => {
                let val = self.immediate(interconnect, ppu);
                self.sub(val);
                self.bump_pc(2);
                2
            }, 
            SBCAbs => {
                let addr = self.absolute(interconnect);
                let val = interconnect.read_byte(ppu, addr);
                self.sub(val);
                self.bump_pc(3);
                4
            }, 
            SBCIndY => {
                let addr = self.indirect_indexed(interconnect, ppu);
                let val = interconnect.read_byte(ppu, addr);
                self.sub(val);
                self.bump_pc(2);
                5
            }, 
            SBCZPgX  => {
                let addr = self.z_page_indexed(interconnect, ppu, CPURegister::X);
                let val = interconnect.read_byte(ppu, addr);
                self.sub(val);
                self.bump_pc(2);
                4
            }, 
            SBCAx  => {
                let addr = self.absolute_indexed(interconnect, CPURegister::X);
                let val = interconnect.read_byte(ppu, addr);
                self.sub(val);
                self.bump_pc(3);
                4
            }, 
            SBCAy  => {
                let addr = self.absolute_indexed(interconnect, CPURegister::Y);
                let val = interconnect.read_byte(ppu, addr);
                self.sub(val);
                self.bump_pc(3);
                4
            }, 
                 
            ASLZPg => {
                self.asl(interconnect, ppu, AddressingMode::ZeroPage);
                self.bump_pc(2);
                5
            }
            ASL    => {
                let mut val = self.read_reg(CPURegister::A);
                if val & (1 << 7) != 0 {
                    self.set_flag(CARRY_FLAG);
                } else {
                    self.unset_flag(CARRY_FLAG);
                }
                val = val << 1;
                if val  & (1 << 7) != 0 {
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
                2
            }, 
            ASLAbs => {
                self.asl(interconnect, ppu, AddressingMode::Absolute);
                self.bump_pc(3);
                6
            }, 
            ASLZPgX  => {
                self.asl(interconnect, ppu, AddressingMode::ZeroPageIndexed);
                self.bump_pc(2);
                6
            }, 
            ASLAx  => {
                self.asl(interconnect, ppu, AddressingMode::AbsoluteIndexed);
                self.bump_pc(3);
                7
            }, 
            
            LSRZPg => {
                self.lsr(interconnect, ppu, AddressingMode::ZeroPage);
                self.bump_pc(2);
                5
            }, 
            LSR    => {
                let mut val = self.read_reg(CPURegister::A);
                if val & (1 << 0) != 0 {
                    self.set_flag(CARRY_FLAG);
                } else {
                    self.unset_flag(CARRY_FLAG);
                }
                val = val >> 1;
                if val  & (1 << 7) != 0 {
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
                2
            }, 
            LSRAbs => {
                self.lsr(interconnect, ppu, AddressingMode::Absolute);
                self.bump_pc(3);
                6
            }, 
            LSRZPgX  => {
                self.lsr(interconnect, ppu, AddressingMode::ZeroPageIndexed);
                self.bump_pc(2);
                6
            }, 
            LSRAx  => {
                self.lsr(interconnect, ppu, AddressingMode::AbsoluteIndexed);
                self.bump_pc(3);
                7
            }, 
            
            // Rotates
            ROLZPg=> {
                self.rol(interconnect, ppu, AddressingMode::ZeroPage);
                self.bump_pc(2);
                5
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
                if val  & (1 << 7) != 0 {
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
                2
            }, 
            ROLAbs => {
                self.rol(interconnect, ppu, AddressingMode::Absolute);
                self.bump_pc(3);
                6
            }, 
            ROLZPgX  => {
                self.rol(interconnect, ppu, AddressingMode::ZeroPageIndexed);
                self.bump_pc(2);
                6
            }, 
            ROLAx  => {
                self.rol(interconnect, ppu, AddressingMode::AbsoluteIndexed);
                self.bump_pc(3);
                7
            }, 

            RORZPg=> {
                self.ror(interconnect, ppu, AddressingMode::ZeroPage);
                self.bump_pc(2);
                5
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
                if val  & (1 << 7) != 0 {
                    self.set_flag(NEGATIVE_FLAG);
                } else {
                    self.unset_flag(NEGATIVE_FLAG);
                };
                if val  == 0 {self.set_flag(ZERO_FLAG);} else {self.unset_flag(ZERO_FLAG);};
                self.write_to_reg(CPURegister::A, val);
                self.bump_pc(1);
                2
            }, 
            RORAbs => {
                self.ror(interconnect, ppu, AddressingMode::Absolute);
                self.bump_pc(3);
                6
            }, 
            RORZPgX  => {
                self.ror(interconnect, ppu, AddressingMode::ZeroPageIndexed);
                self.bump_pc(2);
                6
            }, 
            RORAx  => {
                self.ror(interconnect, ppu, AddressingMode::AbsoluteIndexed);
                self.bump_pc(3);
                7
            }, 

            // Increments
            DECZpg => {
                self.decrement(interconnect, ppu, AddressingMode::ZeroPage);
                self.bump_pc(2);
                5
            },
            DECAbs => {
                self.decrement(interconnect, ppu, AddressingMode::Absolute);
                self.bump_pc(3);
                6
            }, 
            DECZPgX  => {
                self.decrement(interconnect, ppu, AddressingMode::ZeroPageIndexed);
                self.bump_pc(2);
                6
            }, 
            DECAx  => {
                self.decrement(interconnect, ppu, AddressingMode::AbsoluteIndexed);
                self.bump_pc(3);
                7
            }, 

            INCZPg=> {
                self.increment(interconnect, ppu, AddressingMode::ZeroPage);
                self.bump_pc(2);
                5
            }, 
            INCAbs => {
                self.increment(interconnect, ppu, AddressingMode::Absolute);
                self.bump_pc(3);
                6
            }, 
            INCZPgX  => {
                self.increment(interconnect, ppu, AddressingMode::ZeroPageIndexed);
                self.bump_pc(2);
                6
            }, 
            INCAx  => {
                self.increment(interconnect, ppu, AddressingMode::AbsoluteIndexed);
                self.bump_pc(3);
                7
            }, 
            
            // The ever important nop
            // Observe all its majesty
            NOP => {self.bump_pc(1);2},
        }
    }

    // Addressing modes
    fn immediate(&self, interconnect: &Interconnect, ppu: &Ppu) -> u8 {
        interconnect.read_byte(ppu, self.pc + 1)
    }

    fn absolute(&self, interconnect: &Interconnect) -> u16 {
        interconnect.read_word(self.pc + 1)
    }

    fn zero_page(&self, interconnect: &Interconnect, ppu: &Ppu) -> u16 {
        interconnect.read_byte(ppu, self.pc + 1) as u16
    }

    // TODO: Check for CPURegister::{X or Y}
    fn absolute_indexed(&self, interconnect: &Interconnect, register: CPURegister) -> u16 {
        let addr = interconnect.read_word(self.pc + 1);
        addr.wrapping_add((self.read_reg(register) as u16))
    }

    // TODO: Check for CPURegister::{X or Y}
    fn z_page_indexed(&self, interconnect: &Interconnect, ppu: &Ppu, register: CPURegister) -> u16 {
        let addr = interconnect.read_byte(ppu, self.pc + 1);
        addr.wrapping_add(self.read_reg(register)) as u16
    }

    // PRETTIFYME
    fn indexed_indirect(&self, interconnect: &Interconnect, ppu: &Ppu) -> u16 {
        let addr = interconnect.read_byte(ppu, self.pc + 1);
        let sum_addr = addr.wrapping_add((self.read_reg(CPURegister::X))) as u16;
        if sum_addr == 0xff {
            (interconnect.read_byte(ppu, sum_addr) as u16) |
            ((interconnect.read_byte(ppu, 0x0000) as u16) << 8)
        } else {
            interconnect.read_word(sum_addr as u16)
        }
    }

    fn indirect_indexed(&self, interconnect: &Interconnect, ppu: &Ppu) -> u16 {
        let temp_addr = interconnect.read_byte(ppu, self.pc + 1) as u16;
        if temp_addr == 0xff {
            let addr = interconnect.read_byte(ppu, temp_addr) as u16 |
            ((interconnect.read_byte(ppu, 0x0000) as u16) << 8);
            addr.wrapping_add(self.read_reg(CPURegister::Y) as u16)
        } else {
            let addr = interconnect.read_word(temp_addr);
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

    fn asl(&mut self, interconnect: &mut Interconnect, ppu: &mut Ppu, am: AddressingMode) {
        use self::AddressingMode::*;
        let addr = match am {
            ZeroPage => {self.zero_page(interconnect, ppu)},
            ZeroPageIndexed => {self.z_page_indexed(interconnect, ppu, CPURegister::X)},
            Absolute => {self.absolute(interconnect)},
            AbsoluteIndexed => {self.absolute_indexed(interconnect, CPURegister::X)},
            _ => panic!("ASL with unsupported Addressing Mode: {:?}", am),
        };
        let mut val = interconnect.read_byte(ppu, addr);
        if val & (1 << 7) != 0 {
            self.set_flag(CARRY_FLAG);
        } else {
            self.unset_flag(CARRY_FLAG);
        }
        val = val << 1;
        if val  & (1 << 7) != 0 {
            self.set_flag(NEGATIVE_FLAG);
        } else {
            self.unset_flag(NEGATIVE_FLAG);
        };
        if val  == 0 {
            self.set_flag(ZERO_FLAG);
        } else {
            self.unset_flag(ZERO_FLAG);
        };
        interconnect.write_byte(ppu, addr, val);
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

    fn branch(&mut self, interconnect: &Interconnect, ppu: &Ppu, branch_on: BranchOn) -> u64{
        let branch_target = interconnect.read_byte(ppu, self.pc + 1) as i8;
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
            self.pc = ((pc as i32) + (branch_target as i32)) as u16; 3} else {self.bump_pc(2); 2}
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

    fn decrement(&mut self, interconnect: &mut Interconnect, ppu: &mut Ppu, am: AddressingMode) {
        let addr = match am {
            AddressingMode::ZeroPage => {self.zero_page(interconnect, ppu)},
            AddressingMode::ZeroPageIndexed => {self.z_page_indexed(interconnect, ppu, CPURegister::X)},
            AddressingMode::Absolute => {self.absolute(interconnect)},
            AddressingMode::AbsoluteIndexed => {self.absolute_indexed(interconnect, CPURegister::X)},
            _ => panic!("Decrement with unsupported Addressing Mode: {:?}", am),
        };
        let temp_val = interconnect.read_byte(ppu, addr);
        let val = temp_val.wrapping_sub(1);
        if val > temp_val {
            self.set_flag(OVERFLOW_FLAG);
        } else {
            self.unset_flag(OVERFLOW_FLAG);
        }
        if val  & (1 << 7) != 0 {
            self.set_flag(NEGATIVE_FLAG);
        } else {
            self.unset_flag(NEGATIVE_FLAG);
        };
        if val  == 0 {
            self.set_flag(ZERO_FLAG);
        } else {
            self.unset_flag(ZERO_FLAG);
        };
        interconnect.write_byte(ppu, addr, val);
    }

    fn increment(&mut self, interconnect: &mut Interconnect, ppu: &mut Ppu, am: AddressingMode) {
        let addr = match am {
            AddressingMode::ZeroPage => {self.zero_page(interconnect, ppu)},
            AddressingMode::ZeroPageIndexed => {self.z_page_indexed(interconnect, ppu, CPURegister::X)},
            AddressingMode::Absolute => {self.absolute(interconnect)},
            AddressingMode::AbsoluteIndexed => {self.absolute_indexed(interconnect, CPURegister::X)},
            _ => panic!("Increment with unsupported Addressing Mode: {:?}", am),
        };
        let temp_val = interconnect.read_byte(ppu, addr);
        let val = temp_val.wrapping_add(1);
        if val < temp_val {
            self.set_flag(OVERFLOW_FLAG);
        } else {
            self.unset_flag(OVERFLOW_FLAG);
        }
        if val  & (1 << 7) != 0 {
            self.set_flag(NEGATIVE_FLAG);
        } else {
            self.unset_flag(NEGATIVE_FLAG);
        };
        if val  == 0 {
            self.set_flag(ZERO_FLAG);
        } else {
            self.unset_flag(ZERO_FLAG);
        };
        interconnect.write_byte(ppu, addr, val);
    }

    fn jmp(&mut self, jump_target: u16) {
        self.pc = jump_target;
    }

    fn load(&mut self, register: CPURegister, arg: u8) {
        self.write_to_reg(register, arg);
        if self.read_reg(register) & (1 << 7) != 0 {
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

    fn logic_op<F>(&mut self, arg: u8, f: F) where F: FnOnce(u8, u8) -> u8 {
        let a = self.read_reg(CPURegister::A);
        let eval = f(a, arg);
        self.write_to_reg(CPURegister::A, eval);
        if self.read_reg(CPURegister::A) & (1 << 7) != 0 {
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

    fn lsr(&mut self, interconnect: &mut Interconnect, ppu: &mut Ppu, am: AddressingMode) {
        use self::AddressingMode::*;
        let addr = match am {
            ZeroPage => {self.zero_page(interconnect, ppu)},
            ZeroPageIndexed => {self.z_page_indexed(interconnect, ppu, CPURegister::X)},
            Absolute => {self.absolute(interconnect)},
            AbsoluteIndexed => {self.absolute_indexed(interconnect, CPURegister::X)},
            _ => panic!("LSR with unsupported Addressing Mode: {:?}", am),
        };
        let mut val = interconnect.read_byte(ppu, addr);
        if val & (1 << 0) != 0 {
            self.set_flag(CARRY_FLAG);
        } else {
            self.unset_flag(CARRY_FLAG);
        }
        val = val >> 1;
        if val  & (1 << 7) != 0 {
            self.set_flag(NEGATIVE_FLAG);
        } else {
            self.unset_flag(NEGATIVE_FLAG);
        };
        if val  == 0 {
            self.set_flag(ZERO_FLAG);
        } else {
            self.unset_flag(ZERO_FLAG);
        };
        interconnect.write_byte(ppu, addr, val);
    }

    fn transfer(&mut self, from_reg: CPURegister, to_reg: CPURegister) {
        let val = self.read_reg(from_reg);
        self.write_to_reg(to_reg, val);
        if to_reg != CPURegister::StackPointer {
            if self.read_reg(to_reg) & (1 << 7) != 0 {
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

    fn rol(&mut self, interconnect: &mut Interconnect, ppu: &mut Ppu, am: AddressingMode) {
        use self::AddressingMode::*;
        let addr = match am {
            ZeroPage => {self.zero_page(interconnect, ppu)},
            ZeroPageIndexed => {self.z_page_indexed(interconnect, ppu, CPURegister::X)},
            Absolute => {self.absolute(interconnect)},
            AbsoluteIndexed => {self.absolute_indexed(interconnect, CPURegister::X)},
            _ => panic!("ROL with unsupported Addressing Mode: {:?}", am),
        };
        let mut val = interconnect.read_byte(ppu, addr);
        let carry = if self.check_flag(CARRY_FLAG) {1} else {0};
        if val & (1 << 7) != 0 {
            self.set_flag(CARRY_FLAG);
        } else {
            self.unset_flag(CARRY_FLAG);
        }
        val = (val << 1) | carry;
        if val  & (1 << 7) != 0 {
            self.set_flag(NEGATIVE_FLAG);
        } else {
            self.unset_flag(NEGATIVE_FLAG);
        };
        if val  == 0 {
            self.set_flag(ZERO_FLAG);
        } else {
            self.unset_flag(ZERO_FLAG);
        };
        interconnect.write_byte(ppu, addr, val);
    }

    fn ror(&mut self, interconnect: &mut Interconnect, ppu: &mut Ppu, am: AddressingMode) {
        use self::AddressingMode::*;
        let addr = match am {
            ZeroPage => {self.zero_page(interconnect, ppu)},
            ZeroPageIndexed => {self.z_page_indexed(interconnect, ppu, CPURegister::X)},
            Absolute => {self.absolute(interconnect)},
            AbsoluteIndexed => {self.absolute_indexed(interconnect, CPURegister::X)},
            _ => panic!("ROR with unsupported Addressing Mode: {:?}", am),
        };
        let mut val = interconnect.read_byte(ppu, addr);
        let carry = if self.check_flag(CARRY_FLAG) {1 << 7} else {0};
        if val & (1 << 0) != 0 {
            self.set_flag(CARRY_FLAG);
        } else {
            self.unset_flag(CARRY_FLAG);
        }
        val = (val >> 1) | carry;
        if val  & (1 << 7) != 0 {
            self.set_flag(NEGATIVE_FLAG);
        } else {
            self.unset_flag(NEGATIVE_FLAG);
        };
        if val  == 0 {
            self.set_flag(ZERO_FLAG);
        } else {
            self.unset_flag(ZERO_FLAG);
        };
        interconnect.write_byte(ppu, addr, val);
    }

    fn store(&mut self, interconnect: &mut Interconnect, ppu: &mut Ppu, addr: u16, register: CPURegister) {
        let val = self.read_reg(register);
        interconnect.write_byte(ppu, addr, val);
    }

    fn sub(&mut self, arg: u8) {
        self.add(!arg);
    }

    pub fn interrupt(&mut self, interconnect: &mut Interconnect, ppu: &mut Ppu, interrupt: Interrupt) {
        self.bump_pc(2);
        self.set_flag(IRQ_INHIBIT_FLAG);
        self.unset_flag(BRK_FLAG);
        let status = self.read_reg(CPURegister::Status);
        self.set_flag(IRQ_INHIBIT_FLAG);
        self.push_return_addr(interconnect, ppu);
        self.push_byte_stack(interconnect, ppu, status);

        let branch_target = match interrupt {
            Interrupt::BRK => {
                self.set_flag(BRK_FLAG);
                let addr = interconnect.read_word(IRQBRK_VECTOR);
                addr
            },
            Interrupt::IRQ => {
                self.cycles += 7;
                interconnect.read_word(IRQBRK_VECTOR)
            },
            Interrupt::NMI => {
                self.cycles += 7;
                interconnect.read_word(NMI_VECTOR)
            },
            Interrupt::RESET => {
                self.cycles += 7;
                interconnect.read_word(RESET_VECTOR)
            },
        };
        self.jmp(branch_target);
    }
}

// TODO: Move this to a propper debugger
impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#x} a:0x{:x} x:0x{:x} y:0x{:x} stack_pointer:0x{:x} status:0b{:#b}",
               self.pc, self.a, self.x, self.y,  self.stack_pointer, self.status)
    }
}

