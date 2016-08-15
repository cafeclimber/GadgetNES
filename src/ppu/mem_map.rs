const PALETTE_RAM_BEG: u16 = 0x3f00;
const PALETTE_RAM_END: u16 = 0x3f1F;
const PALETTE_MIRROR_BEG: u16 = 0x3f20;
const PALETTE_MIRROR_END: u16 = 0x3fff;

const VRAM_MIRROR_BEG: u16 = 0x4000;
const VRAM_MIRROR_END: u16 = 0xffff;

enum PhysAddr {
    PatternTable_0(u16),
    PatternTable_1(u16),
    NameTable_0(u16),
    NameTable_1(u16),
    NameTable_2(u16),
    NameTable_3(u16),
    InternalPalette(u16),
}

pub fn map_virt_addr(addr: u16) -> PhysAddr {
    use self::PhysAddr;
    match addr {
        PALETTE_RAM_BEG...PALETTE_RAM_END => InternalPalette(addr),
        VRAM_MIRROR_BEG...VRAM_MIRROR_END => map_virt_addr(addr - 0x4000),
    }
    _ => panic!("Read from this location not implemented: {:#X}", addr);
}
