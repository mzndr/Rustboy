use super::utils;
use super::Cpu;

impl Cpu {
    /// Reads from wram at address.
    pub fn read(&self, address: u16) -> u8 {
        let u_addr = address as usize;
        self.check_address(address);
        return self.wram[u_addr];
    }

    /// Writes u8 to wram at address.
    pub fn write_u8(&mut self, address: u16, val: u8) {
        let u_addr = address as usize;
        self.check_address(address);
        self.wram[u_addr] = val;
    }

    /// Writes u16 to wram at address.
    pub fn write_u16(&mut self, address: u16, val: u16) {
        let split = utils::split_u16(val);
        self.write_u8(address, split.1);
        self.write_u8(address + 1, split.0);
    }

    /// Reads a byte from wram at pc and increases pc by one.
    pub fn read_u8_at_pc_and_increase(&mut self) -> u8 {
        let val = self.read(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        return val;
    }

    /// Reads two bytes from wram at pc and increases pc by two.
    pub fn read_u16_at_pc_and_increase(&mut self) -> u16 {
        let a = self.read_u8_at_pc_and_increase();
        let b = self.read_u8_at_pc_and_increase();

        // Little endian in memory
        return utils::merge_u8s(b, a);
    }

    /// Sets the bit specified in pos {0-7} of byte
    pub fn set_nth_bit(&mut self, pos: u8, byte: u8) -> u8 {
        if pos > 7 {
            // TODO: Throw error or something.
        }
        let mask: u8 = 0b1000000 >> pos;
        return byte | mask;
    }

    pub fn inc16(&mut self, val: u16) -> u16 {
        return val.wrapping_add(1);
    }

    pub fn dec16(&mut self, val: u16) -> u16 {
        return val.wrapping_sub(1);
    }

    pub fn inc8(&mut self, val: u8) -> u8 {
        let w = val.wrapping_add(1);
        self.registers.set_flag_h(w & 0xf == 0xf);
        self.registers.set_flag_z(w == 0);
        self.registers.set_flag_n(false);
        return w;
    }

    pub fn dec8(&mut self, val: u8) -> u8 {
        let w = val.wrapping_sub(1);
        self.registers.set_flag_h(w & 0xf == 0xf);
        self.registers.set_flag_z(w == 0);
        self.registers.set_flag_n(true);
        return w;
    }

    /// Adds a value with HL and stores it in HL.
    pub fn add16(&mut self, a: u16, b: u16) -> u16 {
        let h = (((a & 0xFFF) + (b & 0xFFF)) & 0x1000) == 0x1000;
        let c  = a as u32 + b as u32 > 0xFFFF;
        self.registers.set_flag_h(h);
        self.registers.set_flag_c(c);
        self.registers.set_flag_n(false);
        return a.wrapping_add(b);
    }

    /// Absolute jump by setting PC to address
    pub fn jp(&mut self, address: u16) {
        self.registers.set_pc(address);
    }
    
    /// Relative jump by adding val to PC
    pub fn jr(&mut self, val: u16) {
        self.registers.pc += val;
    }


    /// Xors value with a register and sets flags.
    pub fn xor(&mut self, val: u8) {
        self.registers.a ^= val;
        self.registers.set_flag_z(self.registers.a == 0);
        self.registers.set_flag_n(false);
        self.registers.set_flag_h(false);
        self.registers.set_flag_c(false);
    }
}

#[cfg(test)]
mod tests {
    // TODO: Add tests
}
