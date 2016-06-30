use super::interconnect::Interconnect;

// TODO: extract info
pub struct Cartridge {
    prg_rom: Box<[u8]>,
}

impl Cartridge {
    pub fn new(cart_rom: Vec<u8>) -> Cartridge {
        Cartridge {
            prg_rom: cart_rom.into_boxed_slice(),
        }
    }

    pub fn read_cart(&self, addr: u16) -> u8 {
        println!("Read cart addr: {:#x}", addr);
        self.prg_rom[addr as usize]
    }
}
