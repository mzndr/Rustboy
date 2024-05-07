use super::Cpu;

impl Cpu {
    pub fn exec_cb_instruction(&mut self) -> u8 {
        let opcode = self.read_u8_at_pc_and_increase();
        let dst_idx = opcode & 0b1111_0000 >> 4;
        match opcode {
            0x06 => self.rlc_hl(),
            0x0e => self.rrc_hl(),
            0x46 => self.bit_hl(0),
            0x4e => self.bit_hl(1),
            0x56 => self.bit_hl(2),
            0x5e => self.bit_hl(3),
            0x66 => self.bit_hl(4),
            0x6e => self.bit_hl(5),
            0x76 => self.bit_hl(6),
            0x7e => self.bit_hl(7),

            0x00..=0x07 => self.rlc(dst_idx),
            0x08..=0x0F => self.rrc(dst_idx),
            0x10..=0x17 => self.rl(dst_idx),
            0x18..=0x1F => self.rr(dst_idx),
            0x40..=0x47 => self.bit(0, dst_idx),
            0x48..=0x4f => self.bit(1, dst_idx),
            0x50..=0x57 => self.bit(2, dst_idx),
            0x58..=0x5f => self.bit(3, dst_idx),
            0x60..=0x67 => self.bit(4, dst_idx),
            0x68..=0x6f => self.bit(5, dst_idx),
            0x70..=0x77 => self.bit(6, dst_idx),
            0x78..=0x7f => self.bit(7, dst_idx),

            _ => panic!("Unknown extended instruction: 0xCB{opcode:x}"),
        }
    }

    fn rrc_hl(&mut self) -> u8 {
        todo!("rrc (hl)")
    }

    fn rlc_hl(&mut self) -> u8 {
        todo!("rlc (hl)")
    }


    fn test_bit(&mut self, bit_idx: u8, val: u8) -> u8 {
        self.registers.set_flag_z(
            ((val >> bit_idx) & 1) == 1
        );
        self.registers.set_flag_h(true);
        self.registers.set_flag_n(false);
        3
    }

    fn bit_hl(&mut self, bit_idx: u8) -> u8 {
        let val = *self.read(self.registers.get_hl());
        self.test_bit(bit_idx, val);
        1
    }

    fn bit(&mut self, bit_idx: u8, register_idx: u8) -> u8 {
        self.test_bit(bit_idx, self.registers[register_idx]);
        1
    }
}
