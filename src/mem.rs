// ==== Memory Map =====
const RAM_BEG: u16 = 0x0000;
const RAM_SIZE: usize = 0x0800;
const RAM_END: u16 = 0x07ff;

const RAM_MIRROR_ONE_BEG: u16 = 0x0800;
const RAM_MIRROR_ONE_SIZE: u16 = 0x0800;
const RAM_MIRROR_ONE_END: u16 = 0x0fff;

const RAM_MIRROR_TWO_BEG: u16 = 0x1000;
const RAM_MIRROR_TWO_SIZE: u16 = 0x0800;
const RAM_MIRROR_TWO_END: u16 = 0x17ff;

const RAM_MIRROR_THREE_BEG: u16 = 0x1800;
const RAM_MIRROR_THREE_SIZE: u16 = 0x0800;
const RAM_MIRROR_THREE_END: u16 = 0x1fff;

const PPU_REGS_BEG: u16 = 0x2000;
const PPU_REGS_SIZE: u16 = 0x0008;
const PPU_REGS_END: u16 = 0x2007;

const PPU_MIRRORS_BEG: u16 = 0x2008;
const PPU_MIRRORS_SIZE: u16 = 0x1ff8;
const PPU_MIRRORS_END: u16 = 0x3fff;

const APU_REGS_BEG: u16 = 0x4000;
const APU_REGS_SIZE: u16 = 0x0020;
const APU_REGS_END: u16 = 0x401f;

const CARTRIDGE_SPACE_BEG: u16 = 0x4020;
const CARTRIDGE_SPACE_SIZE: u16 = 0xBFE0;
const CARTRIDGE_SPACE_END: u16 = 0xffff;

pub struct Memory {
    ram: Box<[u8]>,

    cart_rom: Box<[u8]>,
}

impl Memory {
    pub fn new(cart_rom: &Vec<u8>) -> Memory {
        Memory {
            ram: vec![0u8; RAM_SIZE].into_boxed_slice(),

            // once the size of the cartridge is known, it shouldn't change
            cart_rom: vec![0u8; cart_rom.len()].into_boxed_slice(),
        }
    }

    pub fn read_word(address: PhysAddr) -> u8 {
    }

    pub fn write_word() {}
}

fn map_addr(virt_addr: u16) -> PhysAddr {
    match virt_addr{
        _ => panic!("Unrecognized Physical Address {:#x}", virt_addr),
    }
}

pub enum PhysAddr {
}
