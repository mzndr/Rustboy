//! Default MBC when no banking is required and no MBC Chip is present on the ROM. ROM and RAM
//! access get directly mapped to memory.

use super::MBC;

pub(super) const TYPE: u8 = 0x00;

const ROM_MEMORY_SIZE: usize = 0x8000;

const RAM_OFFSET: usize = 0xA000;
const RAM_SIZE: usize = 0x2000;

pub(super) struct Default {
    rom: [u8; ROM_MEMORY_SIZE],
    ram: [u8; RAM_SIZE],
}

impl Default {
    pub fn new(rom: &[u8]) -> Self {
        assert!(rom.len() > ROM_MEMORY_SIZE);
        let mut rom_mem = [0x00; ROM_MEMORY_SIZE];
        for (addr, byte) in rom.iter().enumerate() {
            rom_mem[addr] = *byte;
        }
        Self {
            rom: rom_mem,
            ram: [0x00; RAM_SIZE],
        }
    }
}

impl MBC for Default {
    fn read_rom(&self, address: u16) -> u8 {
        self.rom[address as usize]
    }

    fn write_rom(&mut self, address: u16, val: u8) {
        self.rom[address as usize] = val;
    }

    fn read_ram(&self, address: u16) -> u8 {
        self.ram[(address as usize) - RAM_OFFSET]
    }

    fn write_ram(&mut self, address: u16, val: u8) {
        self.ram[(address as usize) - RAM_OFFSET] = val;
    }
}
