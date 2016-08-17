const PATTERN_TABLES_BEG: u16 = 0x0000;
const PATTERN_TABLES_END: u16 = 0x1fff;

const NAMETABLES_BEG: u16 = 0x2000;
const NAMETABLES_END: u16 = 0x2fff;

const NAMETABLE_MIRRORS_BEG: u16 = 0x3000;
const NAMETABLE_MIRRORS_END: u16 = 0x3eff;

const PALETTE_RAM_BEG: u16 = 0x3f00;
const PALETTE_RAM_END: u16 = 0x3f1F;
const PALETTE_MIRROR_BEG: u16 = 0x3f20;
const PALETTE_MIRROR_END: u16 = 0x3fff;

const VRAM_MIRROR_BEG: u16 = 0x4000;
const VRAM_MIRROR_END: u16 = 0xffff;

pub enum PhysAddr {
    PatternTable(u16),
    NameTable(u16),
    InternalPalette(u16),
}

pub fn map_virt_addr(addr: u16) -> PhysAddr {
    match addr {
        PATTERN_TABLES_BEG...PATTERN_TABLES_END => PhysAddr::PatternTable(addr),
        NAMETABLES_BEG...NAMETABLES_END => PhysAddr::NameTable(addr),
        PALETTE_RAM_BEG...PALETTE_RAM_END => PhysAddr::InternalPalette(addr),
        VRAM_MIRROR_BEG...VRAM_MIRROR_END => map_virt_addr(addr - 0x4000),
        _ => panic!("Read from this location not implemented: {:#X}", addr),
    }
}
