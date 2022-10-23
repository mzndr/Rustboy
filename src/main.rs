mod cpu;
use crate::cpu::init_cpu;
use crate::cpu::clock_cycle;

fn main() {
    let mut cpu = init_cpu();
    clock_cycle(&mut cpu);
}
