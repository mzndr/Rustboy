use crate::cpu::registers::Registers;

#[test]
fn get_register_logic() {
    let mut r = Registers::new();
    r.f = 0b10000000;
    assert_eq!(r.get_flag_z(), 1);
    r.f = 0b00000000;
    assert_eq!(r.get_flag_z(), 0);
}
#[test]
fn set_register_logic_0_1() {
    let mut r = Registers::new();
    r.f = 0b00000000;
    r.set_flag_z(1);
    let expected: u8 = 0b10000000;
    assert_eq!(r.f, expected);
}

#[test]
fn set_register_logic_1_1() {
    let mut r = Registers::new();
    r.f = 0b10000000;
    r.set_flag_z(1);
    let expected: u8 = 0b10000000;
    assert_eq!(r.f, expected);
}

#[test]
fn set_register_logic_1_0() {
    let mut r = Registers::new();
    r.f = 0b10000000;
    r.set_flag_z(0);
    let expected: u8 = 0;
    assert_eq!(r.f, expected);
}

#[test]
fn set_register_logic_0_0() {
    let mut r = Registers::new();
    r.f = 0b00000000;
    r.set_flag_z(0);
    let expected: u8 = 0;
    assert_eq!(r.f, expected);
}

#[test]
/// Test getting and setting of 
/// all 16 bit registers.
fn get_and_set_16bit_registers() {
    let mut r = Registers::new();
    let expected_a: u8 = 0xEA;
    let expected_b: u8 = 0x9E;
    let expected: u16 = 0xEA9E;

    // AF
    r.set_a(expected_a);
    r.set_f(expected_b);
    assert_eq!(r.get_af(), expected);

    // BC
    r.set_b(expected_a);
    r.set_c(expected_b);
    assert_eq!(r.get_bc(), expected);

    // DE
    r.set_d(expected_a);
    r.set_e(expected_b);
    assert_eq!(r.get_de(), expected);

    // HL
    r.set_h(expected_a);
    r.set_l(expected_b);
    assert_eq!(r.get_hl(), expected);

    // PC
    r.set_pc(expected);
    assert_eq!(r.get_pc(), expected);

    // SP
    r.set_sp(expected);
    assert_eq!(r.get_sp(), expected);
}
