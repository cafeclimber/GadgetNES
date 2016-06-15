#[derive(Default, Debug)]
pub struct CpuCore {
    pub a: u8, // Accumulator

    pub x: u8, // x-Index
    pub y: u8, // y-index

    pub pc: u16, // Program counter
    
    pub s: u8, // Stack pointer

    pub p: u8, // Status register

}
