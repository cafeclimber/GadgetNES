#[derive(Default, Debug)]
pub struct Apu {
    pulse_1: [u8; 4], // $4000 - $4003
    pulse_2: [u8; 4], // $4004 - $4007

    triangle: [u8; 4], // $4008 - $400b

    noise: [u8; 4], // $400c - $400f

    dmc: [u8; 4], // $4010 - $ 4013

    snd_chn: u8, // $4015

    frame_counter: u8, //$4017 also mapped by cpu
}
