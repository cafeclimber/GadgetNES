use super::apu::Apu;
use super::ppu::Ppu;
use super::mapper::*;
use super::cart::Cartridge;

pub struct Interconnect {
    ram: Box<[u8]>,
    apu: Apu,
    ppu: Ppu,
    cart: Cartridge,
}

impl Interconnect {
    // TODO Implement chr_rom, prg_ram, and prg_rom
    pub fn new(cart_rom: &Vec<u8>) -> Interconnect {
        Interconnect {
            ram: vec![0; 0x0800].into_boxed_slice(),
            apu: Apu::default(),
            ppu: Ppu::default(),
            cart: Cartridge::new(cart_rom),
        }
    }

    pub fn power_up(&mut self, cart_rom: Vec<u8>) {
        self.cart.mapper.load_rom(cart_rom);
        self.ppu.power_up();
    }

    // PRETTIFYME: Get rid of magic constants
    pub fn read_byte(&self, virt_addr: u16) -> u8 {
        use super::mem_map::*;
        let phys_addr = map_virt_addr(virt_addr);
        match phys_addr {
            PhysAddr::CpuRam(addr) => {self.ram[addr as usize]},
            PhysAddr::RamMirrorOne(addr) => {self.ram[(addr - 0x0800) as usize]},
            PhysAddr::RamMirrorTwo(addr) => {self.ram[(addr - 2 * 0x0800) as usize]},
            PhysAddr::RamMirrorThree(addr) => {self.ram[(addr - 3 * 0x0800) as usize]},
            PhysAddr::PpuRegs(addr) => {self.ppu.read_reg(addr - 0x2000)},
            PhysAddr::PpuMirrors(addr) => {self.ppu.read_reg((addr - 0x2000) % 8)},
            PhysAddr::ApuRegs(addr) => {self.apu.read_reg(addr - 0x4000)},
            PhysAddr::CartSpace(addr) => {self.cart.read_cart(addr)},
        }
    }

    // PRETTIFYME: Get rid of magic constants
    pub fn write_byte(&mut self, virt_addr: u16, val: u8) {
        use super::mem_map::*;
        let phys_addr = map_virt_addr(virt_addr);
        match phys_addr {
            PhysAddr::CpuRam(addr) => {self.ram[addr as usize] = val;},
            PhysAddr::RamMirrorOne(addr) => {self.ram[(addr - 0x0800) as usize] = val;},
            PhysAddr::RamMirrorTwo(addr) => {self.ram[(addr - 2 * 0x0800) as usize] = val;},
            PhysAddr::RamMirrorThree(addr) => {self.ram[(addr - 3 * 0x0800) as usize] = val;},
            PhysAddr::PpuRegs(addr) => {self.ppu.write_to_reg(addr - 0x2000, val)},
            PhysAddr::PpuMirrors(addr) => {self.ppu.write_to_reg((addr - 0x2000) % 8, val)},
            PhysAddr::ApuRegs(addr) => {self.apu.write_to_reg(addr - 0x4000, val)},
            PhysAddr::CartSpace(addr) => {self.cart.write_byte_to_cart(addr, val);},
        }
    }

    pub fn write_word(&mut self, virt_addr: u16, val: u16) {
        use super::mem_map::*;
        let phys_addr = map_virt_addr(virt_addr);
        match phys_addr {
            PhysAddr::CpuRam(addr) => {self.ram[addr as usize] = (val & 0x00ff) as u8;
                                       self.ram[(addr + 1) as usize] = ((val & 0xff00) >> 8) as u8;},
            PhysAddr::RamMirrorOne(addr) => {self.ram[(addr - 0x0800)as usize] = (val & 0x00ff) as u8;
                                             self.ram[(addr + 1 - 0x0800) as usize] = ((val & 0xff00) >> 8) as u8;},
            PhysAddr::RamMirrorTwo(addr) => {self.ram[(addr - 2 * 0x0800)as usize] = (val & 0x00ff) as u8;
                                             self.ram[(addr + 1 - 2 * 0x0800) as usize] = ((val & 0xff00) >> 8) as u8;},
            PhysAddr::RamMirrorThree(addr) => {self.ram[(addr - 2 * 0x0800)as usize] = (val & 0x00ff) as u8;
                                             self.ram[(addr + 1 - 2 * 0x0800) as usize] = ((val & 0xff00) >> 8) as u8;},
            _ => panic!("Attempt to write word to unsupported location: {:?}", phys_addr),
        }
    }

    // PRETTIFYME: Get rid of magic constants
    pub fn read_word(&self, virt_addr: u16) -> u16 {
        use super::mem_map::*;
        let phys_addr = map_virt_addr(virt_addr);
        match phys_addr {
            PhysAddr::CpuRam(addr) =>         {self.ram[addr as usize] as u16 |
                                               (self.ram[(addr + 1) as usize] as u16) << 8},
            PhysAddr::RamMirrorOne(addr) =>   {self.ram[(addr - 0x0800) as usize] as u16 |
                                               (self.ram[(addr + 1 -0x0800) as usize] as u16) << 8},
            PhysAddr::RamMirrorTwo(addr) =>   {self.ram[(addr - 2 * 0x0800) as usize] as u16 |
                                               (self.ram[(addr + 1 - (2 * 0x0800)) as usize] as u16) << 8},
            PhysAddr::RamMirrorThree(addr) => {self.ram[(addr - 3 * 0x0800) as usize] as u16 |
                                               (self.ram[(addr + 1 - (3 * 0x0800)) as usize] as u16) << 8},
            PhysAddr::CartSpace(addr) => {self.cart.read_cart(addr) as u16 | (self.cart.read_cart(addr + 1) as u16) << 8},
            _ => panic!("{:?} does not support reading words", phys_addr),
        }
    }
}
