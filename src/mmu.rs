//! Memory management unit. Handle and map memory access.

use crate::{cpu::utils, ppu::Ppu};

/// Gameboy wram size.
const WRAM_SIZE: usize = 0x10000;

#[derive(Debug, Clone)]
pub struct Mmu {
    wram: [u8; WRAM_SIZE],
    ppu: Ppu,
}

impl Mmu {
    /// Create new wram.
    pub fn new() -> Self {
        Self {
            wram: [0x00; WRAM_SIZE],
            ppu: Ppu::new(),
        }
    }

    /// Needs to be changed for bigger games, since they
    /// are too big to fit into ram, so banking has to be
    /// implemented.
    pub fn load_rom(&mut self, rom: &[u8]) {
        for (address, byte) in rom.iter().enumerate() {
            self.write_u8(address as u16, *byte);
        }
    }

    /// Checks if an address is in valid space,
    /// prints an error message and quits if not.
    fn check_address(address: u16) {
        if address as usize > WRAM_SIZE + 1 {
            tracing::error!("bad wram access at 0x{:x}", &address);
            panic!("bad wram access at 0x{:x}", &address)
        }
    }

    /// Reads from wram at address.
    pub fn read(&self, address: u16) -> u8 {
        let u_addr = address as usize;
        Self::check_address(address);
        self.wram[u_addr]
    }

    /// Writes u8 to wram at address.
    pub fn write_u8(&mut self, address: u16, val: u8) {
        let u_addr = address as usize;
        Self::check_address(address);
        self.wram[u_addr] = val;
    }

    /// Writes u16 to wram at address.
    pub fn write_u16(&mut self, address: u16, val: u16) {
        let split = utils::split_u16(val);
        self.write_u8(address, split.1);
        self.write_u8(address + 1, split.0);
    }
}
