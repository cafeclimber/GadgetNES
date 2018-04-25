use std::io::{self, Write};
use std::collections::HashMap;

mod parser;

use nes::Nes;
use self::parser::Command;

pub struct Debugger<'a> {
    nes: Nes<'a>,
    pub breakpoints: HashMap<usize, usize>,
}

impl<'a> Debugger<'a> {
    pub fn init(nes: Nes<'a>) -> Self {
        let mut debugger = Debugger {
            nes: nes,
            breakpoints: HashMap::new(),
        };
        debugger.reset();
        debugger
    }

    pub fn run(&mut self){
        let mut next_key = 1;
        loop {
            print!("Gidget>");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let command = input.trim().parse::<Command>();
            match command {
                Ok(Command::Step(num_steps)) => self.step(num_steps),
                Ok(Command::Run) => self.step_forever(),
                Ok(Command::Breakpoint(addr)) => self.set_breakpoint(&mut next_key, addr as usize),
                Ok(Command::ListBreakPoints) => self.list_breakpoints(),
                Ok(Command::ClearBreakpoint(num)) => self.clear_bp(&num),
                Ok(Command::Print(addr)) => self.print(addr as usize),
                Ok(Command::PrintRange(low_addr, high_addr)) => self.print_range(low_addr as usize, high_addr as usize),
                Ok(Command::Help) => self.help(),
                Ok(Command::Quit) => break,
                Err(ref e) => println!("{}", e),
            }
        }
    }

    pub fn reset(&mut self) {
        self.nes.reset();
    }

    fn step(&mut self, num_steps: usize) {
        for _ in 0..num_steps {
            let opcode = self.nes.step();
            print!("{:02X} {}", opcode, opcode_to_name(opcode));
            println!("{:?}", self.nes.cpu);
        }
    }

    fn step_forever(&mut self) {
        self.nes.run(Some(&self.breakpoints));
    }

    fn set_breakpoint(&mut self, key: &mut usize, addr: usize) {
        self.breakpoints.insert(*key, addr);
        *key += 1;
    }

    fn list_breakpoints(&mut self) {
        for (key, addr) in self.breakpoints.iter() {
            println!("{:}\t${:04X}", key, addr);
        }
    }

    fn clear_bp(&mut self, key: &usize) {
        self.breakpoints.remove(key);
    }

    fn print(&mut self, addr: usize) {
        if addr <= 0xFFFF {
            println!("M[${:04X}] = {:X}", addr, self.nes.cpu.fetch_byte(&mut self.nes.interconnect, addr as u16));
        } else {
            println!("Invalid address: ${:X}", addr);
        }
    }

    fn print_range(&mut self, low_addr: usize, high_addr: usize) {
        if low_addr > 0xFFFF || high_addr > 0xFFFF {
            println!("Invalid address: LOW: ${:X} HIGH: {:X}", low_addr, high_addr);
        } else if low_addr > high_addr {
            println!("Low address higher than high address: LOW: ${:X} HIGH: ${:X}", low_addr, high_addr);
        } else {
            for (i, addr) in (low_addr..high_addr).enumerate() {
                if i % 16 == 0 { print!("\n${:04X}| ", addr); io::stdout().flush().unwrap(); }
                print!("{:02X} ", self.nes.cpu.fetch_byte(&mut self.nes.interconnect, addr as u16));
            }
            print!("\n");
            io::stdout().flush().unwrap();
        }
    }

    fn help(&self) {
        println!("GIDGET DEBUGGER");
        println!("Usage: COMMAND (SHORTCUT) <Args>");
        println!("\tbreak\t\t(b)\t<Address>\t\t\t- Sets breakpoint at specified address");
        println!("\tlist\t\t(l)\t\t\t\t\t- Lists all active breakpoints");
        println!("\tclear\t\t(cb)\t<Breakpoint Number>\t\t- Clears specified breakpoint");
        println!("\tprint\t\t(p)\t<Address>\t\t\t- Prints value in memory at specified address");
        println!("\tpr\t\t\t<Low Address>:<High Address>\t- Prints the values over the specified range of memory");
        println!("\tstep\t\t(s)\t<Steps>\t\t\t\t- Steps the NES the specified number of times (empty steps 1)");
        println!("\trun/continue\t(r/c)\t\t\t\t\t- Runs the NES as normal");
        println!("\tquit\t\t(q)\t\t\t\t\t- Quits the debugger");
        println!("\thelp\t\t(h)\t\t\t\t\t- Prints this help message");
    }
}

fn opcode_to_name(opcode: u8) -> String {
    format!("{:20}",
        match opcode {
            // Branches
            0x10 => "BPL (label)", 0x30 => "BMI (label)", 0x50 => "BVC (label)",
            0x70 => "BVS (label)", 0x90 => "BCC (label)", 0xB0 => "BCS (label)",
            0xD0 => "BNE (label)", 0xF0 => "BEQ (label)",

            // ALU operations
            0x61 => "ADC (d,x)", 0x65 => "ADC d",   0x69 => "ADC #v",  0x6D => "ADC a",
            0x71 => "ADC (d),y", 0x75 => "ADC d,x", 0x79 => "ADC a,y", 0x7D => "ADC a,x",

            0x21 => "AND (d,x)", 0x25 => "AND d",   0x29 => "AND #v",  0x2D => "AND a",
            0x31 => "AND (d),y", 0x35 => "AND d,x", 0x39 => "AND a,y", 0x3D => "AND a,x",

            0x06 => "ASL d",     0x0A => "ASL A",   0x0E => "ASL a",   0x16 => "ASL d,x",
            0x1E => "ASL a,x",

            0x24 => "BIT d",     0x2C => "BIT a",

            0xC1 => "CMP (d,x)", 0xC5 => "CMP d",   0xC9 => "CMP #v",  0xCD => "CMP a",
            0xD1 => "CMP (d),y", 0xD5 => "CMP d,x", 0xD9 => "CMP a,y", 0xDD => "CMP a,x",

            0xE0 => "CPX #v",    0xE4 => "CPX d",   0xEC => "CPX a",

            0xC0 => "CPY #v",    0xC4 => "CPY d",   0xCC => "CPY a",

            0x41 => "EOR (d,x)", 0x45 => "EOR d",   0x49 => "EOR #v",  0x4D => "EOR a",
            0x51 => "EOR (d),y", 0x55 => "EOR d,x", 0x59 => "EOR a,y", 0x5D => "EOR a,x",

            0x4A => "LSR A",     0x46 => "LSR d",   0x4E => "LSR a",   0x56 => "LSR d,x",
            0x5E => "LSR a,x",

            0x01 => "ORA (d,x)", 0x05 => "ORA d",   0x09 => "ORA #v",  0x0D => "ORA a",
            0x11 => "ORA (d),y", 0x15 => "ORA d,x", 0x19 => "ORA a,y", 0x1D => "ORA a,x",

            0x2A => "ROL A",     0x26 => "ROL d",   0x2E => "ROL a",   0x36 => "ROL d,x",
            0x3E => "ROL a,x",

            0x6A => "ROR A",     0x66 => "ROR d",   0x6E => "ROR a",   0x76 => "ROR d,x",
            0x7E => "ROR a,x",

            0xE1 => "SBC (d,x)", 0xE5 => "SBC d",   0xE9 => "SBC #v",  0xED => "SBC a",
            0xF1 => "SBC (d),y", 0xF5 => "SBC d,x", 0xF9 => "SBC a,y", 0xFD => "SBC a,x",

            // Increments and Decrements
            0xE6 => "INC d", 0xEE => "INC a", 0xF6 => "INC d,x", 0xFE => "INC a,x",
            0xE8 => "INX",   0xC8 => "INY",

            0xC6 => "DEC d", 0xCE => "DEC a", 0xD6 => "DEC d,x", 0xDE => "DEC a,x",
            0xCA => "DEX",   0x88 => "DEY",

            // Loads
            0xA1 => "LDA (d,x)", 0xA5 => "LDA d",   0xA9 => "LDA #v",  0xAD => "LDA a",
            0xB1 => "LDA (d),y", 0xB5 => "LDA d,x", 0xB9 => "LDA a,y", 0xBD => "LDA a,x",

            0xA2 => "LDX #v", 0xA6 => "LDX d", 0xAE => "LDX a", 0xB6 => "LDX d,y",
            0xBE => "LDX a,y",

            0xA0 => "LDY #v", 0xA4 => "LDY d", 0xAC => "LDY a", 0xB4 => "LDY d,x",
            0xBC => "LDY a,x",

            // Stores
            0x81 => "STA (d,x)", 0x85 => "STA d",   0x8D => "STA a",   0x91 => "STA (d),y",
            0x95 => "STA d,x",   0x99 => "STA a,y", 0x9D => "kTA a,x",

            0x86 => "STX d", 0x8E => "STX a", 0x96 => "STX d,y",

            0x84 => "STY d", 0x8C => "STY a", 0x94 => "STY d,x",

            // Flag sets
            0x38 => "SEC", 0x78 => "SEI", 0xF8 => "SED",

            // Flag clears
            0x18 => "CLC", 0xB8 => "CLV", 0xD8 => "CLD",

            // Stack
            0x08 => "PHP", 0x28 => "PLP", 0x48 => "PHA", 0x68 => "PLA",

            // Transfers
            0xAA => "TAX", 0xA8 => "TAY", 0xBA => "TSX", 0x8A => "TXA", 0x9A => "TXS", 0x98 => "TYA",

            // Jumps
            0x4C => "JMP a", 0x6C => "JMP (a)", 0x20 => "JSR", 0x40 => "RTI", 0x60 => "RTS",

            0xEA => "NOP",

            _ => unreachable!(),
        }
    )
}
