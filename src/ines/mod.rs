//! A module for interfacing with iNES ROMS
#![allow(dead_code)]

use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Public interface for an iNES format rom
pub struct InesRom {
    pub header: Header,
    pub mapper_number: u8,
    pub trainer: Vec<u8>,
    pub prg_rom: Vec<u8>,
    pub chr: Vec<u8>,
}

impl InesRom {
    pub fn new<P: AsRef<Path>>(path:P) -> InesRom {
        let mut file = File::open(path).unwrap(); 
        let mut file_buf = Vec::new();
        file.read_to_end(&mut file_buf).expect("Error occured while reading ROM");

        let header = Header::new(&file_buf);

        InesRom {
            header: header,
            mapper_number: header.flags_six.mapper_lower()
                | header.flags_sev.mapper_upper(),
            trainer: if header.flags_six.contains_trainer() {
                vec![0; 512]
            } else {
                vec![0]
            },
            prg_rom: {
                if header.flags_six.contains_trainer() {
                    // Trainer is 512 bytes of 0s following 16 byte header
                    let offset: usize = 512 + 16; 
                    file_buf[offset..(offset + header.prg_size as usize)]
                        .to_owned()
                } else {
                    // Otherwise, it's immediately after the 16 byte header
                    file_buf[16..(16 + header.prg_size) as usize].to_owned()
                }
            },
            chr: if header.chr_size == 0 {
                // Uses CHR-RAM
                vec![0]
            } else if header.flags_six.contains_trainer() {
                // After header, trainer and prg
                let offset: usize = 512 + 16 + header.prg_size as usize;
                file_buf[offset..(offset + header.chr_size as usize)].to_owned()
            } else {
                // After trainer and prg
                let offset: usize = 16 + header.prg_size as usize;
                file_buf[offset..(offset + header.chr_size as usize)].to_owned()
            }
        }
    }
}

/// 16 Byte iNES Header
/// Flags 10 is unofficial and not used in this emulator
#[derive(Clone, Copy)]
pub struct Header {
    prg_size: u16, // Actual header is only an 8 bit value but this is cleaner
    chr_size: u16, // Same as above
    ram_size: u16, // Same as above
    flags_six: FlagsSix,
    flags_sev: FlagsSev,
    flags_nin: FlagsNin,
}

impl Header {
    /// Asserts the first 4 chars of the rom are NES and MS-DOS EOF
    /// Proceeds to construct a Header for use by INESFile Struct
    fn new(rom: &Vec<u8>) -> Header {
        let nes_const = &rom[0..4];
        assert!(nes_const == [78, 69, 83, 26],
                   "ERROR: File is not of iNES format");

        Header {
            prg_size: rom[4] as u16 * 0x4000,
            chr_size: rom[5] as u16 * 0x2000,
            ram_size: if rom[5] == 0 {0x2000} else {rom[5] as u16 * 0x2000},
            flags_six: FlagsSix(rom[6]),
            flags_sev: FlagsSev(rom[7]),
            flags_nin: FlagsNin(rom[9]),
        }
    }
}

/// iNES Header Flags 6
///
/// A bit too complex of a scheme to use the bitflags crate unfortunately
/// 
/// 76543210
/// ||||||||
/// ||||+||+- 0xx0: vertical arrangement/horizontal mirroring 
/// |||| ||   0xx1: horizontal arrangement/vertical mirroring 
/// |||| ||   1xxx: four-screen VRAM
/// |||| |+-- 1: Cartridge contains persistent memory($6000-7FFF) 
/// |||| +--- 1: 512-byte trainer at $7000-$71FF (stored before PRG data)
/// ++++----- Lower nybble of mapper number
#[derive(Clone, Copy)]
struct FlagsSix(u8);

pub enum Mirroring {
    Vertical,
    Horizontal,
    FourScreen,
}

impl FlagsSix {
    fn mirroring(&self) -> Mirroring {
        if self.0 & 0b1000 == 1 {
            Mirroring::FourScreen
        } else {
            if self.0 & 0b1 == 1 {
                Mirroring::Vertical
            } else {
                Mirroring::Horizontal
            }
        }
    }

    fn contains_persistent_mem(&self) -> bool {
        self.0 & 0b10000 != 0
    }

    fn contains_trainer(&self) -> bool {
        self.0 & 0b100 != 0
    }

    fn mapper_lower(&self) -> u8 {
        (&self.0 & 0b11110000) >> 4
    }
}

/// iNES Header Flags 7
///
/// 76543210
/// ||||||||
/// |||||||+- VS Unisystem
/// ||||||+-- PlayChoice-10 (8KB of Hint Screen data stored after CHR data)
/// ||||++--- If equal to 2, flags 8-15 are in NES 2.0 format
/// ++++----- Upper nybble of mapper number
#[derive(Clone, Copy)]
struct FlagsSev(u8);

impl FlagsSev {
    fn vs_unisystem(&self) -> bool {
        self.0 & 0b1 != 0
    }

    fn is_ines_two(&self) -> bool {
        self.0 & 0b1100 == 0b1100
    } 

    fn mapper_upper(&self) -> u8 {
        self.0 & 0b11110000
    }
}

/// iNES Header Flags 9
///
/// 76543210
/// ||||||||
/// |||||||+- TV system (0: NTSC; 1: PAL)
/// +++++++-- Reserved, set to zero
#[derive(Clone, Copy)]
struct FlagsNin(u8);

impl FlagsNin {}
