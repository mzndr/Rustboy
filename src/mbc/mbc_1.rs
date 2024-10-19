//! MBC1 implementation.
//! TODO: Support alternative wiring.

use core::panic;

use super::MBC;

pub(super) const ID: u8 = 0x01;

const ROM_BANK_SIZE: usize = 0x4000;
const RAM_BANK_SIZE: usize = 0x4000;

const RAM_OFFSET: usize = 0xA000;
const RAM_SIZE: usize = 0x2000;

type RomBank = [u8; ROM_BANK_SIZE];

type RamBank = [u8; RAM_BANK_SIZE];

pub(super) struct MBC1 {
    rom_bank_00: RomBank,
    rom_banks: Vec<RomBank>,
    rom_bank_idx: usize,

    ram_enable: bool,
    ram_banks: Vec<RamBank>,
    ram_bank_idx: usize,

    alternative_wiring: bool,
    banking_mode: bool,
}

impl MBC1 {
    pub fn new(rom: &[u8], rom_size: u8, ram_size: u8) -> Self {
        let mut rom_bank_00 = [0x00; ROM_BANK_SIZE];
        let mut rom_banks = vec![[0x00; ROM_BANK_SIZE]; ROM_BANK_NUM];

        for (address, byte) in rom.iter().enumerate() {
            match address {
                0x0000..=0x3FFF => rom_bank_00[address] = *byte,
                0x4000..=0x7FFF => rom_banks[address / 0x4000][address - 0x4000] = *byte,
                _ => panic!("cannot load rom to mbc1"),
            };
        }

        Self {
            rom_bank_00,
            rom_banks,
            ram_banks: vec![[0x00; RAM_BANK_SIZE]; RAM_BANK_NUM],

            banking_mode: false,

            ram_enable: false,
            ram_bank_idx: 0,
            rom_bank_idx: 1,
        }
    }

    fn get_rom_bank_idx(&self) -> usize {
        if self.banking_mode {
            return 0;
        }

        self.rom_bank_idx
    }

    fn get_ram_bank_idx(&self) -> usize {
        if self.banking_mode {
            return 0;
        }

        self.ram_bank_idx
    }
}

impl MBC for MBC1 {
    fn read_rom(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => self.rom_bank_00[address as usize],
            0x4000..=0x7FFF => self.rom_banks[self.get_rom_bank_idx()][address as usize - 0x4000],
            _ => panic!("invalid mbc1 rom read at 0x{address:x}"),
        }
    }

    fn write_rom(&mut self, address: u16, val: u8) {
        match address {
            // RAM enable
            0x0000..=0x1FFF => self.ram_enable = 0x0F == 0x0A,
            // ROM bank Select
            0x2000..=0x3FFF => {
                let masked = val & 0x1F;
                let idx = if masked == 0 { 1 } else { masked };
                self.rom_bank_idx = idx as usize;
            }
            // RAM bank Select
            0x4000..=0x5FFF => {
                let idx = val & 0x03;
                self.ram_bank_idx = idx as usize;
            }
            // Banking mode select
            0x6000..=0x7FFF => self.banking_mode = (val & 0x01) == 0x01,
            // Shouldn't happen.
            _ => panic!("invalid mbc1 rom write at 0x{address:x}"),
        }
    }

    fn read_ram(&self, address: u16) -> u8 {
        self.ram_banks[self.get_ram_bank_idx()][address as usize - RAM_OFFSET]
    }

    fn write_ram(&mut self, address: u16, val: u8) {
        let idx = self.get_ram_bank_idx();
        self.ram_banks[idx][address as usize - RAM_OFFSET] = val;
    }
}
