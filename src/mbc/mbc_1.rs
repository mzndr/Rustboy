//! MBC1 implementation.
//! TODO: Support alternative wiring.

use super::MBC;
use core::panic;

pub(super) const ID: u8 = 0x01;

const ROM_BANK_SIZE: usize = 0x4000;
const RAM_BANK_SIZE: usize = 0x4000;

const RAM_OFFSET: usize = 0xA000;

pub(super) struct MBC1 {
    rom: Vec<u8>,
    rom_bank_idx: usize,

    ram_enable: bool,
    ram: Vec<u8>,
    ram_bank_idx: usize,

    alternative_wiring: bool,
    banking_mode: bool,
}

impl MBC1 {
    pub fn new(rom: &[u8]) -> Self {
        Self {
            rom: Vec::from(rom),
            ram: vec![0x00; RAM_BANK_SIZE * 3],

            banking_mode: false,
            alternative_wiring: rom.len() >= usize::pow(2, 20),

            ram_enable: false,
            ram_bank_idx: 0,
            rom_bank_idx: 0,
        }
    }

    fn get_rom_bank_idx(&self) -> usize {
        if self.banking_mode {
            return 0;
        }
        if self.rom_bank_idx == 0 {
            return 1;
        }
        self.rom_bank_idx
    }

    fn get_ram_bank_idx(&self) -> usize {
        if self.banking_mode || self.alternative_wiring {
            return 0;
        }
        self.ram_bank_idx
    }
}

impl MBC for MBC1 {
    fn read_rom(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => self.rom[address as usize],
            0x4000..=0x7FFF => {
                let rom_bank_offset = ROM_BANK_SIZE * self.get_rom_bank_idx();
                let bank_address = address as usize - ROM_BANK_SIZE;
                let mapped_address = rom_bank_offset + bank_address;
                self.rom[mapped_address]
            }
            _ => panic!("invalid mbc1 rom read at 0x{address:x}"),
        }
    }

    fn write_rom(&mut self, address: u16, val: u8) {
        match address {
            // RAM enable
            0x0000..=0x1FFF => {
                self.ram_enable = val & 0x0F == 0x0A;
                tracing::debug!("ram enable: {}", self.ram_enable);
            }
            // ROM bank Select
            0x2000..=0x3FFF => {
                let mask = 0b11;
                let masked = val & mask;
                tracing::debug!("rom bank {masked} selected");
                self.rom_bank_idx = masked as usize;
            }
            // RAM bank Select, might lend itself to ROM bank select in alternative wiring.
            0x4000..=0x5FFF => {
                let idx = val & 0b11;
                self.ram_bank_idx = idx as usize;
                tracing::debug!("ram bank {idx} selected");
            }
            // Banking mode select
            0x6000..=0x7FFF => {
                self.banking_mode = (val & 0x01) == 0x01;
                tracing::debug!("banking mode: {}", self.banking_mode);
            }
            // Shouldn't happen.
            _ => panic!("invalid mbc1 rom write at 0x{address:x}"),
        }
    }

    fn read_ram(&self, address: u16) -> u8 {
        if !self.ram_enable {
            return 0xFF;
        }
        let ram_bank_offset = self.get_rom_bank_idx() * RAM_BANK_SIZE;
        let bank_address = address as usize - RAM_OFFSET;
        let ram_address = ram_bank_offset + bank_address;
        self.ram[ram_address]
    }

    fn write_ram(&mut self, address: u16, val: u8) {
        if !self.ram_enable {
            return;
        }
        let ram_bank_offset = self.get_rom_bank_idx() * RAM_BANK_SIZE;
        let bank_address = address as usize - RAM_OFFSET;
        let ram_address = ram_bank_offset + bank_address;
        self.ram[ram_address] = val;
    }
}
