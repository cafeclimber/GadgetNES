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
            self.nes.step();
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
            println!("M[${:04X}] = {:X}", addr, self.nes.cpu.fetch_byte(&mut self.nes.cart, addr as u16));
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
                print!("{:02X} ", self.nes.cpu.fetch_byte(&mut self.nes.cart, addr as u16));
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
