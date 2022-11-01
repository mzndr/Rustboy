use crate::cpu::Cpu;
use crate::cpu::instructions;

#[test]
fn nop() { 
    let mut cpu: Cpu = Cpu::new();
    let expected_cpu = cpu.clone();
    let cycles_needed = cpu.nop();
    assert_eq!(cycles_needed, 1);
    assert_eq!(cpu, expected_cpu);
}

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
    let cycles_needed = cpu.ld_bc_d16();

    expected_cpu.registers.b = expected_b;
    expected_cpu.registers.c = expected_c;
    expected_cpu.registers.pc += 2;

    assert_eq!(cycles_needed, 3);
    assert_eq!(cpu, expected_cpu);
}


#[test]
fn inc_b() { 
    let mut cpu = Cpu::new();
    cpu.registers.b = 0xFF;
    cpu.inc_b();
    assert_eq!(cpu.registers.b, 0x00);
    assert_eq!(cpu.registers.get_flag_z(), 1);
    assert_eq!(cpu.registers.get_flag_h(), 1);
    assert_eq!(cpu.registers.get_flag_n(), 0);
}

#[test]
fn dec_b() { 
    let mut cpu = Cpu::new();
    cpu.registers.b = 0x01;
    cpu.dec_b();
    assert_eq!(cpu.registers.b, 0x00);
    assert_eq!(cpu.registers.get_flag_z(), 1);
    assert_eq!(cpu.registers.get_flag_h(), 0);
    assert_eq!(cpu.registers.get_flag_n(), 1);

    cpu.registers.b = 0x00;
    cpu.dec_b();
    assert_eq!(cpu.registers.b, 0xFF);
    assert_eq!(cpu.registers.get_flag_z(), 0);
    assert_eq!(cpu.registers.get_flag_h(), 1);
    assert_eq!(cpu.registers.get_flag_n(), 1);
}

#[test]
fn ld_b_d8() {
    let mut cpu = Cpu::new();
    cpu.wram[0x100] = 0xAE;
    cpu.ld_b_d8();
    assert_eq!(cpu.registers.b, 0xAE);
}

#[test]
fn rlca() {
    let mut cpu = Cpu::new();
    cpu.registers.a = 0b10000001;
    cpu.rlca();
    assert_eq!(cpu.registers.a, 0b00000011);
    assert_eq!(cpu.registers.get_flag_c(), 1);
    cpu.registers.a = 0b00000001;
    cpu.rlca();
    assert_eq!(cpu.registers.a, 0b00000010);
    assert_eq!(cpu.registers.get_flag_c(), 0);
    assert_eq!(cpu.registers.get_flag_z(), 0);
    assert_eq!(cpu.registers.get_flag_n(), 0);
    assert_eq!(cpu.registers.get_flag_h(), 0);
}

#[test]
fn ld_a16p_sp() {
    let mut cpu = Cpu::new();
    cpu.registers.sp = 0xBEEF;
    cpu.wram[0x100] = 0x20;
    cpu.wram[0x101] = 0x25;
    cpu.ld_a16p_sp();
    assert_eq!(cpu.wram[0x2520], 0xEF);
    assert_eq!(cpu.wram[0x2521], 0xBE);

}

#[test]
fn add_hl_bc() {
    let mut cpu = Cpu::new();
    let a = 0x1000;
    let b = 0x1000;
    let sum = a + b;
    cpu.registers.set_hl(a);
    cpu.registers.set_bc(b);
    cpu.add_hl_bc();
    assert_eq!(cpu.registers.get_hl(), sum);
    assert_eq!(cpu.registers.get_flag_n(), 0);
    assert_eq!(cpu.registers.get_flag_c(), 0);
    assert_eq!(cpu.registers.get_flag_h(), 0);
}
