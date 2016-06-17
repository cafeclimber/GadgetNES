use std::fmt;

#[derive(Default)]
pub struct Apu {
    pub pulse_1: [u8; 4], // $4000 - $4003
    pub pulse_2: [u8; 4], // $4004 - $4007

    pub triangle: [u8; 4], // $4008 - $400b

    pub noise: [u8; 4], // $400c - $400f

    pub dmc: [u8; 4], // $4010 - $ 4013

    pub snd_chn: u8, // $4015

    pub frame_counter: u8, //$4017 also mapped by cpu
}

impl fmt::Debug for Apu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "APU: pulse_1: 0x{:x}, 0x{:x}, 0x{:x}, 0x{:x}
APU: pulse_2: 0x{:x}, 0x{:x}, 0x{:x}, 0x{:x}
APU: triangle: 0x{:x}, 0x{:x}, 0x{:x}, 0x{:x}
APU: noise: 0x{:x}, 0x{:x}, 0x{:x}, 0x{:x}
APU: dmc: 0x{:x}, 0x{:x}, 0x{:x}, 0x{:x}
APU: snd_chn: 0x{:x}
APU: frame_counter: 0x{:x}",
               self.pulse_1[0],self.pulse_1[1],self.pulse_1[2],self.pulse_1[3],
               self.pulse_2[0],self.pulse_2[1],self.pulse_2[2],self.pulse_2[3],
               self.triangle[0],self.triangle[1],self.triangle[2],self.triangle[3],
               self.noise[0],self.noise[1],self.noise[2],self.noise[3],
               self.dmc[0],self.dmc[1],self.dmc[2],self.dmc[3],
               self.snd_chn,
               self.frame_counter)
    }
}
