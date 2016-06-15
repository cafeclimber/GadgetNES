mod apu;
mod cpu;

#[derive(Default, Debug)]
pub struct Cpu {
    cpu_core: cpu::CpuCore,
    apu: apu::Apu, 

    oamdma: u8, // $4014

    joy1: u8, // $4016
    joy2: u8, // $4017 also mapped by APU
}

impl Cpu {
    pub fn power_up(&mut self) {
        self.cpu_core.p = 0x34;
        self.cpu_core.s = 0xfd;

        self.apu.snd_chn = 0x00 ; // all channels disabled
        self.joy2 = 0x00; // set frame_irq to enable
    }
}
