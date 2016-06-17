#[derive(Default, Debug)]
pub struct Apu {
    pub pulse_1: [u8; 4], // $4000 - $4003
    pub pulse_2: [u8; 4], // $4004 - $4007

    pub triangle: [u8; 4], // $4008 - $400b

    pub noise: [u8; 4], // $400c - $400f

    pub dmc: [u8; 4], // $4010 - $ 4013

    pub snd_chn: u8, // $4015

    pub frame_counter: u8, //$4017 also mapped by cpu
}

impl Apu {
}
