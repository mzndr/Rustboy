mod cpu;
use cpu::{init_cpu, execute_current_instruction};
use cpu::types::Cpu;

fn main() {
    let mut cpu = init_cpu();
    execute_current_instruction(&mut cpu);
    clock(&mut cpu);
}

fn clock(cpu: &mut Cpu) {
    while true {
        execute_current_instruction(cpu);
    }
}
