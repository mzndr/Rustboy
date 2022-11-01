use super::Cpu;
use super::utils;

impl Cpu {
    pub fn inc16(&mut self, val: u16) -> u16 {
        return val.wrapping_add(1);
    }

    pub fn dec16(&mut self, val: u16) -> u16 {
        return val.wrapping_add(1);
    }

    pub fn inc8(&mut self, val: u8) -> u8 {
        let w = val.wrapping_add(1);
        let hc = utils::has_8bit_half_carry(val, 1);
        self.registers.set_flag_h(hc as u8);
        self.registers.set_flag_z((w == 0) as u8);
        self.registers.set_flag_n(0);
        return w;
    }

    pub fn dec8(&mut self, val: u8) -> u8 {
        let w = val.wrapping_sub(1);

        // TODO: Sub half carry?

        self.registers.set_flag_h((w == 0xFF) as u8);
        self.registers.set_flag_z((w == 0) as u8);
        self.registers.set_flag_n(1);
        return w;
    }

    pub fn add16(&mut self, a: u16, b: u16) -> u16 {
        let hl = self.registers.get_hl();
        let bc = self.registers.get_bc();
        let res = hl.wrapping_add(bc);
        let hc = utils::has_16bit_half_carry(hl, bc);
        let carry = utils::has_16bit_carry(hl, bc);
        self.registers.set_flag_n(0);
        self.registers.set_flag_c(carry as u8);
        self.registers.set_flag_h(hc as u8);
        return res;
    }
}
