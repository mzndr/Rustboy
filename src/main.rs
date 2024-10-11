#![forbid(unsafe_code)]
#![deny(nonstandard_style)]
#![warn(clippy::pedantic, clippy::unwrap_used)]
#![allow(clippy::upper_case_acronyms, clippy::cast_lossless)]

use std::{fs, path};

use clap::Parser;
use tracing_subscriber::EnvFilter;

use crate::cpu::disassembler::disassemble_rom;
use crate::gb::Gameboy;

mod apu;
mod cpu;
mod gb;
mod mmu;
mod ppu;

/// Command line arguments, parsed by [`clap`].
#[derive(Parser, Debug)]
struct Args {
    rom_path: String,
    #[arg(short, long, action)]
    disassemble: bool,
    #[arg(long, action)]
    enable_gbd: bool,
    #[arg(long, action)]
    enable_trace: bool,
}

fn main() {
    let args = Args::parse();
    if args.enable_trace {
        tracing_subscriber::fmt::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .without_time()
            .init();
    }
    tracing::info!(?args, "starting emulator");

    let rom = fs::read(path::Path::new(&args.rom_path)).expect("cannot read ROM");

    if args.disassemble {
        let asm = disassemble_rom(&rom);
        print!("{asm}");
        return;
    }

    let mut gb = Gameboy::new(args.enable_gbd);
    gb.load_rom(&rom);
    gb.run();
}
