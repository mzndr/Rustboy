use super::Cpu;

impl Cpu {
    pub fn exec_cb_instruction(&mut self) -> u8 {
        let opcode = self.read_u8_at_pc_and_increase();
        match opcode {
            0xFE => self.set_7_hlp(),
            _ => panic!("Unknown extended instruction: 0xCB{opcode:x}"),
        }
    }

    /// OP-Code: `0xCBFE`
    /// Mnemonic: `SET 7, (HL)`
    pub fn set_7_hlp(&mut self) -> u8 {
        let hl = self.registers.get_hl();
        let val = *self.read(hl);
        let res = Self::set_nth_bit(7, val);
        self.write_u8(hl, res);
        3
    }
}
