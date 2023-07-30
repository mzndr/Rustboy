#![forbid(unsafe_code)]
#![deny(nonstandard_style)]
#![warn(clippy::pedantic, clippy::unwrap_used)]
#![allow(dead_code)]

mod cpu;
use crate::cpu::Cpu;
use std::{fs, path};
use std::{thread, time};
use time::Duration;

const CLOCK_SPEED: f32 = 4100f32;
const ROM_PATH: &str = "./files/roms/Tetris.gb";

fn main() -> anyhow::Result<()> {
    let mut cpu = Cpu::new();
    cpu.load_rom(fs::read(path::Path::new(ROM_PATH))?.as_slice());
    clock_loop(&mut cpu)?;
    Ok(())
}

/// Executed in loop at roughly 4mhz.
fn clock_cycle() {}

/// Executed every 4 clock clycles or roughly 1mhz.
fn machine_cycle(cpu: &mut Cpu) -> anyhow::Result<()> {
    cpu.cycle()?;
    Ok(())
}

fn clock_loop(cpu: &mut Cpu) -> anyhow::Result<()> {
    let mut cycles: u8 = 1;
    loop {
        let start = time::Instant::now();
        clock_cycle();
        if cycles == 4 {
            cycles = 0;
            machine_cycle(cpu)?;
        }
        sleep_till_next_cycle(start);
        cycles += 1;
    }
}

//TODO: Actually properly convert values
#[allow(clippy::cast_precision_loss, clippy::cast_sign_loss, clippy::cast_possible_truncation)]
fn sleep_till_next_cycle(start: time::Instant) {
    let cycles_per_ms: f32 = CLOCK_SPEED / 1000.0; // 1mhz
    let after = time::Instant::now();
    let passed = after - start;
    let passed_nano = passed.as_nanos();
    let sleep_needed = Duration::from_nanos(((1000.0 / cycles_per_ms) - passed_nano as f32) as u64);

    thread::sleep(sleep_needed);
}
