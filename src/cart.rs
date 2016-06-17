#[derive(Default)]
pub struct Cartridge {
    prg_ram: Box<[u8]>,

    prg_rom: Box<[u8]>,
}

impl Cartridge {
    pub fn load_cartridge(&mut self, cart_rom: Vec<u8>) {
        self.prg_rom = cart_rom.into_boxed_slice();
    }
}
