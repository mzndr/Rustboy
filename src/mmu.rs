use std::{
    array,
    cell::Cell,
    ops::{Deref, DerefMut},
};

use crate::cpu::utils;

/// Gameboy memory size.
const MEMORY_SIZE: usize = 0x10000;

/// Memory as cells to be shared mutable between chips.
pub type Cells = [Cell<u8>; MEMORY_SIZE];

#[derive(Debug, Clone, PartialEq)]
pub struct Memory(Cells);

impl Deref for Memory {
    type Target = Cells;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Memory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Memory {
    /// Create new memory.
    pub fn new() -> Self {
        Self(array::from_fn(|_| Cell::new(0x00)))
    }

    /// Checks if an address is in valid space,
    /// prints an error message and quits if not.
    fn check_address(address: u16) {
        if address as usize > MEMORY_SIZE + 1 {
            tracing::error!("bad memory access at 0x{:x}", &address);
            panic!("bad memory access at 0x{:x}", &address)
        }
    }

    /// Reads from wram at address.
    pub fn read(&self, address: u16) -> u8 {
        let u_addr = address as usize;
        Self::check_address(address);
        self[u_addr].get()
    }

    /// Writes u8 to wram at address.
    pub fn write_u8(&self, address: u16, val: u8) {
        let u_addr = address as usize;
        Self::check_address(address);
        self[u_addr].set(val);
    }

    /// Writes u16 to wram at address.
    pub fn write_u16(&self, address: u16, val: u16) {
        let split = utils::split_u16(val);
        self.write_u8(address, split.1);
        self.write_u8(address + 1, split.0);
    }
}
