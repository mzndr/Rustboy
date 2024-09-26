//! Picture processing unit. Handle sprite loading, vram memory management and delegate information
//! to the renderer.

pub const MEMORY_OFFSET_VRAM: u16 = 0x8000;
pub const VRAM_SIZE: usize = 0x1000;

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
}
