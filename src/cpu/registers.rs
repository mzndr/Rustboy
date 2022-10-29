use std::fmt;

use crate::helpers;
const FLAG_Z_MASK: u8 = 0b10000000;
const FLAG_N_MASK: u8 = 0b01000000;
const FLAG_H_MASK: u8 = 0b00100000;
const FLAG_C_MASK: u8 = 0b00010000;

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

    pub fn get_a(&mut self) -> u8 {
        return self.a;
    }
    pub fn set_a(&mut self, val: u8) {
        self.a = val;
    }

    pub fn get_f(&self) -> u8 {
        return self.f;
    }
    pub fn set_f(&mut self, val: u8) {
        self.f = val;
    }

    pub fn get_af(&self) -> u16 {
        return helpers::merge_u8s(self.a, self.f);
    }

    pub fn set_af(&mut self, val: u16) {
        let split = helpers::split_u16(val);
        self.a = split.0;
        self.f = split.1;
    }

    pub fn get_b(&self) -> u8 {
        return self.b;
    }
    pub fn set_b(&mut self, val: u8) {
        self.b = val;
    }

    pub fn get_c(&self) -> u8 {
        return self.c;
    }
    pub fn set_c(&mut self, val: u8) {
        self.c = val;
    }

    pub fn get_bc(&self) -> u16 {
        return helpers::merge_u8s(self.b, self.c);
    }

    pub fn set_bc(&mut self, val: u16) {
        let split = helpers::split_u16(val);
        self.b = split.0;
        self.c = split.1;
    }

    // Gets the d rgister.
    pub fn get_d(&self) -> u8 {
        return self.d;
    }
    // Sets the d rgister.
    pub fn set_d(&mut self, val: u8) {
        self.d = val;
    }

    // Gets the e rgister.
    pub fn get_e(&self) -> u8 {
        return self.e;
    }
    // Sets the e rgister.
    pub fn set_e(&mut self, val: u8) {
        self.e = val;
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

    /// Gets the h register.
    pub fn get_h(&self) -> u8 {
        return self.h;
    }
    
    /// Gets the h register.
    pub fn set_h(&mut self, val: u8) {
        self.h = val;
    }

    /// Gets the l register.
    pub fn get_l(&self) -> u8 {
        return self.l;
    }
    
    /// Sets the l register.
    pub fn set_l(&mut self, val: u8) {
        self.l = val;
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
        let mask: u8 = FLAG_Z_MASK;
        return (self.f & mask) >> 7;
    }

    pub fn set_flag_z(&mut self, val: u8) {
        self.f &= FLAG_Z_MASK;
        self.f |= FLAG_Z_MASK;
        self.f ^= FLAG_Z_MASK;
        self.f |= val << 7;
        
    }

    pub fn get_flag_n(&self) -> u8 {
        return (self.f & FLAG_N_MASK) >> 6;
    }

    pub fn set_flag_n(&mut self, val: u8) {
        self.f &= FLAG_N_MASK;
        self.f |= FLAG_N_MASK;
        self.f ^= FLAG_N_MASK;
        self.f |= val << 6;
    }

    pub fn get_flag_h(&self) -> u8 {
        return (self.f & FLAG_H_MASK) >> 5;
    }

    pub fn set_flag_h(&mut self, val: u8) {
        self.f &= FLAG_H_MASK;
        self.f |= FLAG_H_MASK;
        self.f ^= FLAG_H_MASK;
        self.f |= val << 5;
    }

    pub fn get_flag_c(&self) -> u8 {
        return (self.f & FLAG_C_MASK) >> 4;
    }

    pub fn set_flag_c(&mut self, val: u8) {
        self.f &= FLAG_C_MASK;
        self.f |= FLAG_C_MASK;
        self.f ^= FLAG_C_MASK;
        self.f |= val << 4;
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
