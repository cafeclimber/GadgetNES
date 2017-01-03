//! A module for interfacing with iNES ROMS
#![allow(dead_code)]

use std::fs::File;
use std::io::Read;
use std::path::Path;

use nes::ppu::Mirroring;

/// Public interface for an iNES format rom
pub struct InesRom {
    pub mapper_number: u8,
    pub prg_rom: Vec<u8>,
    pub chr: Vec<u8>,

    pub header: Header,
}

pub struct Header {
    pub magic_no: [u8; 4],
    prg_size: u8,
    chr_size: u8,
    flags_six: u8,
    flags_sev: u8,
    flags_nin: u8,
    flags_ten: u8,
}

impl Header {
    pub fn new(file_buf: &[u8]) -> Header{
        Header {
            magic_no:  [file_buf[0], file_buf[1], file_buf[2], file_buf[3]],
            prg_size:  file_buf[4],
            chr_size:  file_buf[5],
            flags_six: file_buf[6],
            flags_sev: file_buf[7],
            flags_nin: file_buf[9],
            flags_ten: file_buf[10],
        }
    }

    pub fn mapper_no(&self) -> u8 {
        (self.flags_six & 0b1111_0000) | (self.flags_sev & 0b1111_0000 >> 4)
    }

    pub fn contains_trainer(&self) -> bool {
        self.flags_six & (1 << 2) != 0
    }
}

impl InesRom {
    pub fn new<P: AsRef<Path>>(path:P) -> InesRom {
        let mut file = File::open(path).unwrap(); 
        let mut file_buf = Vec::new();
        file.read_to_end(&mut file_buf).expect("Error occured while reading ROM");

        let header = Header::new(&file_buf[0..15]);
        assert!(header.magic_no == *b"NES\x1a",
                   "ERROR: File is not of iNES format");

        let prg_size: u32 =  header.prg_size as u32 * 0x4000;
        let chr_size: u32 =  header.chr_size as u32 * 0x2000;

        InesRom {
            mapper_number: header.mapper_no(),
            prg_rom: {
                // Does rom contain 512 byte trainer before prg-rom
                if  header.contains_trainer() { 
                    let offset: usize = 512 + 16; 
                    file_buf[offset..(offset + prg_size as usize)].to_owned()
                } else {
                    file_buf[16..(16 + prg_size) as usize].to_owned()
                }
            },
            chr: if header.chr_size == 0 {
                // Uses CHR-RAM
                vec![0]
            } else if header.contains_trainer() {
                // After header, trainer and prg
                let offset: usize = 512 + 16 + prg_size as usize;
                file_buf[offset..(offset + chr_size as usize)].to_owned()
            } else {
                // After trainer and prg
                let offset: usize = 16 + prg_size as usize;
                file_buf[offset..(offset + chr_size as usize)].to_owned()
            },

            header: header,
        }
    }

    pub fn mirroring(&self) -> Mirroring {
        if self.header.flags_six & 0b1000 != 0 { Mirroring::FourWay }
        else if self.header.flags_six & 0b1 != 0 { Mirroring::Horizontal }
        else { Mirroring::Vertical }
    }
}
