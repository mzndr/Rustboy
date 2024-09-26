use std::{
    i128,
    rc::Rc,
    thread,
    time::{self, Duration, Instant},
};

use crate::{apu::Apu, cpu::Cpu, mmu::Memory, ppu::Ppu};

/// Default gameboy clock speed.
const DEFAULT_CLOCK_SPEED: f32 = 4100f32;

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
            ppu: Ppu::new(memory.clone()),
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

        let sleep_nanos = ((1000.0 / cycles_per_ms) - passed_nano as f32) as i128;
        match sleep_nanos.cmp(&0) {
            std::cmp::Ordering::Equal => {
                return;
            }
            std::cmp::Ordering::Less => {
                tracing::warn!("can't keep up with clock, {} ns behind", -sleep_nanos);
                return;
            }
            std::cmp::Ordering::Greater => {
                tracing::trace!("{} ns ahead of clock", sleep_nanos);
            }
        };

        let delta = Duration::from_nanos(sleep_nanos as u64);

        thread::sleep(delta);
    }
}
