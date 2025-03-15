use std::{
    thread,
    time::{self, Duration, Instant},
};

use crate::{cpu::Cpu, Args};

/// Default gameboy clock speed.
const DEFAULT_CLOCK_SPEED: f32 = 4100f32 / 4f32;

pub struct Config {
    pub gb_doctor_enable: bool,
    pub uncap_clock_speed: bool,
}

impl From<Args> for Config {
    fn from(args: Args) -> Self {
        Self {
            gb_doctor_enable: args.enable_gbd,
            uncap_clock_speed: args.uncap_clock_speed,
        }
    }
}

pub struct Gameboy {
    pub cpu: Cpu,
    pub cfg: Config,
}

impl Gameboy {
    pub fn new(rom: &[u8], cfg: Config) -> Self {
        Self {
            cpu: Cpu::new(rom, crate::debug::Debug::new(rom, cfg.gb_doctor_enable)),
            cfg,
        }
    }

    pub fn run(&mut self) {
        loop {
            let start = time::Instant::now();
            self.cpu.cycle();
            Self::sleep_till_next_cycle(start, self.cfg.uncap_clock_speed);
        }
    }

    //TODO: Actually properly convert values
    #[allow(
        clippy::cast_precision_loss,
        clippy::cast_sign_loss,
        clippy::cast_possible_truncation
    )]
    fn sleep_till_next_cycle(start: time::Instant, uncap_clock_speed: bool) {
        if uncap_clock_speed {
            return;
        }

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
                tracing::trace!("can't keep up with clock, {} ns behind", -sleep_nanos);
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
