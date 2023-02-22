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
    pub fn add16(&mut self, val: u16) {
        let hl = self.registers.get_hl();
        // TODO: Set flags
        self.registers.set_hl(hl.wrapping_add(val));
    }

    /// Jumps by setting PC to val
    pub fn jmp(&mut self, address: u16) {
        self.registers.set_pc(address);
    }
}

#[cfg(test)]
mod tests {
    // TODO: Add tests
}
