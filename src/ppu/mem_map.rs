const PATTERN_TABLE_0_BEG: u16 = 0x0000;
const PATTERN_TABLE_0_END: u16 = 0x0fff;

const PATTERN_TABLE_1_BEG: u16 = 0x1000;
const PATTERN_TABLE_1_END: u16 = 0x1fff;

const NAMETABLE_0_BEG: u16 = 0x2000;
const NAMETABLE_0_END: u16 = 0x23ff;

const NAMETABLE_1_BEG: u16 = 0x2400;
const NAMETABLE_1_END: u16 = 0x27ff;

const NAMETABLE_2_BEG: u16 = 0x2800;
const NAMETABLE_2_END: u16 = 0x2bff;

const NAMETABLE_3_BEG: u16 = 0x2c00;
const NAMETABLE_3_END: u16 = 0x2fff;

const NAMETABLE_MIRRORS_BEG: u16 = 0x3000;
const NAMETABLE_MIRRORS_END: u16 = 0x3eff;

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
    use PhysAddr;
    match addr {
        PATTERN_TABLE_0_BEG...PATTERN_TABLE_0_END => PatternTable_0(addr),
        PATTERN_TABLE_1_BEG...PATTERN_TABLE_1_END => PatternTable_1(addr),
        NAMETABLE_0_BEG...NAMETABLE_0_END => NameTable_0(addr),
        NAMETABLE_1_BEG...NAMETABLE_1_END => NameTable_1(addr),
        NAMETABLE_2_BEG...NAMETABLE_2_END => NameTable_2(addr),
        NAMETABLE_3_BEG...NAMETABLE_3_END => NameTable_3(addr),
        PALETTE_RAM_BEG...PALETTE_RAM_END => InternalPalette(addr),
        VRAM_MIRROR_BEG...VRAM_MIRROR_END => map_virt_addr(addr - 0x4000),
        _ => panic!("Read from this location not implemented: {:#X}", addr),
    }
}
