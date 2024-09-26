use std::{
    i128, thread,
    time::{self, Duration, Instant},
};

use crate::cpu::Cpu;

/// Default gameboy clock speed.
const DEFAULT_CLOCK_SPEED: f32 = 4100f32 / 4f32;

pub struct Gameboy {
    pub cpu: Cpu,
}

impl Gameboy {
    pub fn new() -> Self {
        Self { cpu: Cpu::new() }
    }

    pub fn run(&mut self) {
        loop {
            let start = time::Instant::now();
            self.cpu.cycle();
            self.sleep_till_next_cycle(start);
        }
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        self.cpu.mmu.load_rom(rom);
    }

    //TODO: Actually properly convert values
    #[allow(
        clippy::cast_precision_loss,
        clippy::cast_sign_loss,
        clippy::cast_possible_truncation
    )]
    fn sleep_till_next_cycle(&self, start: time::Instant) {
        let cycles_per_ms: f32 = DEFAULT_CLOCK_SPEED / 1000.0; // 1mhz
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
