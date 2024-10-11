//! Memory management unit. Handle and map memory access.
//!
//! Memory map (see <https://gbdev.io/pandocs/Memory_Map.html>):
//! Start   End     Description                        Notes
//! 0000    3FFF    16 KiB ROM bank 00                 From cartridge, usually a fixed bank
//! 4000    7FFF    16 KiB ROM Bank 01–NN              From cartridge, switchable bank via mapper (if any)
//! 8000    9FFF    8 KiB Video RAM (VRAM)             In CGB mode, switchable bank 0/1
//! A000    BFFF    8 KiB External RAM                 From cartridge, switchable bank if any
//! C000    CFFF    4 KiB Work RAM (WRAM)
//! D000    DFFF    4 KiB Work RAM (WRAM)              In CGB mode, switchable bank 1–7
//! E000    FDFF    Echo RAM (mirror of C000–DDFF)     Nintendo says use of this area is prohibited.
//! FE00    FE9F    Object attribute memory (OAM)
//! FEA0    FEFF    Not Usable    =                    Nintendo says use of this area is prohibited.
//! FF00    FF7F    I/O Registers
//! FF80    FFFE    High RAM (HRAM)
//! FFFF    FFFF    Interrupt Enable register (IE)

use crate::{
    apu::Apu,
    cpu::{self, utils, Cpu},
    ppu::Ppu,
};

/// Gameboy wram size.
const WRAM_SIZE: usize = 0x10000;

/// Offset to handle echo ram redirection.
const WRAM_ECHO_OFFSET: u16 = 0x2000;

/// Memory management unit. Handle and map memory access.
#[derive(Debug, Clone)]
pub struct Mmu {
    wram: [u8; WRAM_SIZE],
    ppu: Ppu,
    apu: Apu,
}

impl Mmu {
    /// Create new wram.
    pub fn new() -> Self {
        Self {
            wram: [0x00; WRAM_SIZE],
            ppu: Ppu::new(),
            apu: Apu::new(),
        }
    }

    pub fn cycle(&mut self) {
        self.ppu.cycle();
    }

    pub fn ppu_ref<'a>(&'a self) -> &'a Ppu {
        &self.ppu
    }

    /// Needs to be changed for bigger games, since they
    /// are too big to fit into ram, so banking has to be
    /// implemented.
    pub fn load_rom(&mut self, rom: &[u8]) {
        for (address, byte) in rom.iter().enumerate() {
            self.write_u8(address as u16, *byte);
        }
    }

    /// Reads from wram at address.
    pub fn read(&self, address: u16) -> u8 {
        let u_addr = address as usize;

        match u_addr {
            0x0000..=0x7FFF => self.wram[u_addr],
            0x8000..=0x9FFF => self.ppu.read(address),
            0xE000..=0xFDFF => self.read(address - WRAM_ECHO_OFFSET),
            0xFF44 => 0x90,//self.ppu.ly,
            _ => self.wram[u_addr],
            //_ => panic!("unsupported wram read access at {u_addr:x}"),
        }
    }

    /// Writes u8 to wram at address.
    pub fn write_u8(&mut self, address: u16, val: u8) {
        let u_addr = address as usize;
        match u_addr {
            0x0000..=0x7FFF => self.wram[u_addr] = val,
            0x8000..=0x9FFF => self.ppu.write_u8(address, val),
            0xE000..=0xFDFF => self.write_u8(address - WRAM_ECHO_OFFSET, val),
            0xFF44 => self.ppu.ly = val,
            _ => self.wram[address as usize] = val,
            //_ => panic!("unsupported wram write access at {u_addr:x}"),
        }
    }

    /// Writes u16 to wram at address.
    pub fn write_u16(&mut self, address: u16, val: u16) {
        let split = utils::split_u16(val);
        self.write_u8(address, split.1);
        self.write_u8(address + 1, split.0);
    }
}
