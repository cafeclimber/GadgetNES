#[derive(Default)]
pub struct Cartridge {
    prg_ram: Box<[u8]>,

    prg_rom: Box<[u8]>,
}

impl Cartridge {
    pub fn get_prg_rom_size(){} // TODO
}
