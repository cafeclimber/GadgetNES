const RAM_SIZE: usize = 0x800;
const PRG_RAM_SIZE: usize = 0x2000;
const TODO_CHR_MEM: usize = 0x8000;

#[derive(Default)]
pub struct Memory {
    cpu_ram: Box<[u8]>,

    prg_ram: Box<[u8]>,

    prg_rom: Box<[u8]>,

    chr_mem: Box<[u8]>,
}

impl Memory {
    // TODO Implement chr_rom, prg_ram, and prg_rom
    // TODO make this function yield chr_rom and prg_ram as well
    pub fn load_cartridge(&mut self, cart_rom: Vec<u8>) {
        self.cpu_ram = vec![0u8; RAM_SIZE].into_boxed_slice();
        self.prg_ram = vec![0u8; PRG_RAM_SIZE].into_boxed_slice();
        self.prg_rom = cart_rom.into_boxed_slice();
        self.chr_mem = vec![0u8; TODO_CHR_MEM].into_boxed_slice();
    }

    pub fn read_instr(&self, addr: u16) -> u8 {
        self.prg_rom[addr as usize]
    }

    pub fn read_ram(&self, addr: u16) -> u8 {
        self.cpu_ram[addr as usize]
    }

    // For fetching rest of instructions
    pub fn rom_byte(&self, addr: u16) -> u8 {
        self.prg_rom[addr as usize]
    }

    pub fn rom_word(&self, addr: u16) -> u16 {
        (self.prg_rom[(addr+1) as usize] as u16) << 8 |
        self.prg_rom[addr as usize] as u16
    }
}
