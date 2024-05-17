use std::{
    thread,
    time::{self, Duration, Instant},
};

use crate::{apu::Apu, cpu::Cpu, ppu::Ppu};

const DEFAULT_CLOCK_SPEED: f32 = 4100f32;

pub struct Gameboy {
    pub cpu: Cpu,
    pub ppu: Ppu,
    pub apu: Apu,

    pub clock_speed: f32,
}

impl Gameboy {
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            ppu: Ppu::new(),
            apu: Apu::new(),
            clock_speed: DEFAULT_CLOCK_SPEED,
        }
    }

    /// Needs to be changed for bigger games, since they
    /// are too big to fit into ram, so banking has to be
    /// implemented.
    pub fn load_rom(&mut self, rom: &[u8]) {
        for (address, byte) in rom.iter().enumerate() {
            self.cpu.wram[address] = *byte;
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
