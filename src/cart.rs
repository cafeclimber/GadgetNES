#[derive(Default)]
pub struct Cartridge {
    prg_ram: Box<[u8]>,

    prg_rom: Box<[u8]>,

    chr_mem: Box<[u8]>,
}

impl Cartridge {
    pub fn load_cartridge(&mut self, cart_rom: Vec<u8>) {
        self.prg_rom = cart_rom.into_boxed_slice();
    }

    pub fn read_rom(&self, addr: usize) -> u8 {
        self.prg_rom[addr]
    }

    pub fn read_rom_word(&self, addr:usize) -> u16 {
        (self.prg_rom[addr + 1] as u16) << 8 |
        (self.prg_rom[addr] as u16)
    }
}
