use std::{
    array,
    cell::Cell,
    ops::{Deref, DerefMut},
    rc::Rc,
    thread,
    time::{self, Duration, Instant},
};

use crate::{
    apu::Apu,
    cpu::{utils, Cpu},
    ppu::Ppu,
};

const DEFAULT_CLOCK_SPEED: f32 = 4100f32;

const MEMORY_SIZE: usize = 0x10000; //0x20 * 0x400;

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
        assert!((address as usize).lt(&MEMORY_SIZE));
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

pub struct Gameboy {
    pub cpu: Cpu,
    pub ppu: Ppu,
    pub apu: Apu,
    pub memory: Rc<Memory>,

    pub clock_speed: f32,
}

impl Gameboy {
    pub fn new() -> Self {
        let memory = Rc::new(Memory::new());
        Self {
            cpu: Cpu::new(memory.clone()),
            ppu: Ppu::new(),
            apu: Apu::new(),
            memory,
            clock_speed: DEFAULT_CLOCK_SPEED,
        }
    }

    /// Needs to be changed for bigger games, since they
    /// are too big to fit into ram, so banking has to be
    /// implemented.
    pub fn load_rom(&mut self, rom: &[u8]) {
        for (address, byte) in rom.iter().enumerate() {
            self.memory[address].set(*byte);
        }
    }

    /// Executed in loop at roughly 4mhz.
    fn clock_cycle(&mut self) {}

    pub fn run(&mut self) {
        let mut cycles: u8 = 1;
        loop {
            let start = time::Instant::now();
            self.clock_cycle();
            if cycles == 4 {
                cycles = 0;
                self.cpu.cycle();
            }
            self.sleep_till_next_cycle(start);
            cycles += 1;
        }
    }

    //TODO: Actually properly convert values
    #[allow(
        clippy::cast_precision_loss,
        clippy::cast_sign_loss,
        clippy::cast_possible_truncation
    )]
    fn sleep_till_next_cycle(&self, start: time::Instant) {
        let cycles_per_ms: f32 = self.clock_speed / 1000.0; // 1mhz
        let after = Instant::now();
        let passed = after - start;
        let passed_nano = passed.as_nanos();
        let delta = Duration::from_nanos(((1000.0 / cycles_per_ms) - passed_nano as f32) as u64);
        thread::sleep(delta);
    }
}
