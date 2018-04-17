use super::cart::Cartridge;
use super::nes::KILOBYTE;
use std::fmt;

pub struct Cpu {
    registers: Registers,
    cycles: u64,
    ram: [u8; 2 * KILOBYTE],
}

// Register names match what's listed on NESDevWiki
#[derive(Default)]
struct Registers {
    a: u8,
    x: u8,
    y: u8,
    s: u8, // Stack pointer
    p: ProcessorFlags,
    pc: u16,
}

impl fmt::Debug for Registers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X}",
               self.a, self.x, self.y, self.p, self.s)
    }
}

bitflags! {
    #[derive(Default)]
    struct ProcessorFlags: u8 {
        const CARRY      = 0b00000001;
        const ZERO       = 0b00000010;
        const INTERRUPT  = 0b00000100;
        const DECIMAL    = 0b00001000;
        const STACK_COPY = 0b00010000;
        const ALWAYS_SET = 0b00100000;
        const OVERFLOW   = 0b01000000;
        const NEGATIVE   = 0b10000000;
    }
}

trait AddressingMode {
    fn load(&self, cpu: &mut Cpu, cart: &mut Cartridge) -> u8;
    fn store(&self, cpu: &mut Cpu, cart: &mut Cartridge, val: u8);
}
struct AccumulatorAM;
struct ImmediateAM;
struct ZeroPageAM {
    arg: u8,
}
struct AbsoluteAM {
    arg: u16,
}
struct RelativeAM {
    arg: u8,
}
struct ZeroPageIdxXAM {
    arg: u8,
}
struct ZeroPageIdxYAM {
    arg: u8,
}
struct AbsoluteIdxXAM {
    arg: u16,
}
struct AbsoluteIdxYAM {
    arg: u16,
}
struct IndexedIndirectAM {
    arg: u16,
}
struct IndirectIndexedAM {
    arg: u16,
}

impl AddressingMode for AccumulatorAM {
    fn load(&self, cpu: &mut Cpu, _cart: &mut Cartridge) -> u8 {
        cpu.registers.a
    }
    fn store(&self, cpu: &mut Cpu, _cart: &mut Cartridge, val: u8) {
        cpu.registers.a = val;
    }
}

impl AddressingMode for ImmediateAM {
    fn load(&self, cpu: &mut Cpu, cart: &mut Cartridge) -> u8 {
        let ret = cpu.load_next_byte_bump_pc(cart);
        print!("#${:02X}{:24}{:?} CYC:{:}", ret, " ", cpu.registers, cpu.cycles);
        ret
    }
    fn store(&self, _cpu: &mut Cpu, _cart: &mut Cartridge, _val: u8) {
        panic!("Hah...no");
    }
}

impl AddressingMode for ZeroPageAM {
    fn load(&self, cpu: &mut Cpu, cart: &mut Cartridge) -> u8 {
        let val = cpu.fetch_byte(cart, self.arg as u16);
        print!("${:02X} = {:02X}{:20}{:?} CYC:{:}", self.arg as u8, val, " ", cpu.registers, cpu.cycles);
        val
    }
    fn store(&self, cpu: &mut Cpu, cart: &mut Cartridge, val: u8) {
        let addr = self.arg as u16;
        print!("${:02X} = {:02X}{:20}{:?} CYC:{:}", addr as u8, cpu.fetch_byte(cart, addr), " ", cpu.registers, cpu.cycles);
        cpu.store(cart, addr, val);
    }
}

impl AddressingMode for AbsoluteAM {
    fn load(&self, cpu: &mut Cpu, cart: &mut Cartridge) -> u8 {
        let addr = self.arg;
        print!("${:04X} = {:02X}{:18}{:?} CYC:{:}", addr, cpu.fetch_byte(cart, addr), " ", cpu.registers, cpu.cycles);
        cpu.fetch_byte(cart, self.arg)
    }
    fn store(&self, cpu: &mut Cpu, cart: &mut Cartridge, val: u8) {
        let addr = self.arg;
        print!("${:04X} = {:02X}{:18}{:?} CYC:{:}", addr, cpu.fetch_byte(cart, addr), " ", cpu.registers, cpu.cycles);
        cpu.store(cart, addr, val);
    }
}

impl AddressingMode for ZeroPageIdxXAM {
    fn load(&self, cpu: &mut Cpu, cart: &mut Cartridge) -> u8 {
        let addr = self.arg.wrapping_add(cpu.registers.x);
        cpu.fetch_byte(cart, addr as u16)
    }
    fn store(&self, cpu: &mut Cpu, cart: &mut Cartridge, val: u8) {}
}

impl AddressingMode for ZeroPageIdxYAM {
    fn load(&self, cpu: &mut Cpu, cart: &mut Cartridge) -> u8 {
        let addr = self.arg.wrapping_add(cpu.registers.y);
        cpu.fetch_byte(cart, addr as u16)
    }
    fn store(&self, cpu: &mut Cpu, cart: &mut Cartridge, val: u8) {}
}

impl AddressingMode for AbsoluteIdxXAM {
    fn load(&self, cpu: &mut Cpu, cart: &mut Cartridge) -> u8 {
        let addr = self.arg.wrapping_add(cpu.registers.x as u16);
        cpu.fetch_byte(cart, addr)
    }
    fn store(&self, cpu: &mut Cpu, cart: &mut Cartridge, val: u8) {}
}

impl AddressingMode for AbsoluteIdxYAM {
    fn load(&self, cpu: &mut Cpu, cart: &mut Cartridge) -> u8 {
        let addr = self.arg.wrapping_add(cpu.registers.y as u16);
        cpu.fetch_byte(cart, addr)
    }
    fn store(&self, cpu: &mut Cpu, cart: &mut Cartridge, val: u8) {}
}

impl AddressingMode for IndexedIndirectAM {
    fn load(&self, cpu: &mut Cpu, cart: &mut Cartridge) -> u8 {
        let index = cpu.load_next_byte_bump_pc(cart) + cpu.registers.x;
        let addr = cpu.fetch_word(cart, index as u16);
        cpu.fetch_byte(cart, addr)
    }
    fn store(&self, cpu: &mut Cpu, cart: &mut Cartridge, val: u8) {}
}

impl AddressingMode for IndirectIndexedAM {
    fn load(&self, cpu: &mut Cpu, cart: &mut Cartridge) -> u8 {
        let index = cpu.load_next_byte_bump_pc(cart);
        let addr = cpu.fetch_word(cart, index as u16) + cpu.registers.y as u16;
        cpu.fetch_byte(cart, addr)
    }
    fn store(&self, cpu: &mut Cpu, cart: &mut Cartridge, val: u8) {}
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            registers: Registers::default(),
            cycles: 0,
            ram: [0u8; 2 * KILOBYTE],
        }
    }

    pub fn reset(&mut self) {
        self.registers.pc = 0xC000; // FIXME: This is only for running NESTEST
        self.registers.s = 0xFD;
        self.registers.p = ProcessorFlags::from_bits(0x24).unwrap(); // TODO: ...unwrap?
    }

    pub fn step(&mut self, cart: &mut Cartridge) {
        let pc = self.registers.pc;
        let opcode = self.load_next_byte_bump_pc(cart);
        print!("{:X}  {:} ", pc, self.debug_print(cart, opcode));
        match opcode {
            // Branches
            0x10 => { let am = RelativeAM{ arg: self.load_next_byte_bump_pc(cart) }; self.bpl(am); }
            0x30 => { let am = RelativeAM{ arg: self.load_next_byte_bump_pc(cart) }; self.bmi(am); }
            0x50 => { let am = RelativeAM{ arg: self.load_next_byte_bump_pc(cart) }; self.bvc(am); }
            0x70 => { let am = RelativeAM{ arg: self.load_next_byte_bump_pc(cart) }; self.bvs(am); }
            0x90 => { let am = RelativeAM{ arg: self.load_next_byte_bump_pc(cart) }; self.bcc(am); }
            0xB0 => { let am = RelativeAM{ arg: self.load_next_byte_bump_pc(cart) }; self.bcs(am); }
            0xD0 => { let am = RelativeAM{ arg: self.load_next_byte_bump_pc(cart) }; self.bne(am); }
            0xF0 => { let am = RelativeAM{ arg: self.load_next_byte_bump_pc(cart) }; self.beq(am); }

            // Flag sets
            0x38 => { self.sec(); }
            0x78 => { self.sei(); }
            0xF8 => { self.sed(); }

            // Flag clears
            0x18 => { self.clc(); }
            0xB8 => { self.clv(); }
            0xD8 => { self.cld(); }

            // Stack
            0x08 => { self.php(); }
            0x28 => { self.plp(); }
            0x48 => { self.pha(); }
            0x68 => { self.pla(); }

            // ALU operations
            0x69 => { let am = ImmediateAM; self.adc(cart, am); }

            0x29 => { let am = ImmediateAM; self.and(cart, am); }

            0x24 => { let am = ZeroPageAM{ arg: self.load_next_byte_bump_pc(cart) }; self.bit(cart, am); }

            0xC9 => { let am = ImmediateAM; self.cmp(cart, am); }

            0xE0 => { let am = ImmediateAM; self.cpx(cart, am); }

            0xC0 => { let am = ImmediateAM; self.cpy(cart, am); }

            0x49 => { let am = ImmediateAM; self.eor(cart, am); }

            0x09 => { let am = ImmediateAM; self.ora(cart, am); }

            0xE9 => { let am = ImmediateAM; self.sbc(cart, am); }

            // Increments and Decrements
            0xE8 => { self.inx(); }
            0xC8 => { self.iny(); }
            0xCA => { self.dex(); }
            0x88 => { self.dey(); }

            // Transfers
            0xAA => { self.tax(); }
            0xA8 => { self.tay(); }
            0xBA => { self.tsx(); }
            0x8A => { self.txa(); }
            0x9A => { self.txs(); }
            0x98 => { self.tya(); }

            // Loads
            0xA9 => { let am = ImmediateAM; self.lda(cart, am); }
            0xAD => { let am = AbsoluteAM{ arg: self.load_next_word_bump_pc(cart) }; self.lda(cart, am); }

            0xA2 => { let am = ImmediateAM; self.ldx(cart, am); }
            0xAE => { let am = AbsoluteAM{ arg: self.load_next_word_bump_pc(cart) }; self.ldx(cart, am); }

            0xA0 => { let am = ImmediateAM; self.ldy(cart, am); }

            // Stores
            0x85 => { let am = ZeroPageAM{ arg: self.load_next_byte_bump_pc(cart) }; self.sta(cart, am); }

            0x86 => { let am = ZeroPageAM{ arg: self.load_next_byte_bump_pc(cart) }; self.stx(cart, am); }
            0x8E => { let am = AbsoluteAM{ arg: self.load_next_word_bump_pc(cart) }; self.stx(cart, am); }

            // Jumps
            0x4C => self.jmp_indirect(cart),
            0x20 => self.jsr(cart),
            0x60 => self.rts(),

            0xEA => self.nop(),
            _ => panic!("Unrecognized opcode: {:#X}", opcode),
        };
        println!("");
    }

    fn load_next_byte_bump_pc(&mut self, cart: &mut Cartridge) -> u8 {
        self.registers.pc += 1;
        cart.prg_read(self.registers.pc - 1)
    }

    fn load_next_word_bump_pc(&mut self, cart: &mut Cartridge) -> u16 {
        self.load_next_byte_bump_pc(cart) as u16 | (self.load_next_byte_bump_pc(cart) as u16) << 8
    }

    // For use by debugger
    pub fn pc(&self) -> u16 {
        self.registers.pc
    }

    // Public for debugger
    pub fn fetch_byte(&mut self, cart: &mut Cartridge, addr: u16) -> u8 {
        match addr {
            0x0...0x07FF => self.ram[addr as usize],
            0x0800...0x1FFF => self.ram[(addr % 0x0800) as usize],
            0x2000...0x2007 => unimplemented!(), // PPU registers
            0x2008...0x3FFF => unimplemented!(), // PPU register mirrors
            0x4000...0x4017 => unimplemented!(), // APU registers
            0x4018...0x401F => panic!("These registers are disabled during normal operation"),
            0x4020...0xFFFF => cart.prg_read(addr),
            _ => unreachable!(),
        }
    }

    fn fetch_word(&mut self, cart: &mut Cartridge, addr: u16) -> u16 {
        // The 6502 is little endian (LSB first)
        ((self.fetch_byte(cart, addr + 1) as u16) << 8) | self.fetch_byte(cart, addr) as u16
    }

    fn store(&mut self, cart: &mut Cartridge, addr: u16, val: u8) {
        match addr {
            0x0...0x07FF => self.ram[addr as usize] = val,
            0x0800...0x1FFF => self.ram[(addr % 0x0800) as usize] = val,
            0x2000...0x2007 => unimplemented!(), // PPU registers
            0x2008...0x3FFF => unimplemented!(), // PPU register mirrors
            0x4000...0x4017 => unimplemented!(), // APU registers
            0x4018...0x401F => panic!("These registers are disabled during normal operation"),
            0x4020...0xFFFF => cart.prg_write(addr, val),
            _ => unreachable!(),
        }
    }

    fn stack_push_byte(&mut self, val: u8) {
        self.ram[(self.registers.s as usize) + 0x100] = val;
        self.registers.s -= 1;
    }

    fn stack_push_word(&mut self, val: u16) {
        let high_byte = ((val >> 8) & 0xFF) as u8;
        let low_byte = (val & 0xFF) as u8;
        self.stack_push_byte(high_byte);
        self.stack_push_byte(low_byte);
    }

    fn stack_pop_byte(&mut self) -> u8 {
        self.registers.s += 1;
        let val = self.ram[(self.registers.s as usize) + 0x100];
        val
    }

    fn stack_pop_word(&mut self) -> u16 {
        let low_byte = self.stack_pop_byte();
        let high_byte = self.stack_pop_byte();
        ((high_byte as u16) << 8) | (low_byte as u16)
    }

    fn branch(&mut self, am: RelativeAM, flag: ProcessorFlags, set: bool, branch_type: &str) {
        let offset_addr = self.registers.pc.wrapping_add((am.arg as i8) as u16);
        print!(" {:} ${:4X}{:23}{:?} CYC:{:}", branch_type, offset_addr, " ", self.registers, self.cycles);
        match set {
            true => {
                if self.registers.p.contains(flag) {
                    self.registers.pc = offset_addr;
                }
            }
            false => {
                if !self.registers.p.contains(flag) {
                    self.registers.pc = offset_addr;
                }
            }
        }
    }

    // INSTRUCTIONS
    // Branches
    fn bcs(&mut self, am: RelativeAM) {
        self.branch(am, ProcessorFlags::CARRY, true, "BCS");
    }

    fn bcc(&mut self, am: RelativeAM) {
        self.branch(am, ProcessorFlags::CARRY, false, "BCC");
    }

    fn beq(&mut self, am: RelativeAM) {
        self.branch(am, ProcessorFlags::ZERO, true, "BEQ");
    }

    fn bne(&mut self, am: RelativeAM) {
        self.branch(am, ProcessorFlags::ZERO, false, "BNE");
    }

    fn bvs(&mut self, am: RelativeAM) {
        self.branch(am, ProcessorFlags::OVERFLOW, true, "BVS");
    }

    fn bvc(&mut self, am: RelativeAM) {
        self.branch(am, ProcessorFlags::OVERFLOW, false, "BVC");
    }

    fn bpl(&mut self, am: RelativeAM) {
        self.branch(am, ProcessorFlags::NEGATIVE, false, "BPL");
    }

    fn bmi(&mut self, am: RelativeAM) {
        self.branch(am, ProcessorFlags::NEGATIVE, true, "BMI");
    }

    // Flag sets
    fn sec(&mut self) {
        print!(" SEC{:29}{:?} CYC:{:}", " ", self.registers, self.cycles);
        self.registers.p.set(ProcessorFlags::CARRY, true);
    }

    fn sei(&mut self) {
        print!(" SEI{:29}{:?} CYC:{:}", " ", self.registers, self.cycles);
        self.registers.p.set(ProcessorFlags::INTERRUPT, true);
    }

    fn sed(&mut self) {
        print!(" SED{:29}{:?} CYC:{:}", " ", self.registers, self.cycles);
        self.registers.p.set(ProcessorFlags::DECIMAL, true);
    }

    // Flag clears
    fn clc(&mut self) {
        print!(" CLC{:29}{:?} CYC:{:}", " ", self.registers, self.cycles);
        self.registers.p.set(ProcessorFlags::CARRY, false);
    }

    fn cld(&mut self) {
        print!(" CLD{:29}{:?} CYC:{:}", " ", self.registers, self.cycles);
        self.registers.p.set(ProcessorFlags::DECIMAL, false);
    }

    fn clv(&mut self) {
        print!(" CLV{:29}{:?} CYC:{:}", " ", self.registers, self.cycles);
        self.registers.p.set(ProcessorFlags::OVERFLOW, false);
    }

    // Stack
    fn php(&mut self) {
        print!(" PHP{:29}{:?} CYC:{:}", " ", self.registers, self.cycles);
        let p = self.registers.p.bits() | ProcessorFlags::STACK_COPY.bits();
        self.stack_push_byte(p);
    }

    fn pla(&mut self) {
        print!(" PLA{:29}{:?} CYC:{:}", " ", self.registers, self.cycles);
        self.registers.a = self.stack_pop_byte();
        self.registers.p.set(ProcessorFlags::ZERO, self.registers.a == 0);
        self.registers.p.set(ProcessorFlags::NEGATIVE, self.registers.a & (1 << 7) != 0);
    }

    fn pha(&mut self) {
        print!(" PHA{:29}{:?} CYC:{:}", " ", self.registers, self.cycles);
        let a = self.registers.a;
        self.stack_push_byte(a);
    }

    fn plp(&mut self) {
        print!(" PLP{:29}{:?} CYC:{:}", " ", self.registers, self.cycles);
        let p = self.stack_pop_byte();
        self.registers.p = ProcessorFlags::from_bits(p & 0b1110_1111).unwrap(); // PLP ignores bit 5
        self.registers.p.set(ProcessorFlags::ALWAYS_SET, true);
    }

    // ALU Ops
    fn adc<AM: AddressingMode>(&mut self, cart: &mut Cartridge, am: AM) {
        print!(" ADC ");
        let a = self.registers.a;
        let m = am.load(self, cart);
        let c = if self.registers.p.contains(ProcessorFlags::CARRY) { 1 } else { 0 };
        let result = a.wrapping_add(m).wrapping_add(c);
        self.registers.a = result;
        self.registers.p.set(ProcessorFlags::ZERO, result == 0);
        self.registers.p.set(ProcessorFlags::OVERFLOW, (!(a ^ m) & (a ^ result) & 0x80) != 0);
        self.registers.p.set(ProcessorFlags::CARRY, (a as u16 + m as u16 + c as u16) > (result as u16));
        self.registers.p.set(ProcessorFlags::NEGATIVE, result & (1 << 7) != 0);
    }

    fn and<AM: AddressingMode>(&mut self, cart: &mut Cartridge, am: AM) {
        print!(" AND ");
        let a = self.registers.a & am.load(self, cart);
        self.registers.a = a;
        self.registers.p.set(ProcessorFlags::ZERO, a == 0);
        self.registers.p.set(ProcessorFlags::NEGATIVE, a & (1 << 7) != 0);
    }

    fn bit<AM: AddressingMode>(&mut self, cart: &mut Cartridge, am: AM) {
        print!(" BIT ");
        let a = self.registers.a;
        let m = am.load(self, cart);
        self.registers.p.set(ProcessorFlags::ZERO, a & m == 0);
        self.registers.p.set(ProcessorFlags::OVERFLOW, m & (1 << 6) != 0);
        self.registers.p.set(ProcessorFlags::NEGATIVE, m & (1 << 7) != 0);
    }

    fn cmp<AM: AddressingMode>(&mut self, cart: &mut Cartridge, am: AM) {
        print!(" CMP ");
        let a = self.registers.a;
        let m = am.load(self, cart);
        self.registers.p.set(ProcessorFlags::CARRY, a >= m);
        self.registers.p.set(ProcessorFlags::ZERO, a == m);
        self.registers.p.set(ProcessorFlags::NEGATIVE, a.wrapping_sub(m) & (1 << 7) != 0);
    }

    fn cpx<AM: AddressingMode>(&mut self, cart: &mut Cartridge, am: AM) {
        print!(" CPX ");
        let x = self.registers.x;
        let m = am.load(self, cart);
        self.registers.p.set(ProcessorFlags::CARRY, x >= m);
        self.registers.p.set(ProcessorFlags::ZERO, x == m);
        self.registers.p.set(ProcessorFlags::NEGATIVE, x.wrapping_sub(m) & (1 << 7) != 0);
    }

    fn cpy<AM: AddressingMode>(&mut self, cart: &mut Cartridge, am: AM) {
        print!(" CPY ");
        let y = self.registers.y;
        let m = am.load(self, cart);
        self.registers.p.set(ProcessorFlags::CARRY, y >= m);
        self.registers.p.set(ProcessorFlags::ZERO, y == m);
        self.registers.p.set(ProcessorFlags::NEGATIVE, y.wrapping_sub(m) & (1 << 7) != 0);
    }

    fn eor<AM: AddressingMode>(&mut self, cart: &mut Cartridge, am: AM) {
        print!(" EOR ");
        let a = self.registers.a;
        let m = am.load(self, cart);
        self.registers.a = a ^ m;
        self.registers.p.set(ProcessorFlags::ZERO, self.registers.a == 0);
        self.registers.p.set(ProcessorFlags::NEGATIVE, self.registers.a & (1 << 7) != 0);
    }

    fn ora<AM: AddressingMode>(&mut self, cart: &mut Cartridge, am: AM) {
        print!(" ORA ");
        let a = self.registers.a;
        let m = am.load(self, cart);
        self.registers.a = a | m;
        self.registers.p.set(ProcessorFlags::ZERO, self.registers.a == 0);
        self.registers.p.set(ProcessorFlags::NEGATIVE, self.registers.a & (1 << 7) != 0);
    }

    fn sbc<AM: AddressingMode>(&mut self, cart: &mut Cartridge, am: AM) {
        print!(" SBC ");
        let a = self.registers.a;
        let m = !am.load(self, cart);
        let c = if self.registers.p.contains(ProcessorFlags::CARRY) { 1 } else { 0 };
        let result = a.wrapping_add(m).wrapping_add(c);
        self.registers.a = result;
        self.registers.p.set(ProcessorFlags::ZERO, result == 0);
        self.registers.p.set(ProcessorFlags::OVERFLOW, (!(a ^ m) & (a ^ result) & 0x80) != 0);
        self.registers.p.set(ProcessorFlags::CARRY, (a as u16 + m as u16 + c as u16) > (result as u16));
        self.registers.p.set(ProcessorFlags::NEGATIVE, result & (1 << 7) != 0);
    }

    // Increments and decrements
    fn inx(&mut self) {
        print!(" INX{:29}{:?} CYC:{:}", " ", self.registers, self.cycles);
        let x = self.registers.x.wrapping_add(1);
        self.registers.x = x;
        self.registers.p.set(ProcessorFlags::ZERO, self.registers.x == 0);
        self.registers.p.set(ProcessorFlags::NEGATIVE, self.registers.x & (1 << 7) != 0);
    }

    fn iny(&mut self) {
        print!(" INY{:29}{:?} CYC:{:}", " ", self.registers, self.cycles);
        let y = self.registers.y.wrapping_add(1);
        self.registers.y = y;
        self.registers.p.set(ProcessorFlags::ZERO, self.registers.y == 0);
        self.registers.p.set(ProcessorFlags::NEGATIVE, self.registers.y & (1 << 7) != 0);
    }

    fn dex(&mut self) {
        print!(" DEX{:29}{:?} CYC:{:}", " ", self.registers, self.cycles);
        let x = self.registers.x.wrapping_sub(1);
        self.registers.x = x;
        self.registers.p.set(ProcessorFlags::ZERO, self.registers.x == 0);
        self.registers.p.set(ProcessorFlags::NEGATIVE, self.registers.x & (1 << 7) != 0);
    }

    fn dey(&mut self) {
        print!(" DEY{:29}{:?} CYC:{:}", " ", self.registers, self.cycles);
        let y = self.registers.y.wrapping_sub(1);
        self.registers.y = y;
        self.registers.p.set(ProcessorFlags::ZERO, self.registers.y == 0);
        self.registers.p.set(ProcessorFlags::NEGATIVE, self.registers.y & (1 << 7) != 0);
    }

    // Transfers
    fn tax(&mut self) {
        print!(" TAX{:29}{:?} CYC:{:}", " ", self.registers, self.cycles);
        self.registers.x = self.registers.a;
        self.registers.p.set(ProcessorFlags::ZERO, self.registers.x == 0);
        self.registers.p.set(ProcessorFlags::NEGATIVE, self.registers.x & (1 << 7) != 0);
    }

    fn tay(&mut self) {
        print!(" TAY{:29}{:?} CYC:{:}", " ", self.registers, self.cycles);
        self.registers.y = self.registers.a;
        self.registers.p.set(ProcessorFlags::ZERO, self.registers.y == 0);
        self.registers.p.set(ProcessorFlags::NEGATIVE, self.registers.y & (1 << 7) != 0);
    }

    fn tsx(&mut self) {
        print!(" TSX{:29}{:?} CYC:{:}", " ", self.registers, self.cycles);
        self.registers.x = self.registers.s;
        self.registers.p.set(ProcessorFlags::ZERO, self.registers.x == 0);
        self.registers.p.set(ProcessorFlags::NEGATIVE, self.registers.x & (1 << 7) != 0);
    }

    fn txa(&mut self) {
        print!(" TXA{:29}{:?} CYC:{:}", " ", self.registers, self.cycles);
        self.registers.a = self.registers.x;
        self.registers.p.set(ProcessorFlags::ZERO, self.registers.a == 0);
        self.registers.p.set(ProcessorFlags::NEGATIVE, self.registers.a & (1 << 7) != 0);
    }

    fn txs(&mut self) {
        print!(" TXS{:29}{:?} CYC:{:}", " ", self.registers, self.cycles);
        self.registers.s = self.registers.x;
    }

    fn tya(&mut self) {
        print!(" TYA{:29}{:?} CYC:{:}", " ", self.registers, self.cycles);
        self.registers.a = self.registers.y;
        self.registers.p.set(ProcessorFlags::ZERO, self.registers.a == 0);
        self.registers.p.set(ProcessorFlags::NEGATIVE, self.registers.a & (1 << 7) != 0);
    }

    // Loads
    fn lda<AM: AddressingMode>(&mut self, cart: &mut Cartridge, am: AM) {
        print!(" LDA ");
        self.registers.a = am.load(self, cart);
        self.registers.p.set(ProcessorFlags::ZERO, self.registers.a == 0);
        self.registers.p.set(ProcessorFlags::NEGATIVE, self.registers.a & (1 << 7) != 0 );
    }

    fn ldx<AM: AddressingMode>(&mut self, cart: &mut Cartridge, am: AM) {
        print!(" LDX ");
        self.registers.x = am.load(self, cart);
        self.registers.p.set(ProcessorFlags::ZERO, self.registers.x == 0);
        self.registers.p.set(ProcessorFlags::NEGATIVE, self.registers.x & (1 << 7) != 0);
    }

    fn ldy<AM: AddressingMode>(&mut self, cart: &mut Cartridge, am: AM) {
        print!(" LDY ");
        self.registers.y = am.load(self, cart);
        self.registers.p.set(ProcessorFlags::ZERO, self.registers.y == 0);
        self.registers.p.set(ProcessorFlags::NEGATIVE, self.registers.y & (1 << 7) != 0);
    }

    // Stores
    fn sta<AM: AddressingMode>(&mut self, cart: &mut Cartridge, am: AM) {
        print!(" STA ");
        let a = self.registers.a;
        am.store(self, cart, a);
    }

    fn stx<AM: AddressingMode>(&mut self, cart: &mut Cartridge, am: AM) {
        print!(" STX ");
        let x = self.registers.x;
        am.store(self, cart, x);
    }

    // Jumps are the only instructions that use absolute addressing, so they are given two methods
    fn jmp_indirect(&mut self, cart: &mut Cartridge) {
        let addr = self.load_next_word_bump_pc(cart);
        print!(" JMP ${:X}{:23}{:?} CYC:{:}", addr, " ", self.registers, self.cycles);
        self.registers.pc = addr;
    }

    fn jsr(&mut self, cart: &mut Cartridge) {
        let target = self.load_next_word_bump_pc(cart);
        print!(" JSR ${:X}{:23}{:?} CYC:{:}", target, " ", self.registers, self.cycles);
        let addr = self.registers.pc - 1;
        self.stack_push_word(addr);
        self.registers.pc = target;
    }

    fn rts(&mut self) {
        print!(" RTS {:28}{:?} CYC:{:}", " ", self.registers, self.cycles);
        self.registers.pc = self.stack_pop_word() + 1;
    }

    fn nop(&mut self) {
        print!(" NOP{:29}{:?} CYC:{:}", " ", self.registers, self.cycles);
    }
}

impl Cpu {
    fn debug_print(&mut self, cart: &mut Cartridge, op: u8) -> String {
        let pc = self.registers.pc;
        let debug_string = match op {
            // 2 byte instructions
            0x69 | 0x65 | 0x75 | 0x61 | 0x71 | 0x29 | 0x25 | 0x35 | 0x21 | 0x31 | 0x06 | 0x16
            | 0x90 | 0xB0 | 0xF0 | 0x24 | 0x30 | 0xD0 | 0x10 | 0x50 | 0x70 | 0xC9 | 0xC5 | 0xD5
            | 0xC1 | 0xD1 | 0xE0 | 0xE4 | 0xC0 | 0xC4 | 0xC6 | 0xD6 | 0x49 | 0x45 | 0x55 | 0x41
            | 0x51 | 0xE6 | 0xF6 | 0xA9 | 0xA5 | 0xB5 | 0xA1 | 0xB1 | 0xA2 | 0xA6 | 0xB6 | 0xA0
            | 0x46 | 0x56 | 0x09 | 0x05 | 0x15 | 0x01 | 0x11 | 0x26 | 0x36 | 0x66 | 0x76 | 0xE9
            | 0xE5 | 0xF5 | 0xE1 | 0xF1 | 0x85 | 0x95 | 0x81 | 0x91 | 0x86 | 0x96 | 0x84 | 0x94 => {
                format!("{:02X} {:02X}{:3}", op, self.fetch_byte(cart, pc), " ")
            }

            // 3 byte instructions
            0x6D | 0x7D | 0x79 | 0x2D | 0x3D | 0x39 | 0x0E | 0x1E | 0x2C | 0xCD | 0xDD | 0xD9
            | 0xEC | 0xCC | 0xCE | 0xDE | 0x4D | 0x5D | 0x59 | 0xEE | 0xFE | 0x4C | 0x6C | 0x20
            | 0xAD | 0xBD | 0xB9 | 0xAE | 0xBE | 0xAC | 0xBC | 0x4E | 0x5E | 0x0D | 0x1D | 0x19
            | 0x2E | 0x3E | 0x6E | 0x7E | 0xED | 0xFD | 0xF9 | 0x8D | 0x9D | 0x99 | 0x8E | 0x8C => {
                format!(
                    "{:02X} {:02X} {:02X}",
                    op,
                    self.fetch_byte(cart, pc),
                    self.fetch_byte(cart, pc + 1)
                )
            }

            // 1 byte instructions
            _ => format!("{:02X}{:6}", op, " "),
        };
        debug_string
    }
}
