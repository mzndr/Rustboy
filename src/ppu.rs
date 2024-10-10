//! Picture processing unit. Handle sprite loading, vram memory management and delegate information
//! to the renderer.
//!
//! Start   End     Description                        Notes
//! 8000    9FFF    8 KiB Video RAM (VRAM)             In CGB mode, switchable bank 0/1

/// VRAM size.
pub const VRAM_SIZE: usize = 0x2000;
/// OAM Memory location in VRAM.
pub const VRAM_LY_OFFSET: usize = 0x0F44;
/// OAM Memory location in VRAM.
pub const VRAM_OAM_OFFSET: usize = 0x0E00;
/// LCD Control register
pub const VRAM_LCDC_OFFSET: usize = 0x0F40;
/// WY Register Memory location in VRAM.
pub const VRAM_WY_OFFSET: usize = 0x0F4A;
/// WX Register Memory location in VRAM.
pub const VRAM_WX_OFFSET: usize = 0x0F4B;
/// VRAM offset in WRAM.
pub const VRAM_OFFSET: usize = 0x8000;

pub const LY_VBLANK_START: u8 = 144;

/// PPU State. State cycles throughout operation and determines what the PPU does.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum State {
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
    /// Current pixel.
    lx: u8,
    /// Currently loaded sprites.
    sprite_buffer: Vec<Sprite>,

    t_cycle: u16,
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
            lx: 0,
            t_cycle: 0,
            state: State::OAMSearch,
            sprite_buffer: Vec::with_capacity(10),
        }
    }

    pub fn state(&self) -> State {
        self.state
    }

    /// Get the ly register value from vram;
    pub fn ly(&self) -> u8 {
        self.vram[VRAM_LY_OFFSET]
    }

    /// Set ly to a value
    fn set_ly(&mut self, val: u8) {
        self.vram[VRAM_LY_OFFSET] = val;
    }

    /// Increase ly by one and return it.
    fn inc_ly(&mut self) -> u8 {
        self.vram[VRAM_LY_OFFSET] = self.vram[VRAM_LY_OFFSET].wrapping_add(1);
        self.vram[VRAM_LY_OFFSET]
    }

    fn lcdc(&self) -> u8 {
        self.vram[VRAM_LCDC_OFFSET]
    }

    fn lcdc_display_enable(&self) -> bool {
        (self.lcdc() >> 7 & 1) == 1
    }

    fn lcdc_window_tile_map_select(&self) -> bool {
        (self.lcdc() >> 6 & 1) == 1
    }

    fn lcdc_window_display_enable(&self) -> bool {
        (self.lcdc() >> 5 & 1) == 1
    }

    fn lcdc_tile_data_select_mode(&self) -> bool {
        (self.lcdc() >> 4 & 1) == 1
    }

    fn lcdc_bg_tile_map_select_mode(&self) -> bool {
        (self.lcdc() >> 3 & 1) == 1
    }

    fn lcdc_sprite_height(&self) -> bool {
        (self.lcdc() >> 2 & 1) == 1
    }

    fn lcdc_sprite_enable(&self) -> bool {
        (self.lcdc() >> 1 & 1) == 1
    }

    fn lcdc_bg_enable(&self) -> bool {
        (self.lcdc() >> 0 & 1) == 1
    }

    /// Load a sprites information (not pixel data) from OAM Memory.
    #[allow(clippy::cast_possible_wrap)]
    fn oam_load_sprite(&self, sprite_position: u8) -> Option<Sprite> {
        let sprite_address = VRAM_OAM_OFFSET + (sprite_position * 4) as usize;
        let sprite = Sprite {
            y: self.vram[sprite_address] as i8,
            x: self.vram[sprite_address + 1] as i8,
            tile_number: self.vram[sprite_address + 2],
            sprite_flags: self.vram[sprite_address + 3].into(),
        };

        let sprite_height = if self.lcdc_sprite_height() { 16 } else { 8 };

        // Check if the sprite is on screen.
        if sprite.x > 0
            && ((self.ly() + 16) as i8) >= sprite.y
            && ((self.ly() + 16) as i8) < sprite.y + sprite_height
            && self.sprite_buffer.len() <= 10
        {
            return Some(sprite);
        }

        None
    }

    /// Perform the OAM scan, loading [`Sprite`] information into the `sprite_buffer`.
    fn oam_scan(&mut self) {
        for i in 0..19 {
            if let Some(sprite) = self.oam_load_sprite(i) {
                self.sprite_buffer.push(sprite);
                tracing::warn!("{}", self.sprite_buffer.len());
            }
        }
    }

    fn pixel_transfer(&mut self) {}

    /// Reads from vram at address.
    pub fn read(&self, address: u16) -> u8 {
        let u_addr = (address as usize) - VRAM_OFFSET;

        match u_addr {
            VRAM_LY_OFFSET => 0x90,
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

    /// Cycle the mmu, returning the resulting state.
    #[tracing::instrument(skip(self) fields(sprites_loaded=%self.sprite_buffer.len()))]
    pub fn cycle(&mut self) -> State {
        match self.state {
            State::OAMSearch => {
                tracing::trace!("performing orm search");
                if self.t_cycle % 80 == 0 {
                    self.oam_scan();
                    self.state = State::PixelTransfer;
                }
            }
            State::PixelTransfer => {
                tracing::trace!("performing pixel transfer");
                self.pixel_transfer();
                self.state = State::HBlank;
            }
            State::HBlank => {
                tracing::trace!("performing horizontal blanks");
                if self.t_cycle % 456 == 0 {
                    self.inc_ly();
                    self.sprite_buffer.resize(0, Sprite::default());
                    if self.ly() >= LY_VBLANK_START {
                        self.state = State::VBlank;
                    } else {
                        self.state = State::OAMSearch;
                    }
                }
            }
            State::VBlank => {
                tracing::trace!("horizontal vertical blanks");
                if self.t_cycle % 4560 == 0 {
                    self.set_ly(0);
                    self.state = State::OAMSearch;
                }
            }
        };
        self.t_cycle = self.t_cycle.wrapping_add(1);

        self.state
    }
}
