use std::env;
use std::path::Path;

mod rom;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Expected 1 parameter. Got {:}", args.len() - 1);
        return;
    }

    let rom_path = Path::new(&args[1]);
    match rom::read_rom(rom_path) {
        Ok(rom) => {
            // TODO: Run emulator!
        }
        Err(e) => println!("{}", e),
    }
}
