//! Memory management unit. Handle and map memory access, delegates reads and writes
//! to the corresponding chip (ppu, apu, gamepad input)
//!
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

use std::array;

use crate::{
    apu::Apu,
    cpu::utils::{self, split_u16},
    mbc::{self, MBC},
    ppu::Ppu,
};

/// Gameboy wram size.
const WRAM_SIZE: usize = 0x10000;

/// Offset to handle echo ram redirection.
const WRAM_ECHO_OFFSET: u16 = 0x2000;

const HRAM_SIZE: usize = 0x7F;

/// Memory management unit. Handle and map memory access.
pub struct Mmu {
    wram: [u8; WRAM_SIZE],
    hram: [u8; HRAM_SIZE],
    ppu: Ppu,
    apu: Apu,
    mbc: Box<dyn MBC>,

    debug: crate::debug::Debug,

    /// Interrupt master enable.
    pub ime: bool,
}

impl Mmu {
    /// Create new wram.
    pub fn new(rom: &[u8], debug: crate::debug::Debug) -> Self {
        tracing::info!("initializing mmu");
        let mut mmu = Self {
            wram: array::from_fn(|_| rand::random()),
            hram: array::from_fn(|_| rand::random()),
            ppu: Ppu::new(),
            apu: Apu::new(),
            mbc: mbc::load_cartridge(rom),
            ime: false,
            debug,
        };
        mmu.initial_write();
        mmu
    }

    fn initial_write(&mut self) {
        self.write_u8(0xFF05, 0x00);
        self.write_u8(0xFF06, 0x00);
        self.write_u8(0xFF07, 0x00);
        self.write_u8(0xFF10, 0x80);
        self.write_u8(0xFF11, 0xBF);
        self.write_u8(0xFF12, 0xF3);
        self.write_u8(0xFF14, 0xBF);
        self.write_u8(0xFF16, 0x3F);
        self.write_u8(0xFF16, 0x3F);
        self.write_u8(0xFF17, 0x00);
        self.write_u8(0xFF19, 0xBF);
        self.write_u8(0xFF1A, 0x7F);
        self.write_u8(0xFF1B, 0xFF);
        self.write_u8(0xFF1C, 0x9F);
        self.write_u8(0xFF1E, 0xFF);
        self.write_u8(0xFF20, 0xFF);
        self.write_u8(0xFF21, 0x00);
        self.write_u8(0xFF22, 0x00);
        self.write_u8(0xFF23, 0xBF);
        self.write_u8(0xFF24, 0x77);
        self.write_u8(0xFF25, 0xF3);
        self.write_u8(0xFF26, 0xF1);
        self.write_u8(0xFF40, 0x91);
        self.write_u8(0xFF42, 0x00);
        self.write_u8(0xFF43, 0x00);
        self.write_u8(0xFF45, 0x00);
        self.write_u8(0xFF47, 0xFC);
        self.write_u8(0xFF48, 0xFF);
        self.write_u8(0xFF49, 0xFF);
        self.write_u8(0xFF4A, 0x00);
        self.write_u8(0xFF4B, 0x00);
    }

    pub fn cycle(&mut self) {
        self.ppu.cycle();
    }

    pub fn ppu_ref(&self) -> &Ppu {
        &self.ppu
    }

    /// Reads from wram at address.
    pub fn read_u8(&self, address: u16) -> u8 {
        let u_addr = address as usize;

        match u_addr {
            0x0000..=0x7FFF => self.mbc.read_rom(address),
            0x8000..=0x9FFF => self.ppu.read_u8(address),
            0xA000..=0xBFFF => self.mbc.read_ram(address),

            0xE000..=0xFDFF => self.read_u8(address - WRAM_ECHO_OFFSET),
            0xFF44 => 0x90,                                        //self.ppu.ly,
            0xFF80..=0xFFFE => self.hram[address as usize & 0x7F], // 0x7F -> divide number
            // by two if msb is 1
            _ => self.wram[u_addr],
            //_ => self.wram[u_addr],
            //_ => panic!("unsupported wram read access at {u_addr:x}"),
        }
    }

    pub fn read_u16(&self, address: u16) -> u16 {
        let l = self.read_u8(address);
        let h = self.read_u8(address + 1);
        utils::merge_u8s(h, l)
    }

    /// Writes u8 to wram at address.
    pub fn write_u8(&mut self, address: u16, val: u8) {
        let u_addr = address as usize;

        match u_addr {
            0x0000..=0x7FFF => self.mbc.write_rom(address, val),
            0x8000..=0x9FFF => self.ppu.write_u8(address, val),
            0xA000..=0xBFFF => self.mbc.write_ram(address, val),

            0xE000..=0xFDFF => self.write_u8(address - WRAM_ECHO_OFFSET, val),
            0xFF01..=0xFF02 => Self::serial_write(val),
            0xFF44 => self.ppu.ly = val,
            0xFF80..=0xFFFE => self.hram[address as usize & 0x7F] = val, // 0x7F -> divide number
            // by two if msb is 1
            _ => self.wram[address as usize] = val,
            //_ => panic!("unsupported wram write access at {u_addr:x}"),
        }
    }

    /// TODO: Add proper serial handling
    pub fn serial_write(val: u8) {
        let c = val as char;
        if !c.is_ascii() || !c.is_whitespace() || !c.is_control() {
            return;
        }
        tracing::info!("[SERIAL]: {}", val as char);
    }

    /// Writes u16 to wram at address.
    pub fn write_u16(&mut self, address: u16, val: u16) {
        let (h, l) = split_u16(val);
        self.write_u8(address, l);
        self.write_u8(address + 1, h);
    }
}
