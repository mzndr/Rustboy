use core::panic;

use crate::cpu::interrupt::Interrupt;

// IO offset in memory
const IO_OFFSET: usize = 0xFF00;
// Size of IO range in memory
const IO_SIZE: usize = 0x70;

const SERIAL_BUFFER_SIZE: usize = 2;

// IO Registers
const REGISTER_DIV_OFFSET: usize = 0x04;
const REGISTER_TIMA_OFFSET: usize = 0x05;
const REGISTER_TMA_OFFSET: usize = 0x06;
const REGISTER_TAC_OFFSET: usize = 0x07;

pub struct Io {
    memory: [u8; IO_SIZE],
    serial_buf: [u8; SERIAL_BUFFER_SIZE],
    cycles: u8,

    div: u8,
    tima: u8,
    tma: u8,
    tac: u8,
}

impl Io {
    pub fn new() -> Self {
        Self {
            cycles: 0,
            memory: [0; IO_SIZE],
            serial_buf: [0; SERIAL_BUFFER_SIZE],

            div: 0,
            tima: 0,
            tma: 0,
            tac: 0,
        }
    }

    pub fn read_u8(&self, address: u16) -> u8 {
        let address = address as usize - IO_OFFSET;
        match address {
            REGISTER_DIV_OFFSET => self.div,
            REGISTER_TIMA_OFFSET => self.tima,
            REGISTER_TMA_OFFSET => self.tma,
            REGISTER_TAC_OFFSET => self.tac,

            0x00..=IO_SIZE => self.memory[address],
            _ => panic!("invalid IO read"),
        }
    }

    pub fn cycle(&mut self) -> Vec<Interrupt> {
        let mut interrupts = Vec::new();
        if self.tima == 0xFF {
            self.tima = self.tac;
            interrupts.push(Interrupt::Timer);
        }

        self.div = self.div.wrapping_add(1);
        if self.timer_enabled() && self.cycles % self.clock_select() == 0 {
            self.tima = self.tima.wrapping_add(1);
        }

        self.cycles = self.cycles.wrapping_add(1);

        interrupts
    }

    pub fn reset_div(&mut self) {
        self.div = 0x00;
    }

    fn timer_enabled(&self) -> bool {
        self.tac & 0b100 == 0b100
    }

    fn clock_select(&self) -> u8 {
        let val = self.tac & 0b11;
        match val {
            0 => 255,
            1 => 4,
            2 => 16,
            3 => 64,
            _ => panic!("invalid clock select"),
        }
    }

    pub fn write_u8(&mut self, address: u16, val: u8) {
        let address = address as usize - IO_OFFSET;
        match address {
            REGISTER_DIV_OFFSET => self.reset_div(),
            REGISTER_TIMA_OFFSET => self.tima = val,
            REGISTER_TMA_OFFSET => self.tma = val,
            REGISTER_TAC_OFFSET => self.tac = val,

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
