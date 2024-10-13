//! When no banking is required and no MBC Chip is present on the ROM. ROM and RAM
//! access get directly mapped to memory.

use super::MBC;

pub(super) const ID: u8 = 0x00;

/// ROM Memory Size 
const ROM_MEMORY_SIZE: usize = 0x8000;

/// External RAM offset in WRAM.
const RAM_OFFSET: usize = 0xA000;
/// External RAM size.
const RAM_SIZE: usize = 0x2000;

/// MBC0 doesn't exist and mimics the behaviour when no MBC 
/// is present on the rom.
pub(super) struct MBC0 {
    rom: [u8; ROM_MEMORY_SIZE],
    ram: [u8; RAM_SIZE],
}

impl MBC0 {
    pub fn new(rom: &[u8]) -> Self {
        assert!(rom.len() <= ROM_MEMORY_SIZE);
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

impl MBC for MBC0 {
    fn read_rom(&self, address: u16) -> u8 {
        self.rom[address as usize]
    }

    fn write_rom(&mut self, _: u16, _: u8) { }

    fn read_ram(&self, address: u16) -> u8 {
        self.ram[(address as usize) - RAM_OFFSET]
    }

    fn write_ram(&mut self, address: u16, val: u8) {
        self.ram[(address as usize) - RAM_OFFSET] = val;
    }
}
