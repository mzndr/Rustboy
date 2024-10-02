//! Picture processing unit. Handle sprite loading, vram memory management and delegate information
//! to the renderer.
//!
//! Start   End     Description                        Notes
//! 8000    9FFF    8 KiB Video RAM (VRAM)             In CGB mode, switchable bank 0/1

use std::array;

/// VRAM size.
pub const VRAM_SIZE: usize = 0x2000;
/// OAM Memory location in VRAM.
pub const VRAM_OAM_OFFSET: usize = 0x0E00;
/// WY Register Memory location in VRAM.
pub const VRAM_WY_OFFSET: usize = 0x0F4A;
/// WX Register Memory location in VRAM.
pub const VRAM_WX_OFFSET: usize = 0x0F4B;
/// VRAM offset in WRAM.
pub const VRAM_OFFSET: usize = 0x8000;

/// PPU State. State cycles throughout operation and determines what the PPU does.
#[derive(Debug, Clone)]
enum State {
    /// Load sprite pixels for current scanline.
    OAMSearch,
    /// Draw pixels to LCD (pass them to the renderer).
    PixelTransfer,
    /// Idle for remainder of scanline.
    HBlank,
    /// Idle for V-Blank scanlines.
    VBlank,
}

#[derive(Debug, Clone)]
pub struct Ppu {
    /// Video Memory.
    vram: [u8; VRAM_SIZE],
    /// PPU State.
    state: State,
    /// Current scanline.
    ly: i8,
    /// Currently loaded sprites.
    sprite_buffer: Vec<Sprite>,
}

#[derive(Debug, Clone, Copy)]
struct SpriteFlags {
    /// Object background priority.
    pub(self) obj_to_bg_priority: bool,
    /// Flip the sprite on y axis?
    pub(self) y_flip: bool,
    /// Flip the sprite on x axis?
    pub(self) x_flip: bool,
    /// Use OBP0 or OBP1 register for the color palette.
    pub(self) palette_number: bool,
}

impl From<u8> for SpriteFlags {
    fn from(value: u8) -> Self {
        Self {
            obj_to_bg_priority: (value >> 7 & 1) == 1,
            y_flip: (value >> 6 & 1) == 1,
            x_flip: (value >> 5 & 1) == 1,
            palette_number: (value >> 4 & 1) == 1,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Sprite {
    pub(self) x: i8,
    pub(self) y: i8,
    pub(self) tile_number: u8,
    pub(self) sprite_flags: SpriteFlags,
}

impl Default for Sprite {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            tile_number: 0,
            sprite_flags: 0.into(),
        }
    }
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            vram: [0; VRAM_SIZE],
            state: State::OAMSearch,
            ly: 0,
            sprite_buffer: Vec::with_capacity(10),
        }
    }

    #[allow(clippy::cast_possible_wrap)]
    fn oam_load_sprite(&self, sprite_position: u8) -> Option<Sprite> {
        let sprite_address = VRAM_OAM_OFFSET + (sprite_position * 4) as usize;
        let sprite = Sprite {
            y: self.vram[sprite_address] as i8,
            x: self.vram[sprite_address + 1] as i8,
            tile_number: self.vram[sprite_address + 2],
            sprite_flags: self.vram[sprite_address + 3].into(),
        };

        // TODO: Check how to determine sprite height
        let sprite_height = 8;

        // Check if the sprite is on screen.
        if sprite.x < 0
            || sprite.y > (self.ly + 16)
            || sprite.y < (self.ly + sprite_height)
            || self.sprite_buffer.len() >= 10
        {
            return None;
        }

        Some(sprite)
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

    #[tracing::instrument()]
    pub fn cycle(&self) {
        match self.state {
            State::OAMSearch => {}
            State::HBlank => {}
            State::PixelTransfer => {}
            State::VBlank => {}
        };
    }
}
