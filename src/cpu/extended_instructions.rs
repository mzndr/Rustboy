use super::{disassembler::decode_instruction, Cpu};

impl Cpu {
    #[tracing::instrument(name = "extended", target = "", skip(self), fields(c))]
    pub fn exec_cb_instruction(&mut self) -> u8 {
        let opcode = self.read_u8_at_pc_and_increase();
        let mnemonic = decode_instruction(opcode);
        let pc_mem = self.read_u16_at_pc();
        tracing::Span::current().record("c", format!("(PC):0x{pc_mem:0>4x} 0x{opcode:0>2x} 0x{opcode:0>2x}: {mnemonic}"));
        tracing::debug!("executing extended instruction");

        let dst_idx = opcode >> 4;
        match opcode {
            0x06 => self.rlc_hl(),
            0x0e => self.rrc_hl(),
            0x16 => self.rl_hl(),
            0x1e => self.rr_hl(),
            0x26 => self.sla_hl(),
            0x2e => self.sra_hl(),
            0x36 => self.sll_hl(),
            0x3e => self.srl_hl(),
            0x46 => self.bit_hl(0),
            0x4e => self.bit_hl(1),
            0x56 => self.bit_hl(2),
            0x5e => self.bit_hl(3),
            0x66 => self.bit_hl(4),
            0x6e => self.bit_hl(5),
            0x76 => self.bit_hl(6),
            0x7e => self.bit_hl(7),
            0x86 => self.res_hl(0),
            0x8e => self.res_hl(1),
            0x96 => self.res_hl(2),
            0x9e => self.res_hl(3),
            0xa6 => self.res_hl(4),
            0xae => self.res_hl(5),
            0xb6 => self.res_hl(6),
            0xbe => self.res_hl(7),
            0xc6 => self.set_hl(0),
            0xce => self.set_hl(1),
            0xd6 => self.set_hl(2),
            0xde => self.set_hl(3),
            0xe6 => self.set_hl(4),
            0xee => self.set_hl(5),
            0xf6 => self.set_hl(6),
            0xfe => self.set_hl(7),

            0x00..=0x07 => self.rlc(dst_idx),
            0x08..=0x0f => self.rrc(dst_idx),
            0x10..=0x17 => self.rl(dst_idx),
            0x18..=0x1f => self.rr(dst_idx),
            0x20..=0x27 => self.sla(dst_idx),
            0x28..=0x2f => self.sra(dst_idx),
            0x30..=0x37 => self.sll(dst_idx),
            0x38..=0x3f => self.srl(dst_idx),
            0x40..=0x47 => self.bit(0, dst_idx),
            0x48..=0x4f => self.bit(1, dst_idx),
            0x50..=0x57 => self.bit(2, dst_idx),
            0x58..=0x5f => self.bit(3, dst_idx),
            0x60..=0x67 => self.bit(4, dst_idx),
            0x68..=0x6f => self.bit(5, dst_idx),
            0x70..=0x77 => self.bit(6, dst_idx),
            0x78..=0x7f => self.bit(7, dst_idx),
            0x80..=0x87 => self.res(0, dst_idx),
            0x88..=0x8f => self.res(1, dst_idx),
            0x90..=0x97 => self.res(2, dst_idx),
            0x98..=0x9f => self.res(3, dst_idx),
            0xa0..=0xa7 => self.res(4, dst_idx),
            0xa8..=0xaf => self.res(5, dst_idx),
            0xb0..=0xb7 => self.res(6, dst_idx),
            0xb8..=0xbf => self.res(7, dst_idx),
            0xc0..=0xc7 => self.set(0, dst_idx),
            0xc8..=0xcf => self.set(1, dst_idx),
            0xd0..=0xd7 => self.set(2, dst_idx),
            0xd8..=0xdf => self.set(3, dst_idx),
            0xe0..=0xe7 => self.set(4, dst_idx),
            0xe8..=0xef => self.set(5, dst_idx),
            0xf0..=0xf7 => self.set(6, dst_idx),
            0xf8..=0xff => self.set(7, dst_idx),
        }
    }

    fn set_bit(val: u8, idx: u8) -> u8 {
        let bit = 1 << idx;
        // write 1 to the bit, xor with 1 to set it to 0
        val | bit
    }

    fn set(&mut self, bit_idx: u8, register_idx: u8) -> u8 {
        self.registers[register_idx] = Self::set_bit(self.registers[register_idx], bit_idx);
        1
    }

    fn set_hl(&mut self, bit_idx: u8) -> u8 {
        let address = self.registers.get_hl();
        let val = *self.read(address);
        let result = Self::set_bit(val, bit_idx);
        self.write_u8(address, result);
        1
    }

    fn reset_bit(val: u8, idx: u8) -> u8 {
        let bit = 1 << idx;
        // write 1 to the bit, xor with 1 to set it to 0
        (val | bit) ^ bit
    }

    fn res(&mut self, bit_idx: u8, register_idx: u8) -> u8 {
        self.registers[register_idx] = Self::reset_bit(self.registers[register_idx], bit_idx);
        1
    }

    fn res_hl(&mut self, bit_idx: u8) -> u8 {
        let address = self.registers.get_hl();
        let val = *self.read(address);
        let result = Self::reset_bit(val, bit_idx);
        self.write_u8(address, result);
        1
    }

    fn sll_val(&mut self, val: u8) -> u8 {
        let result = val << 1;
        self.registers.set_flag_c(val >> 7 == 1);
        self.registers.set_flag_z(result == 0);
        self.registers.set_flag_h(false);
        self.registers.set_flag_n(false);
        result
    }

    fn sll(&mut self, register_idx: u8) -> u8 {
        self.registers[register_idx] = self.sll_val(self.registers[register_idx]);
        1
    }

    fn sll_hl(&mut self) -> u8 {
        let address = self.registers.get_hl();
        let result = self.sll_val(*self.read(address));
        self.write_u8(address, result);
        2
    }

    fn srl_val(&mut self, val: u8) -> u8 {
        let result = val >> 1;
        self.registers.set_flag_c(val & 1 == 1);
        self.registers.set_flag_z(result == 0);
        self.registers.set_flag_h(false);
        self.registers.set_flag_n(false);
        result
    }

    fn srl(&mut self, register_idx: u8) -> u8 {
        self.registers[register_idx] = self.srl_val(self.registers[register_idx]);
        1
    }

    fn srl_hl(&mut self) -> u8 {
        let address = self.registers.get_hl();
        let result = self.srl_val(*self.read(address));
        self.write_u8(address, result);
        2
    }

    fn sla_val(&mut self, val: u8) -> u8 {
        let result = val << 1;
        self.registers.set_flag_h(false);
        self.registers.set_flag_n(false);
        self.registers.set_flag_c((val >> 7) == 1);
        self.registers.set_flag_z(result == 0);
        result
    }

    fn sla(&mut self, register_idx: u8) -> u8 {
        self.registers[register_idx] = self.sla_val(self.registers[register_idx]);
        1
    }

    fn sla_hl(&mut self) -> u8 {
        let address = self.registers.get_hl();
        let result = self.sla_val(*self.read(address));
        self.write_u8(address, result);
        2
    }

    fn sra_val(&mut self, val: u8) -> u8 {
        self.registers.set_flag_h(false);
        self.registers.set_flag_n(false);
        self.registers.set_flag_c((val & 1) == 1);
        let result = (val & 0b1000_0000) | (val >> 1);
        self.registers.set_flag_z(result == 0);
        result
    }

    fn sra(&mut self, register_idx: u8) -> u8 {
        self.registers[register_idx] = self.sra_val(self.registers[register_idx]);
        1
    }

    fn sra_hl(&mut self) -> u8 {
        let address = self.registers.get_hl();
        let result = self.sra_val(*self.read(address));
        self.write_u8(address, result);
        2
    }

    fn rr_hl(&mut self) -> u8 {
        let address = self.registers.get_hl();
        let result = self.rr_val(*self.read(address));
        self.write_u8(address, result);
        2
    }

    fn rl_hl(&mut self) -> u8 {
        let address = self.registers.get_hl();
        let result = self.rl_val(*self.read(address));
        self.write_u8(address, result);
        2
    }

    fn rrc_hl(&mut self) -> u8 {
        let address = self.registers.get_hl();
        let result = self.rrc_val(*self.read(address));
        self.write_u8(address, result);
        2
    }

    fn rlc_hl(&mut self) -> u8 {
        let address = self.registers.get_hl();
        let result = self.rlc_val(*self.read(address));
        self.write_u8(address, result);
        2
    }

    fn test_bit(&mut self, bit_idx: u8, val: u8) -> u8 {
        self.registers.set_flag_z(((val >> bit_idx) & 1) == 1);
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
