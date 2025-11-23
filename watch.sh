#!/bin/sh
RUST_LOG=trace cargo run ./files/game-boy-test-roms/blargg-test-roms/cpu_instrs/individual/01-special.gb --enable-gbd --uncap-clock-speed | ./files/gb-doctor/gameboy-doctor - cpu_instrs 1
