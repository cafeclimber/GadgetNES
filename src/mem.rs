/*===== Memory Map ======*/
const RAM_BEG: u16 = 0x0000;
const RAM_SIZE: usize = 0x0800;
const RAM_END: u16 = 0x07ff;

const RAM_MIRROR_ONE_BEG: u16 = 0x0800;
const RAM_MIRROR_ONE_SIZE: u16 = 0x0800;
const RAM_MIRROR_ONE_END: u16 = 0x0fff;

const RAM_MIRROR_TWO_BEG: u16 = 0x1000;
const RAM_MIRROR_TWO_SIZE: u16 = 0x0800;
const RAM_MIRROR_TWO_END: u16 = 0x17ff;

const RAM_MIRROR_THREE_BEG: u16 = 0x1800;
const RAM_MIRROR_THREE_SIZE: u16 = 0x0800;
const RAM_MIRROR_THREE_END: u16 = 0x1fff;

const PPU_REGS_BEG: u16 = 0x2000;
const PPU_REGS_SIZE: u16 = 0x0008;
const PPU_REGS_END: u16 = 0x2007;

const PPU_MIRRORS_BEG: u16 = 0x2008;
const PPU_MIRRORS_SIZE: u16 = 0x1ff8;
const PPU_MIRRORS_END: u16 = 0x3fff;

const APU_REGS_BEG: u16 = 0x4000;
const APU_REGS_SIZE: u16 = 0x0020;
const APU_REGS_END: u16 = 0x401f;

const CARTRIDGE_SPACE_BEG: u16 = 0x4020;
const CARTRIDGE_SPACE_SIZE: u16 = 0xBFE0;
const CARTRIDGE_SPACE_END: u16 = 0xffff;

const PRG_RAM_SIZE: usize = 0x2000;
const PRG_ROM_DEFAULT: usize = 0x8000;
const TODO_CHR_MEM: usize = 0x4000;

#[derive(Default)]
pub struct Memory {
    cpu_ram: Box<[u8]>,

    prg_ram: Box<[u8]>,

    prg_rom: Box<[u8]>,

    chr_mem: Box<[u8]>,
}

pub enum AddressingMode {
    Absolute(u16),
    Immediate(u16),
    ZeroPage(u16),
    Relative(u16),
    AbsX(u16, u8),
    AbsY(u16, u8),
    ZPageX(u16, u8),
    ZPageY(u16, u8),
    IndexedIndirect(u16, u8),
    IndirectIndexed(u16, u8),
}

impl Memory {
    // TODO Implement chr_rom, prg_ram, and prg_rom
    // TODO make this function yield chr_rom and prg_ram as well
    pub fn load_cartridge(&mut self, cart_rom: Vec<u8>) {
        self.cpu_ram = vec![0u8; RAM_SIZE].into_boxed_slice();
        self.prg_ram = vec![0u8; PRG_RAM_SIZE].into_boxed_slice();
        self.prg_rom = cart_rom.into_boxed_slice();
        self.chr_mem = vec![0u8; TODO_CHR_MEM].into_boxed_slice();
    }


    fn map_mem(&self, addr: u16) -> u8 {
        match addr {
            RAM_BEG ... RAM_END => {self.cpu_ram[addr as usize]},
            // TODO: Deal with mappers and other memory spaces
            CARTRIDGE_SPACE_BEG ... CARTRIDGE_SPACE_END => {self.prg_rom[addr as usize]},
            _ => panic!("Unrecognized virtual address: {:#x}", addr)
        }
    }

    pub fn read_instr(&self, addr: u16) -> u8 {
        self.prg_rom[addr as usize]
    }
    // For fetching rest of instructioons
    fn rom_byte(&self, addr: u16) -> u8 {
        self.prg_rom[addr as usize]
    }

    fn rom_word(&self, addr: u16) -> u16 {
        (self.prg_rom[(addr+1) as usize] as u16) << 8 |
        self.prg_rom[addr as usize] as u16
    }

    // TODO: Double check these are correct
    pub fn read_mem(&self, AddressingMode: AddressingMode) -> u8{
        use self::AddressingMode::*;
        match AddressingMode {
            Absolute(pc)      => {self.map_mem(self.rom_word(pc+1))},
            Immediate(pc)     => {self.rom_byte(pc+1)}
            ZeroPage(pc)      => {self.map_mem((0x0000 | self.rom_byte(pc+1) as u16))},
            Relative(pc)      => {self.map_mem((pc + self.rom_byte(pc+1) as u16))},
            AbsX(pc, cpu_x)   => {self.map_mem((cpu_x as u16 + self.rom_word(pc+1)))},
            AbsY(pc, cpu_y)   => {self.map_mem((cpu_y as u16 + self.rom_word(pc+1)))},
            ZPageX(pc, cpu_x) => {self.map_mem(((cpu_x + self.rom_byte(pc+1)) as u16))},
            ZPageY(pc, cpu_y) => {self.map_mem(((cpu_y as u16) + (self.rom_byte(pc+1)) as u16))},
            IndexedIndirect(pc, cpu_x) => {self.map_mem((self.rom_word((self.rom_byte(pc+1) + cpu_x) as u16)))},
            IndirectIndexed(pc, cpu_y) => {self.map_mem((self.rom_word(((self.rom_byte((pc+1) as u16)) + cpu_y) as u16)))},
        }
    }
}
