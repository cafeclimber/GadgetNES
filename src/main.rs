use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

mod cpu;

fn main() {
    let rom_name = env::args().nth(1).unwrap();

    let rom = read_cartridge(rom_name);
}

// Thanks to yupferris for this!
fn read_cartridge<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let mut file = File::open(path).unwrap();
    let mut file_buf = Vec::new();
    file.read_to_end(&mut file_buf).unwrap();
    file_buf
}
