#![allow(dead_code)]
mod cpu;
use crate::cpu::Cpu;
use std::{fs, path};
use std::{
    thread,
    time,
};
use time::Duration;

const CLOCK_SPEED: u16 = 4100;

pub mod helpers;

fn main() {
    let mut cpu = Cpu::new();
    let tetris = fs::read(path::Path::new("./files/roms/Tetris.gb")).unwrap();

    cpu.load_rom(tetris);
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
        sleep_til_next_cycle(start);
        cycles += 1;
    }
}

fn sleep_til_next_cycle(start: time::Instant) {
    let cycles_per_ms: f32 = CLOCK_SPEED as f32 / 1000.0; // 1mhz
    let after = time::Instant::now();
    let passed = after - start;
    let passed_nano = passed.as_nanos();
    let sleep_needed = Duration::from_nanos(((1000.0 / cycles_per_ms) - passed_nano as f32) as u64);

    thread::sleep(sleep_needed);
}
