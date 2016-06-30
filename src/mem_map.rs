const CPU_RAM_BEG: u16 = 0x0000;
const CPU_RAM_END: u16 = 0x07ff;

const RAM_MIRROR_ONE_BEG: u16 = 0x0800;
const RAM_MIRROR_ONE_END: u16 = 0x0fff;
const RAM_MIRROR_TWO_BEG: u16 = 0x1000;
const RAM_MIRROR_TWO_END: u16 = 0x17ff;
const RAM_MIRROR_THREE_BEG: u16 = 0x1800;
const RAM_MIRROR_THREE_END: u16 = 0x1fff;

const PPU_REGS_BEG: u16 = 0x2000;
const PPU_REGS_END: u16 = 0x2007;

const PPU_MIRRORS_BEG: u16 = 0x2008;
const PPU_MIRRORS_END: u16 = 0x3fff;

const APU_REGS_BEG: u16 = 0x4000;
const APU_REGS_END: u16 = 0x401f;

const CART_SPACE_BEG: u16 = 0x4020;
const CART_SPACE_END: u16 = 0xffff;

#[derive(Debug)]
pub enum PhysAddr {
    CpuRam(u16),
    RamMirrorOne(u16),
    RamMirrorTwo(u16),
    RamMirrorThree(u16),
    PpuRegs(u16),
    PpuMirrors(u16),
    ApuRegs(u16),
    CartSpace(u16),
}

pub fn map_virt_addr(addr: u16) -> PhysAddr {
    match addr {
        CPU_RAM_BEG ... CPU_RAM_END => PhysAddr::CpuRam(addr),

        RAM_MIRROR_ONE_BEG ... RAM_MIRROR_ONE_END => PhysAddr::RamMirrorOne(addr),
        RAM_MIRROR_TWO_BEG ... RAM_MIRROR_TWO_END => PhysAddr::RamMirrorTwo(addr),
        RAM_MIRROR_THREE_BEG ... RAM_MIRROR_THREE_END => PhysAddr::RamMirrorThree(addr),

        PPU_REGS_BEG ... PPU_REGS_END => PhysAddr::PpuRegs(addr),
        PPU_MIRRORS_BEG ... PPU_MIRRORS_END => PhysAddr::PpuMirrors(addr),

        APU_REGS_BEG ... APU_REGS_END => PhysAddr::ApuRegs(addr),

        CART_SPACE_BEG ... CART_SPACE_END => PhysAddr::CartSpace(addr),

        _ => panic!("Unrecognized virtual address: {:#x}", addr),
    }
}
