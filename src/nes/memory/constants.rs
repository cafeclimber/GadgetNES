// For RAM
pub const ZERO_PAGE_BEG: u16 = 0x0000;
pub const ZERO_PAGE_END: u16 = 0x00FF;
pub const ZERO_PAGE_SIZE: usize = 0x0100;
pub const STACK_BEG: u16 = 0x0100;
pub const STACK_END: u16 = 0x01FF;
pub const STACK_SIZE: usize = 0x0100;
pub const RAM_BEG: u16 = 0x0200;
pub const RAM_END: u16 = 0x07FF;
pub const RAM_SIZE: usize = 0x600;
