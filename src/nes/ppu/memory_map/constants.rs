// Pattern tables
pub const PAT_TABLE_0_BEG: u16 = 0x0000;
pub const PAT_TABLE_0_END: u16 = 0x0FFF;
pub const PAT_TABLE_1_BEG: u16 = 0x1000;
pub const PAT_TABLE_1_END: u16 = 0x1FFF;

// Nametables
pub const NAMETABLE_0_BEG: u16 = 0x2000;
pub const NAMETABLE_0_END: u16 = 0x23FF;

pub const NAMETABLE_1_BEG: u16 = 0x2400;
pub const NAMETABLE_1_END: u16 = 0x27FF;

pub const NAMETABLE_2_BEG: u16 = 0x2800;
pub const NAMETABLE_2_END: u16 = 0x2BFF;

pub const NAMETABLE_3_BEG: u16 = 0x2C00;
pub const NAMETABLE_3_END: u16 = 0x2FFF;

// Mirrors (only mirrors 0x2000..0x2EFF because...reasons)
pub const NAMETABLE_MIRRORS_BEG: u16 = 0x3000;
pub const NAMETABLE_MIRRORS_END: u16 = 0x3EFF;

// Palette RAM and mirrors
pub const PALETTE_RAM_BEG: u16 = 0x3F00;
pub const PALETTE_RAM_END: u16 = 0x3F1F;

pub const PALETTE_MIRRORS_BEG: u16 = 0x3F20;
pub const PALETTE_MIRRORS_END: u16 = 0x3FFF;
