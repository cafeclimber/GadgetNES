use std::fmt;
use super::apu::Apu;
use super::cart::Cartridge;
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

    pub fn read_instr(&self) -> u8 {
        self.cart.read_rom(self.pc as usize)
    }

    pub fn run_instr(&mut self, instr: u8) {
        match instr {
            // TODO: Put this in an enum to make printing easier?
            // TODO: Implement unofficial opcodes
            /*----------Control Operations--------*/
            0x00 => {/* BRK                      */},
            0x08 => {/* PHP                      */},
            0x10 => {/* BPL zero-page +          */},
            0x18 => {/* CLC                      */},
            0x20 => {/* JSR absolute             */},
            0x24 => {/* BIT zero-page            */},
            0x28 => {/* PLP                      */},
            0x2c => {/* BIT absolute             */},
            0x30 => {/* BMI zero-page +          */},
            0x38 => {/* SEC                      */},
            0x40 => {/* RTI                      */},
            0x48 => {/* PHA                      */},
            0x4c => {/* JMP absolute             */},
            0x50 => {/* BVC zero-page plus       */},
            0x58 => {/* CLU                      */},
            0x60 => {/* RTS                      */},
            0x68 => {/* PLA                      */},
            0x6c => {/* JMP indirect             */},
            0x70 => {/* BVS zero-page +          */},
            0x78 => {/* SEI                      */},
            0x84 => {/* STY zero-page            */},
            0x88 => {/* DEY                      */},
            0x8c => {/* STY absolute             */},
            0x90 => {/* BCC zero-page +          */},
            0x94 => {/* STY z-page indexed (x)   */},
            0x98 => {/* TYA                      */},
            0x9c => {/* SHY absolute indexed (x) */},
            0xa0 => {/* LDY immediate            */},
            0xa4 => {/* LDY zero-page            */},
            0xa8 => {/* TAY                      */},
            0xac => {/* LDY absolute             */},
            0xb0 => {/* BCS zero-page +          */},
            0xb4 => {/* LDY z-page indexed (x)   */},
            0xb8 => {/* CLV                      */},
            0xbc => {/* LDY absolute indexed (x) */},
            0xc0 => {/* CPY immediate            */},
            0xc4 => {/* CPY zero-page            */},
            0xc8 => {/* INY                      */},
            0xcc => {/* CPY absolute             */},
            0xd0 => {/* BNE zero-page +          */},
            0xd8 => {/* CLD                      */},
            0xe0 => {/* CPX immediate            */},
            0xe4 => {/* CPX zero-page            */},
            0xe8 => {/* INX                      */},
            0xec => {/* CPX absolute             */},
            0xf0 => {/* BEQ zero-page +          */},
            0xf8 => {/* SED                      */},

            /*------------ALU Operations----------*/
            0x01 => {/* ORA indexed indirect     */},
            0x05 => {/* ORA zero-page            */},
            0x09 => {/* ORA immediate            */},
            0x0d => {/* ORA absolute             */},
            0x11 => {/* ORA indirect indexed     */},
            0x15 => {/* ORA z-page indexed (x)   */},
            0x19 => {/* ORA absolute indexed (y) */},
            0x1d => {/* ORA absolute indexed (x) */},
            
            0x21 => {/* AND indexed indirect     */},
            0x25 => {/* AND zero-page            */},
            0x29 => {/* AND immediate            */},
            0x2d => {/* AND absolute             */},
            0x31 => {/* AND indirect indexed     */},
            0x35 => {/* AND z-page indexed (x)   */},
            0x39 => {/* AND absolute indexed (y) */},
            0x3d => {/* AND absolute indexed (x) */},
            
            0x41 => {/* EOR indexed indirect     */},
            0x45 => {/* EOR zero-page            */},
            0x49 => {/* EOR immediate            */},
            0x4d => {/* EOR absolute             */},
            0x51 => {/* EOR indirect indexed     */},
            0x55 => {/* EOR z-page indexed (x)   */},
            0x59 => {/* EOR absolute indexed (y) */},
            0x5d => {/* EOR absolute indexed (x) */},

            0x61 => {/* ADC indexed indirect     */},
            0x65 => {/* ADC zero-page            */},
            0x69 => {/* ADC immediate            */},
            0x6d => {/* ADC absolute             */},
            0x71 => {/* ADC indirect indexed     */},
            0x75 => {/* ADC z-page indexed (x)   */},
            0x79 => {/* ADC absolute indexed (y) */},
            0x7d => {/* ADC absolute indexed (x) */},

            0x81 => {/* STA indexed indirect     */},
            0x85 => {/* STA zero-page            */},
            0x8d => {/* STA absolute             */},
            0x91 => {/* STA indirect indexed     */},
            0x95 => {/* STA z-page indexed (x)   */},
            0x99 => {/* STA absolute indexed (y) */},
            0x9d => {/* STA absolute indexed (x) */},

            0xa1 => {/* LDA indexed indirect     */},
            0xa5 => {/* LDA zero-page            */},
            0xa9 => {/* LDA immediate            */},
            0xad => {/* LDA absolute             */},
            0xb1 => {/* LDA indirect indexed     */},
            0xb5 => {/* LDA z-page indexed (x)   */},
            0xb9 => {/* LDA absolute indexed (y) */},
            0xbd => {/* LDA absolute indexed (x) */},

            0xc1 => {/* CMP indexed indirect     */},
            0xc5 => {/* CMP zero-page            */},
            0xc9 => {/* CMP immediate            */},
            0xcd => {/* CMP absolute             */},
            0xd1 => {/* CMP indirect indexed     */},
            0xd5 => {/* CMP z-page indexed (x)   */},
            0xd9 => {/* CMP absolute indexed (y) */},
            0xdd => {/* CMP absolute indexed (x) */},

            0xe1 => {/* SBC indexed indirect     */},
            0xe5 => {/* SBC zero-page            */},
            0xe9 => {/* SBC immediate            */},
            0xed => {/* SBC absolute             */},
            0xf1 => {/* SBC indirect indexed     */},
            0xf5 => {/* SBC z-page indexed (x)   */},
            0xf9 => {/* SBC absolute indexed (y) */},
            0xfd => {/* SBC absolute indexed (x) */},

            /*----------Read-Modify-Write---------*/
            0x06 => {/* ASL zero-page            */},
            0x0a => {/* ASL                      */},
            0x0e => {/* ASL absolute             */},
            0x16 => {/* ASL z-page indexed (x)   */},
            0x1e => {/* ASL absolute indexed (x) */},

            0x26 => {/* ROL zero-page            */},
            0x2a => {/* ROL                      */},
            0x2e => {/* ROL absolute             */},
            0x36 => {/* ROL z-page indexed (x)   */},
            0x3e => {/* ROL absolute indexed (x) */},

            0x46 => {/* LSR zero-page            */},
            0x4a => {/* LSR                      */},
            0x4e => {/* LSR absolute             */},
            0x56 => {/* LSR z-page indexed (x)   */},
            0x5e => {/* LSR absolute indexed (x) */},

            0x66 => {/* ROR zero-page            */},
            0x6a => {/* ROR                      */},
            0x6e => {/* ROR absolute             */},
            0x76 => {/* ROR z-page indexed (x)   */},
            0x7e => {/* ROR absolute indexed (x) */},

            0x86 => {/* STX zero-page            */},
            0x8a => {/* TXA                      */},
            0x8e => {/* STX absolute             */},
            0x96 => {/* STX z-page indexed (y)   */},
            0x9a => {/* TXS                      */},

            0xa2 => {/* LDX immediate            */},
            0xa6 => {/* LDX zero-page            */},
            0xaa => {/* TAX                      */},
            0xae => {/* LDX absolute             */},
            0xb6 => {/* LDX z-page indexed (y)   */},
            0xba => {/* TSX                      */},
            0xbe => {/* LDX absolute indexed (y) */},

            0xc6 => {/* DEC zero-page            */},
            0xca => {/* DEX                      */},
            0xce => {/* DEC absolute             */},
            0xd6 => {/* DEC z-page indexed (x)   */},
            0xde => {/* DEC absolute indexed (x) */},

            0xe6 => {/* INC zero-page            */},
            0xee => {/* INC absolute             */},
            0xf6 => {/* INC z-page indexed (x)   */},
            0xfe => {/* INC absolute indexed (x) */},

            0xea => {/* NOP                      */},
            _    => panic!("Unrecognized instruction: {:#x}", instr),
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
