#[derive(Default, Debug)]
pub struct CpuCore {
    a: u8, // Accumulator

    x: u8, // x-Index
    y: u8, // y-index

    pc: u16, // Program counter
    
    s: u8, // Stack pointer

    p: u8, // Status register

}
