use std::rc::Rc;

use crate::mmu::Memory;

pub const MEMORY_OFFSET_VRAM: u16 = 0x8000;

pub struct Ppu {
    pub memory: Rc<Memory>,
}

pub type Tile = [u8; 8 * 2];

impl Ppu {
    pub fn new(memory: Rc<Memory>) -> Self {
        Self { memory }
    }

    /// Load a tile the '8000' method.
    fn load_tile_8000(&self, tile_number: u8) -> Tile {
        let tile_number = u16::from(tile_number);
        let base_address = MEMORY_OFFSET_VRAM + (16 * tile_number);
        [
            self.memory.read(base_address),
            self.memory.read(base_address + 0x1),
            self.memory.read(base_address + 0x2),
            self.memory.read(base_address + 0x3),
            self.memory.read(base_address + 0x4),
            self.memory.read(base_address + 0x5),
            self.memory.read(base_address + 0x6),
            self.memory.read(base_address + 0x7),
            self.memory.read(base_address + 0x8),
            self.memory.read(base_address + 0x9),
            self.memory.read(base_address + 0xa),
            self.memory.read(base_address + 0xb),
            self.memory.read(base_address + 0xc),
            self.memory.read(base_address + 0xd),
            self.memory.read(base_address + 0xe),
            self.memory.read(base_address + 0xf),
        ]
    }

    /// Load a tile the '8000' method.
    fn load_tile_8800(&self, tile_number: i8) -> Tile {
        todo!();
    }
}
