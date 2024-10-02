//! Picture processing unit. Handle sprite loading, vram memory management and delegate information
//! to the renderer.
//!
//! Start   End     Description                        Notes
//! 8000    9FFF    8 KiB Video RAM (VRAM)             In CGB mode, switchable bank 0/1

/// VRAM size.
pub const VRAM_SIZE: usize = 0x2000;
/// VRAM offset in WRAM.
pub const VRAM_OFFSET: usize = 0x8000;

/// PPU State. State cycles throughout operation and determines what the PPU does.
#[derive(Debug, Clone)]
pub enum State {
    OAMSearch,
    PixelTransfer,
    HBlank,
    VBlank,
}

#[derive(Debug, Clone)]
pub struct Ppu {
    vram: [u8; VRAM_SIZE],
    state: State,
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            vram: [0; VRAM_SIZE],
            state: State::OAMSearch,
        }
    }

    /// Reads from vram at address.
    pub fn read(&self, address: u16) -> u8 {
        let u_addr = (address as usize) - VRAM_OFFSET;

        match u_addr {
            0x0000..=0x1FFF => self.vram[u_addr],
            _ => panic!("unsupported vram read access at 0x{u_addr:x}, was 0x{address:x}"),
        }
    }

    /// Writes u8 to vram at address.
    pub fn write_u8(&mut self, address: u16, val: u8) {
        let u_addr = (address as usize) - VRAM_OFFSET;
        match u_addr {
            0x0000..=0x1FFF => self.vram[u_addr] = val,
            _ => panic!("unsupported vram write access at 0x{u_addr:x}, was 0x{address:x}"),
        }
    }

    pub fn cycle(&self) {}
}
