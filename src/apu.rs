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

impl Apu {
    pub fn read_reg(&self, addr: u16) -> u8 {
        match addr {
            0x4000 ... 0x4003 => {self.pulse_1[(addr - 0x4000) as usize]},
            0x4004 ... 0x4007 => {self.pulse_2[(addr - 0x4004) as usize]},
            0x4008 ... 0x400b => {self.triangle[(addr - 0x4008) as usize]},
            0x400c ... 0x400f => {self.noise[(addr - 0x400c) as usize]},
            0x4010 ... 0x4013 => {self.dmc[(addr - 0x4010) as usize]},
            0x4015            => {self.snd_chn},
            0x4017            => {self.frame_counter},
            _                 => panic!("Attempted access on nonexistent APU register: {:#x}", addr),
        }
    }
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
