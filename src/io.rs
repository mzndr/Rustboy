use core::panic;

use crate::cpu::interrupt::Interrupt;

// IO offset in memory
const IO_OFFSET: usize = 0xFF00;
// Size of IO range in memory
const IO_SIZE: usize = 0x70;

const SERIAL_BUFFER_SIZE: usize = 2;

pub struct Io {
    memory: [u8; IO_SIZE],
    serial_buf: [u8; SERIAL_BUFFER_SIZE],
}

impl Io {
    pub fn new() -> Self {
        Self {
            memory: [0; IO_SIZE],
            serial_buf: [0; SERIAL_BUFFER_SIZE],
        }
    }

    pub fn read_u8(&self, address: u16) -> u8 {
        let address = address as usize - IO_OFFSET;
        match address {
            0x00..=IO_SIZE => self.memory[address],
            _ => panic!("invalid IO read"),
        }
    }

    pub fn cycle(&mut self) -> Vec<Interrupt> {
        let mut interrupts = Vec::new();
        interrupts
    }

    pub fn write_u8(&mut self, address: u16, val: u8) {
        let address = address as usize - IO_OFFSET;
        match address {
            0x00..=IO_SIZE => self.memory[address] = val,
            _ => panic!("invalid IO write"),
        }
    }

    /// TODO: Add proper serial handling
    fn serial_write(val: u8) {
        let c = val as char;
        if !c.is_ascii() || !c.is_whitespace() || !c.is_control() {
            return;
        }
        tracing::info!("[SERIAL]: {}", val as char);
    }
}
