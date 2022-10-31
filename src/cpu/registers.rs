use std::fmt;

use crate::helpers;
const FLAG_Z_INDEX: u8 = 7;
const FLAG_N_INDEX: u8 = 6;
const FLAG_H_INDEX: u8 = 5;
const FLAG_C_INDEX: u8 = 4;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Registers {
    pub a: u8,
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,

    pub sp: u16,
    pub pc: u16,
}

impl Registers {

    pub fn new() -> Registers {
        return Registers {
            a: 0x00,
            f: 0x00,
            b: 0x00,
            c: 0x00,
            d: 0x00,
            e: 0x00,
            h: 0x00,
            l: 0x00,
            sp: 0x000,
            pc: 0x100,
        };
    }
    pub fn set_flag_at_index(&mut self, index: u8, val: u8) {
        let mask: u8 = 1 << index;
        self.f &= !mask;
        self.f |= mask;
        self.f ^= mask;
        self.f |= val << index;

       }
    pub fn get_flag_at_index(&self, index: u8) -> u8 {
        let mask: u8 = 1 << index;
        return (self.f & mask) >> index;
    }
    pub fn get_af(&self) -> u16 {
        return helpers::merge_u8s(self.a, self.f);
    }

    pub fn set_af(&mut self, val: u16) {
        let split = helpers::split_u16(val);
        self.a = split.0;
        self.f = split.1;
    }

    pub fn get_bc(&self) -> u16 {
        return helpers::merge_u8s(self.b, self.c);
    }

    pub fn set_bc(&mut self, val: u16) {
        let split = helpers::split_u16(val);
        self.b = split.0;
        self.c = split.1;
    }

    // Gets the de rgister.
    pub fn get_de(&self) -> u16 {
        return helpers::merge_u8s(self.d, self.e);
    }

    // Sets the de rgister.
    pub fn set_de(&mut self, val: u16) {
        let split = helpers::split_u16(val);
        self.d = split.0;
        self.e = split.1;
    }

    /// Gets the hl register.
    pub fn get_hl(&self) -> u16 {
        return helpers::merge_u8s(self.h, self.l);
    }

    /// Gets the sp register.
    pub fn set_hl(&mut self, val: u16) {
        let split = helpers::split_u16(val);
        self.h = split.0;
        self.l = split.1;
    }

    /// Gets the sp register.
    pub fn get_sp(& self) -> u16 {
        return self.sp;
    }

    /// Sets the sp register.
    pub fn set_sp(&mut self, val: u16) {
        self.sp = val;
    }

    /// Gets the pc register.
    pub fn get_pc(&self) -> u16 {
        return self.pc;
    }

    /// Sets the pc register.
    pub fn set_pc(&mut self, val: u16) {
        self.pc = val;
    }

    /// FLAGS
    pub fn get_flag_z(&self) -> u8 {
        return self.get_flag_at_index(FLAG_Z_INDEX);
    }

    pub fn set_flag_z(&mut self, val: u8) {
        self.set_flag_at_index(FLAG_Z_INDEX, val);
    }

    pub fn get_flag_n(&self) -> u8 {
        return self.get_flag_at_index(FLAG_N_INDEX);
    }

    pub fn set_flag_n(&mut self, val: u8) {
        self.set_flag_at_index(FLAG_N_INDEX, val);
    }

    pub fn get_flag_h(&self) -> u8 {
        return self.get_flag_at_index(FLAG_H_INDEX);
    }

    pub fn set_flag_h(&mut self, val: u8) {
        self.set_flag_at_index(FLAG_H_INDEX, val);
    }

    pub fn get_flag_c(&self) -> u8 {
        return self.get_flag_at_index(FLAG_C_INDEX);
    }

    pub fn set_flag_c(&mut self, val: u8) {
        self.set_flag_at_index(FLAG_C_INDEX, val);
    }


}

impl fmt::Display for Registers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(
            f, 
            "AF: 0x{:x} BC: 0x{:x} DE: 0x{:x} HL: 0x{:x} SP: 0x{:x} PC: 0x{:x}", 
            self.get_af(),
            self.get_bc(),
            self.get_de(),
            self.get_hl(),
            self.get_sp(),
            self.get_pc(),
        );
    }
}
