#![allow(dead_code)]
// For overall map
pub const CPU_RAM_BEG: u16 = 0x0000;
pub const CPU_RAM_END: u16 = 0x07FF;
pub const RAM_MIRRORS_BEG: u16 = 0x0800;
pub const RAM_MIRRORS_END: u16 = 0x1FFF;
pub const PPU_REGS_BEG: u16 = 0x2000;
pub const PPU_REGS_END: u16 = 0x2007;
pub const PPU_MIRRORS_BEG: u16 = 0x2008;
pub const PPU_MIRRORS_END: u16 = 0x3FFF;
pub const IO_REGS_BEG: u16 = 0x4000;
pub const IO_REGS_END: u16 = 0x4017;
pub const CART_SPACE_BEG: u16 = 0x4020;
pub const CART_SPACE_END: u16 = 0xFFFF;
