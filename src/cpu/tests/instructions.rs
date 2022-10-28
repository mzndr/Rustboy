use crate::cpu::Cpu;
use crate::cpu::instructions;

#[test]
fn ld_bc() { 
    // 0xAEFE
    let expected_b: u8 = 0xAE;
    let expected_c: u8 = 0xFE;
    let mut cpu: Cpu = Cpu::new();

    // Keep big endianess in mind.
    cpu.wram[0x100] = expected_c;
    cpu.wram[0x101] = expected_b;

    let mut expected_cpu = cpu.clone();
    let cycles_needed = instructions::ld_bc(&mut cpu);

    expected_cpu.registers.b = expected_b;
    expected_cpu.registers.c = expected_c;
    expected_cpu.registers.pc += 2;

    assert_eq!(cycles_needed, 3);
    assert_eq!(cpu, expected_cpu);
}
