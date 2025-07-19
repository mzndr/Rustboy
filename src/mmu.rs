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
    cpu::{
        interrupt::Interrupt,
        utils::{self, split_u16},
    },
    io::Io,
    mbc::{self, MBC},
    ppu::Ppu,
};

/// Gameboy wram size.
const WRAM_SIZE: usize = 0x2000;

/// Offset to handle echo ram redirection.
const WRAM_ECHO_OFFSET: u16 = 0x2000;

const HRAM_SIZE: usize = 0x7F;

/// Memory management unit. Handle and map memory access.
pub struct Mmu {
    wram: [u8; WRAM_SIZE],
    hram: [u8; HRAM_SIZE],
    ppu: Ppu,
    apu: Apu,
    io: Io,
    mbc: Box<dyn MBC>,

    pub interrupt_enable: u8,
    debug: crate::debug::Debug,
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
            io: Io::new(),
            interrupt_enable: 0,
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

    // cycle does one execution cycle of the mmu and all associated
    // parts, like the ppu. It returns all requested interrupts during
    // the cycle.
    pub fn cycle(&mut self) -> Vec<Interrupt> {
        let mut interrupts = Vec::new();
        interrupts.append(&mut self.ppu.cycle());
        interrupts.append(&mut self.io.cycle());
        interrupts
    }

    /// Reads from wram at address.
    pub fn read_u8(&self, address: u16) -> u8 {
        match address {
            // ROM, BANKS
            0x0000..=0x7FFF => self.mbc.read_rom(address),
            // VRAM
            0x8000..=0x9FFF => self.ppu.read_u8(address),
            // External RAM
            0xA000..=0xBFFF => self.mbc.read_ram(address),
            // WRAM
            0xC000..=0xDFFF => self.wram[address as usize - 0xC000],
            // Echo RAM
            0xE000..=0xFDFF => self.read_u8(address - WRAM_ECHO_OFFSET),
            // OAM
            0xFE00..=0xFE9F => todo!("OAM read"),
            // Not Usable
            0xFEA0..=0xFEFF => 0xFF,
            // PPU LY REGISTER
            0xFF44 => 0x90,
            // IO
            0xFF00..=0xFF7F => self.io.read_u8(address),
            // HRAM
            0xFF80..=0xFFFE => self.hram[address as usize - 0xFF80],
            // Interrupt Enable
            0xFFFF => self.interrupt_enable,
        }
    }

    pub fn read_u16(&self, address: u16) -> u16 {
        let l = self.read_u8(address);
        let h = self.read_u8(address + 1);
        utils::merge_u8s(h, l)
    }

    /// Writes u8 to wram at address.
    pub fn write_u8(&mut self, address: u16, val: u8) {
        match address {
            // ROM, BANKS
            0x0000..=0x7FFF => self.mbc.write_rom(address, val),
            // VRAM
            0x8000..=0x9FFF => self.ppu.write_u8(address, val),
            // External RAM
            0xA000..=0xBFFF => self.mbc.write_ram(address, val),
            // WRAM
            0xC000..=0xDFFF => self.wram[address as usize - 0xC000] = val,
            // Echo RAM
            0xE000..=0xFDFF => self.write_u8(address - WRAM_ECHO_OFFSET, val),
            // OAM
            0xFE00..=0xFE9F => todo!("OAM write"),
            // Not Usable
            0xFEA0..=0xFEFF => (),
            // PPU LY REGISTER
            0xFF44 => (),
            // IO
            0xFF00..=0xFF7F => self.io.write_u8(address, val),
            // HRAM
            0xFF80..=0xFFFE => self.hram[address as usize - 0xFF80] = val,
            // Interrupt Enable
            0xFFFF => self.interrupt_enable = val,
        }
    }

    /// Writes u16 to wram at address.
    pub fn write_u16(&mut self, address: u16, val: u16) {
        let (h, l) = split_u16(val);
        self.write_u8(address, l);
        self.write_u8(address + 1, h);
    }
}
