use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

mod nes;
mod cpu;
mod apu;
mod cart;

fn main() {
    let rom_name = env::args().nth(1).unwrap();

    let cart_rom = read_cartridge(rom_name);
    read_rom_header(&cart_rom);
    
    let mut nes = nes::Nes::new();
    nes.power_up(cart_rom);
}

// Thanks to yupferris for this!
fn read_cartridge<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let mut file = File::open(path).unwrap();
    let mut file_buf = Vec::new();
    file.read_to_end(&mut file_buf).unwrap();
    file_buf
}

fn read_rom_header(cart_rom: &Vec<u8>) {
    let header = ((cart_rom[0] as u32) << 16) | ((cart_rom[1] as u32) << 8) | (cart_rom[2] as u32); // TODO: Write better
    assert_eq!(0x4e4553, header); // "NES"

    let num_prg_rom_pages = cart_rom[3];
    let num_chr_rom_pages = cart_rom[4];

    let mapper = ((cart_rom[5] & 0b1111_0000) >> 4) | (cart_rom[6] & 0b1111_0000);
    
    let four_screen_mode = cart_rom[5] & (1 << 3);
    let trainer_present = cart_rom[5] & (1 << 2);
    let battery_backed = cart_rom[5] & (1 << 1);
    let mirroring = cart_rom[5] & 1;

    let playchoice = cart_rom[6] & (1 << 1); 
    let unisystem = cart_rom[6] & 1;
}
