//! This module provides an interface for the 6502 as used in the NES

/// The NES Picture Processing Unit or PPU
///
/// Contains 8 registers mapped by the CPU, and the OAMDMA which is actually
/// located on the CPU:
/// 
/// PPUCTRL:   $2000 
///     Contains a number of flags used in controlling the PPU
/// PPUMASK:   $2001
///     Controls rendering of sprites and backgrounds as well as color effects
/// PPUSTATUS: $2002
///     Contains information regarding the state of the PPU
/// OAMADDR:   $2003
///     The address of OAM to access
/// OAMDATA:   $2004
///     OAM data is written here
/// PPUSCROLL: $2005
///     Changes the scroll position
/// PPUADDR:   $2006
///     This is how the CPU interacts with PPU memory. CPU sets the address here
/// PPUDATA:   $2007
///     And reads/writes occur here
/// OAMDMA:    $4014
///     How large amounts of data are transferred quickly
#[derive(Default)]
pub struct Ppu {
    ppu_ctrl: u8,
    ppu_mask: u8,
    ppu_status: u8,
    oam_addr: u8,
    oam_data: u8,
    ppu_scroll: u8,
    ppu_addr: u8,
    ppu_data: u8,

    oam_dma: u8,
}

// TODO: Add power_up function
impl Ppu {
    pub fn new() -> Ppu {
        Ppu::default()
    } 
}
