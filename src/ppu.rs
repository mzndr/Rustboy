//! Picture processing unit. Handle sprite loading, vram memory management and delegate information
//! to the renderer.

/// VRAM size.
pub const VRAM_SIZE: usize = 0x1000;
/// VRAM offset in WRAM.
pub const VRAM_OFFSET: usize = 0x7FFF;

#[derive(Debug, Clone)]
pub struct Ppu {
    vram: [u8; VRAM_SIZE],
}

pub type Tile = [u8; 8 * 2];

impl Ppu {
    pub fn new() -> Self {
        Self {
            vram: [0; VRAM_SIZE],
        }
    }

    /// Reads from vram at address.
    pub fn read(&self, address: u16) -> u8 {
        let u_addr = (address as usize) - VRAM_OFFSET;

        match u_addr {
            0x00..=0x7FFF => self.vram[u_addr],
            _ => panic!("unsupported read access at {u_addr:x}"),
        }
    }

    /// Writes u8 to vram at address.
    pub fn write_u8(&mut self, address: u16, val: u8) {
        let u_addr = (address as usize) - VRAM_OFFSET;
        match u_addr {
            0x00..=0x7FFF => self.vram[u_addr] = val,
            _ => panic!("unsupported write access at {u_addr:x}"),
        }
    }

    pub fn cycle(&self) {}
}
