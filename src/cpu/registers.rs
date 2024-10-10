use super::utils;
use std::{
    fmt,
    ops::{Index, IndexMut},
};

pub const FLAG_Z_INDEX: u8 = 0x07;
pub const FLAG_N_INDEX: u8 = 0x06;
pub const FLAG_H_INDEX: u8 = 0x05;
pub const FLAG_C_INDEX: u8 = 0x04;

pub const REGISTER_B_INDEX: u8 = 0;
pub const REGISTER_C_INDEX: u8 = 1;
pub const REGISTER_D_INDEX: u8 = 2;
pub const REGISTER_E_INDEX: u8 = 3;
pub const REGISTER_H_INDEX: u8 = 4;
pub const REGISTER_L_INDEX: u8 = 5;
pub const REGISTER_F_INDEX: u8 = 6;
pub const REGISTER_A_INDEX: u8 = 7;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Registers {
    // Maybe swap these out for an actual array to do better
    // combined register access.
    /// Accumulator.
    pub a: u8,
    /// Flags.
    pub f: u8,
    /// General purpose register.
    pub b: u8,
    /// General purpose register.
    pub c: u8,
    /// General purpose register.
    pub d: u8,
    /// General purpose register.
    pub e: u8,
    /// General purpose register.
    pub h: u8,
    /// General purpose register.
    pub l: u8,

    /// Stack pointer.
    pub sp: u16,
    /// Program counter.
    pub pc: u16,

    /// Interrupt master enable.
    pub ime: bool,
}

/// Impl index for registers to easilly decode
/// register access by opcode.
impl Index<u8> for Registers {
    type Output = u8;
    fn index(&self, index: u8) -> &Self::Output {
        match index % 8 {
            REGISTER_B_INDEX => &self.b,
            REGISTER_C_INDEX => &self.c,
            REGISTER_D_INDEX => &self.d,
            REGISTER_E_INDEX => &self.e,
            REGISTER_H_INDEX => &self.h,
            REGISTER_L_INDEX => &self.l,
            REGISTER_F_INDEX => &self.f,
            REGISTER_A_INDEX => &self.a,
            _ => panic!("invalid register at 0x{:x}", &index),
        }
    }
}

impl IndexMut<u8> for Registers {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        match index % 8 {
            REGISTER_B_INDEX => &mut self.b,
            REGISTER_C_INDEX => &mut self.c,
            REGISTER_D_INDEX => &mut self.d,
            REGISTER_E_INDEX => &mut self.e,
            REGISTER_H_INDEX => &mut self.h,
            REGISTER_L_INDEX => &mut self.l,
            REGISTER_F_INDEX => &mut self.f,
            REGISTER_A_INDEX => &mut self.a,
            _ => panic!("invalid register at 0x{:x}", &index),
        }
    }
}

/// Initial `PC` value
pub const PC_INIT_VAL: u16 = 0x0100;

impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: 0x01,
            f: 0xb0,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xd8,
            h: 0x01,
            l: 0x4d,
            sp: 0xfffe,
            pc: PC_INIT_VAL,

            ime: false,
        }
    }

    pub fn set_flag_at_index(&mut self, index: u8, val: bool) {
        self.f = utils::set_bit(self.f, index, val);
    }

    pub fn get_flag_at_index(&self, index: u8) -> bool {
        let mask: u8 = 1 << index;
        ((self.f & mask) >> index) == 1
    }
    pub fn get_af(&self) -> u16 {
        utils::merge_u8s(self.a, self.f)
    }

    pub fn set_af(&mut self, val: u16) {
        let split = utils::split_u16(val);
        self.a = split.0;
        self.f = split.1;
    }

    pub fn get_bc(&self) -> u16 {
        utils::merge_u8s(self.b, self.c)
    }

    pub fn set_bc(&mut self, val: u16) {
        let split = utils::split_u16(val);
        self.b = split.0;
        self.c = split.1;
    }

    /// Gets the de rgister.
    pub fn get_de(&self) -> u16 {
        utils::merge_u8s(self.d, self.e)
    }

    /// Sets the de rgister.
    pub fn set_de(&mut self, val: u16) {
        let split = utils::split_u16(val);
        self.d = split.0;
        self.e = split.1;
    }

    /// Gets the hl register.
    pub fn get_hl(&self) -> u16 {
        utils::merge_u8s(self.h, self.l)
    }

    /// Gets the sp register.
    pub fn set_hl(&mut self, val: u16) {
        let split = utils::split_u16(val);
        self.h = split.0;
        self.l = split.1;
    }

    /// Gets the sp register.
    pub fn get_sp(&self) -> u16 {
        self.sp
    }

    /// Sets the sp register.
    pub fn set_sp(&mut self, val: u16) {
        self.sp = val;
    }

    /// Gets the pc register.
    pub fn get_pc(&self) -> u16 {
        self.pc
    }

    /// Sets the pc register.
    pub fn set_pc(&mut self, val: u16) {
        self.pc = val;
    }

    /// FLAGS
    pub fn get_flag_z(&self) -> bool {
        self.get_flag_at_index(FLAG_Z_INDEX)
    }

    pub fn set_flag_z(&mut self, val: bool) {
        self.set_flag_at_index(FLAG_Z_INDEX, val);
    }

    pub fn get_flag_n(&self) -> bool {
        self.get_flag_at_index(FLAG_N_INDEX)
    }

    pub fn set_flag_n(&mut self, val: bool) {
        self.set_flag_at_index(FLAG_N_INDEX, val);
    }

    pub fn get_flag_h(&self) -> bool {
        self.get_flag_at_index(FLAG_H_INDEX)
    }

    pub fn set_flag_h(&mut self, val: bool) {
        self.set_flag_at_index(FLAG_H_INDEX, val);
    }

    pub fn get_flag_c(&self) -> bool {
        self.get_flag_at_index(FLAG_C_INDEX)
    }

    pub fn set_flag_c(&mut self, val: bool) {
        self.set_flag_at_index(FLAG_C_INDEX, val);
    }
}

impl fmt::Display for Registers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "AF: {:0>4x} BC: {:0>4x} DE: {:0>4x} HL: {:0>4x} SP: {:0>4x} PC: {:0>4x}",
            self.get_af(),
            self.get_bc(),
            self.get_de(),
            self.get_hl(),
            self.get_sp(),
            self.get_pc(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::registers::Registers;

    #[test]
    fn get_register_logic() {
        let mut r = Registers::new();
        r.f = 0b0010_0000;
        assert!(r.get_flag_at_index(5));
        r.f = 0b1101_1111;
        assert!(!r.get_flag_at_index(5));
    }

    #[test]
    fn set_register_logic_0_1() {
        let mut r = Registers::new();
        r.f = 0b0000_0000;
        r.set_flag_at_index(5, true);
        let expected: u8 = 0b0010_0000;
        assert_eq!(r.f, expected);
    }

    #[test]
    fn set_register_logic_1_1() {
        let mut r = Registers::new();
        r.f = 0b0010_0000;
        r.set_flag_at_index(5, true);
        let expected: u8 = 0b0010_0000;
        assert_eq!(r.f, expected);
    }

    #[test]
    fn set_register_logic_1_0() {
        let mut r = Registers::new();
        r.f = 0b1111_1111;
        r.set_flag_at_index(5, false);
        let expected: u8 = 0b1101_1111;
        assert_eq!(r.f, expected);
    }

    #[test]
    fn set_register_logic_0_0() {
        let mut r = Registers::new();
        r.f = 0b1101_1111;
        r.set_flag_at_index(5, false);
        let expected: u8 = 0b1101_1111;
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
        r.a = expected_a;
        r.f = expected_b;
        assert_eq!(r.get_af(), expected);

        // BC
        r.b = expected_a;
        r.c = expected_b;
        assert_eq!(r.get_bc(), expected);

        // DE
        r.d = expected_a;
        r.e = expected_b;
        assert_eq!(r.get_de(), expected);

        // HL
        r.h = expected_a;
        r.l = expected_b;
        assert_eq!(r.get_hl(), expected);

        // PC
        r.set_pc(expected);
        assert_eq!(r.get_pc(), expected);

        // SP
        r.set_sp(expected);
        assert_eq!(r.get_sp(), expected);
    }
}
