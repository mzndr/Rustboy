mod cpu;
use cpu::{init_cpu, execute_current_instruction};

fn main() {
    let mut cpu = init_cpu();
    execute_current_instruction(&mut cpu);
}
