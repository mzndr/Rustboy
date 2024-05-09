#![forbid(unsafe_code)]
#![deny(nonstandard_style)]
#![warn(clippy::pedantic, clippy::unwrap_used)]

use std::{fs, path};
use std::{thread, time};

use clap::Parser;
use time::Duration;

use crate::cpu::Cpu;

mod cpu;

const CLOCK_SPEED: f32 = 4100f32;

/// Command line arguments, parsed by [`clap`].
#[derive(Parser, Debug)]
struct Args {
    rom_path: String,
}

fn main() {
    tracing_subscriber::fmt::init();

    let args = Args::parse();
    tracing::info!(?args, "starting emulator");

    let mut cpu = Cpu::new();

    cpu.load_rom(
        fs::read(path::Path::new(&args.rom_path))
            .expect("cannot read ROM")
            .as_slice(),
    );
    clock_loop(&mut cpu);
}

/// Executed in loop at roughly 4mhz.
fn clock_cycle() {}

/// Executed every 4 clock clycles or roughly 1mhz.
fn machine_cycle(cpu: &mut Cpu) {
    cpu.cycle();
}

fn clock_loop(cpu: &mut Cpu) {
    let mut cycles: u8 = 1;
    loop {
        let start = time::Instant::now();
        clock_cycle();
        if cycles == 4 {
            cycles = 0;
            machine_cycle(cpu);
        }
        sleep_till_next_cycle(start);
        cycles += 1;
    }
}

//TODO: Actually properly convert values
#[allow(
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation
)]
fn sleep_till_next_cycle(start: time::Instant) {
    let cycles_per_ms: f32 = CLOCK_SPEED / 1000.0; // 1mhz
    let after = time::Instant::now();
    let passed = after - start;
    let passed_nano = passed.as_nanos();
    let delta = Duration::from_nanos(((1000.0 / cycles_per_ms) - passed_nano as f32) as u64);

    thread::sleep(delta);
}
