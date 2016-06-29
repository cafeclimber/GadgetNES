use super::apu::Apu;
use super::ppu::Ppu;
use super::cart::Cartridge;

pub struct Interconnect {
    ram: Box<[u8]>,
    apu: Apu,
    ppu: Ppu,
    cart: Cartridge,
}

impl Interconnect {
    // TODO Implement chr_rom, prg_ram, and prg_rom
    pub fn new(cart_rom: Vec<u8>) -> Interconnect {
        Interconnect {
            ram: vec![0; 0x0800].into_boxed_slice(),
            apu: Apu::default(),
            ppu: Ppu::default(),
            cart: Cartridge::new(cart_rom),
        }
    }

    pub fn read_byte(&self, virt_addr: u16) -> u8 {
        use super::mem_map::*;
        let phys_addr = map_virt_addr(virt_addr);
        match phys_addr {
            PhysAddr::CpuRam(addr) => {},
            PhysAddr::RamMirrorOne(addr) => {},
            PhysAddr::RamMirrorTwo(addr) => {},
            PhysAddr::RamMirrorThree(addr) => {},
            PhysAddr::PpuRegs(addr) => {},
            PhysAddr::PpuMirrors(addr) => {},
            PhysAddr::ApuRegs(addr) => {},
            PhysAddr::CartSpace(addr) => {},
        }
    }
}
