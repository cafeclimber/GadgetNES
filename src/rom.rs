//! Provides an abstraction for iNES Rom format

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn read_rom(path: &Path) -> Result<Rom, &str> {
    let file = File::open(path);
    match file {
        Err(_) => {
            return Err("Failed to open file");
        }
        _ => {}
    }
    let mut file_buf = Vec::new();
    // We know file is good at this point
    match file.unwrap().read_to_end(&mut file_buf) {
        Err(_) => {
            return Err("Failed to read from file");
        }
        _ => {}
    }
    let mut header_buf = vec![0; 16];
    header_buf.copy_from_slice(&file_buf[0..16]);
    let header = Header::new(header_buf)?;

    let mut prg_rom = vec![0; header.prg_size()];
    let offset = calc_prg_offset(&header);
    prg_rom.copy_from_slice(&file_buf[offset..header.prg_size() + offset]);

    let mut chr_rom = vec![0; header.chr_size()];

    if header.chr_size() != 0 {
        let offset = header.prg_size() + offset;
        chr_rom.copy_from_slice(&file_buf[offset..offset + header.chr_size()]);
    }
    Ok(Rom {
        mapper_number: header.mapper_no(),
        prg_rom: prg_rom,
        chr_rom: chr_rom,
        header: header,
    })
}

fn calc_prg_offset(header: &Header) -> usize {
    let mut offset = 16; // Always at least the header
    offset += if header.flags_six.trainer_present() {
        512
    } else {
        0
    };

    offset
}

#[derive(Debug)]
pub struct Rom {
    pub mapper_number: u8,
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>, // This is not always present

    pub header: Header,
}

#[derive(Debug)]
pub struct Header {
    mapper_no: u8,
    prg_size: usize,
    chr_size: usize,
    ram_size: usize,
    flags_six: FlagsSix,
    flags_sev: FlagsSev,
}

#[derive(Debug)]
pub struct FlagsSix(u8);

#[derive(Debug)]
pub struct FlagsSev(u8);

impl Header {
    pub fn new(header: Vec<u8>) -> Result<Header, &'static str> {
        if header.get(0..4).unwrap() != &[0x4E, 0x45, 0x53, 0x1A] {
            return Err("File is not a valid iNES ROM");
        }

        let prg_ram_size = if header[8] == 0 {
            8192
        } else {
            header[8] as usize * 8192
        };
        let flags_six = FlagsSix(header[6]);
        let flags_sev = FlagsSev(header[7]);

        Ok(Header {
            mapper_no: flags_sev.mapper_no_upper_nibble() | flags_six.mapper_no_lower_nibble(),
            prg_size: header[4] as usize * 16384,
            chr_size: header[5] as usize * 8192,
            ram_size: prg_ram_size,
            flags_six: flags_six,
            flags_sev: flags_sev,
        })
    }

    pub fn mapper_no(&self) -> u8 {
        self.mapper_no
    }

    pub fn prg_size(&self) -> usize {
        self.prg_size
    }

    pub fn chr_size(&self) -> usize {
        self.chr_size
    }

    pub fn ram_size(&self) -> usize {
        self.ram_size
    }
}

impl FlagsSix {
    // TODO: pub fn mirroring() -> Mirroring (FROM PPU)
    pub fn contains_batt_backed_ram(&self) -> bool {
        self.0 & (1 << 1) == 1
    }

    pub fn trainer_present(&self) -> bool {
        self.0 & (1 << 2) == 1
    }

    pub fn ignore_mirroring(&self) -> bool {
        self.0 & (1 << 3) == 1
    }

    pub fn mapper_no_lower_nibble(&self) -> u8 {
        (self.0 & ((1 << 7) | (1 << 6) | (1 << 5) | (1 << 4))) >> 4
    }
}

impl FlagsSev {
    pub fn vs_unisystem(&self) -> bool {
        self.0 & (1 << 0) == 1
    }

    pub fn playchoice_ten(&self) -> bool {
        self.0 & (1 << 1) == 1
    }

    pub fn ines_two_fmt(&self) -> bool {
        self.0 & (1 << 2) == 1
    }

    pub fn mapper_no_upper_nibble(&self) -> u8 {
        self.0 & ((1 << 7) | (1 << 6) | (1 << 5) | (1 << 4))
    }
}
