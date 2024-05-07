use super::Cpu;

impl Cpu {
    pub fn exec_cb_instruction(&mut self) -> u8 {
        let opcode = self.read_u8_at_pc_and_increase();
        let dst_idx = opcode & 0b1111_0000 >> 4;
        let src_idx = opcode & 0b1111;
        match opcode {
            0x06 => self.rlc_hl(),
            0x0e => self.rrc_hl(),

            0x00..=0x07 => self.rlc(dst_idx),
            0x08..=0x0F => self.rrc(dst_idx),
            0x10..=0x17 => self.rl(dst_idx),
            0x18..=0x1F => self.rr(dst_idx),
            _ => panic!("Unknown extended instruction: 0xCB{opcode:x}"),
        }
    }

    fn rrc_hl(&mut self) -> u8 {
        todo!("rrc (hl)")
    }

    fn rlc_hl(&mut self) -> u8 {
        todo!("rlc (hl)")
    }
}
