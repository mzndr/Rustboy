use super::Cpu;

impl Cpu {
    /// XXX dst, src
    #[allow(clippy::too_many_lines)]
    pub fn exec_instruction(&mut self, opcode: u8) -> u8 {
        let dst_idx = opcode & 0b1111_0000 >> 4;
        let src_idx = opcode & 0b1111;

        match opcode {
            0x00 => Self::nop(),
            0x03 => self.inc_bc(),
            0x04 => self.inc_b(),
            0x05 => self.dec_b(),
            0x07 => self.rlca(),
            0x09 => self.add_hl_bc(),
            0x0b => self.dec_bc(),
            0x0c => self.inc_c(),
            0x0d => self.dec_c(),
            0x0f => self.rrca(),
            0x13 => self.inc_de(),
            0x14 => self.inc_d(),
            0x15 => self.dec_d(),
            0x17 => self.rla(),
            0x18 => self.jr_r8(),
            0x19 => self.add_hl_de(),
            0x1b => self.dec_de(),
            0x1c => self.inc_e(),
            0x1d => self.dec_e(),
            0x1f => self.rra(),
            0x20 => self.jr_nz_r8(),
            0x23 => self.inc_hl(),
            0x24 => self.inc_h(),
            0x25 => self.dec_h(),
            0x29 => self.add_hl_hl(),
            0x2b => self.dec_hl(),
            0x2c => self.inc_l(),
            0x2d => self.dec_l(),
            0x33 => self.inc_sp(),
            0x34 => self.inc_hlp(),
            0x35 => self.dec_hlp(),
            0x39 => self.add_hl_sp(),
            0x3b => self.dec_sp(),
            0x3c => self.inc_a(),
            0x3d => self.dec_a(),

            0x02 => {
                Self::ld(self.registers.a, self.read_mut(self.registers.get_bc()));
                2
            }
            0x12 => {
                Self::ld(self.registers.a, self.read_mut(self.registers.get_de()));
                2
            }
            0x22 => {
                let hl = self.registers.get_hl();
                Self::ld(self.registers.a, self.read_mut(hl));
                self.registers.set_hl(hl.wrapping_add(1));
                2
            }
            0x32 => {
                let hl = self.registers.get_hl();
                Self::ld(self.registers.a, self.read_mut(hl));
                self.registers.set_hl(hl.wrapping_sub(1));
                2
            }

            0x06 => {
                Self::ld(self.read_u8_at_pc_and_increase(), &mut self.registers.b);
                2
            }
            0x16 => {
                Self::ld(self.read_u8_at_pc_and_increase(), &mut self.registers.d);
                2
            }
            0x26 => {
                Self::ld(self.read_u8_at_pc_and_increase(), &mut self.registers.h);
                2
            }
            0x36 => {
                Self::ld(
                    self.read_u8_at_pc_and_increase(),
                    self.read_mut(self.registers.get_hl()),
                );
                3
            }

            0x0A => {
                Self::ld(*self.read(self.registers.get_bc()), &mut self.registers.a);
                2
            }
            0x1A => {
                Self::ld(*self.read(self.registers.get_de()), &mut self.registers.a);
                2
            }
            0x2A => {
                let hl = self.registers.get_hl();
                Self::ld(*self.read(hl), &mut self.registers.a);
                self.registers.set_hl(hl.wrapping_add(1));
                2
            }
            0x3A => {
                let hl = self.registers.get_hl();
                Self::ld(*self.read(hl), &mut self.registers.a);
                self.registers.set_hl(hl.wrapping_sub(1));
                2
            }

            0x0E => {
                Self::ld(self.read_u8_at_pc_and_increase(), &mut self.registers.c);
                2
            }
            0x1E => {
                Self::ld(self.read_u8_at_pc_and_increase(), &mut self.registers.e);
                2
            }
            0x2E => {
                Self::ld(self.read_u8_at_pc_and_increase(), &mut self.registers.l);
                2
            }
            0x3E => {
                Self::ld(self.read_u8_at_pc_and_increase(), &mut self.registers.a);
                2
            }

            0x4E | 0x5E | 0x6E | 0x7E | 0x46 | 0x56 | 0x66 => {
                Self::ld(
                    *self.read(self.registers.get_hl()),
                    &mut self.registers[dst_idx],
                );
                2
            }
            0x76 => todo!("HLT"),
            0x70..=0x77 => {
                Self::ld(
                    self.registers[dst_idx],
                    self.read_mut(self.registers.get_hl()),
                );
                2
            }
            0x40..=0x7f => Self::ld(self.registers[src_idx], &mut self.registers[dst_idx]),

            0x80 => self.add_a_b(),
            0x81 => self.add_a_c(),
            0x82 => self.add_a_d(),
            0x83 => self.add_a_e(),
            0x84 => self.add_a_h(),
            0x85 => self.add_a_l(),
            0x86 => self.add_a_hlp(),
            0x87 => self.add_a_a(),
            0x88 => self.adc_a_b(),
            0x89 => self.adc_a_c(),
            0x8a => self.adc_a_d(),
            0x8b => self.adc_a_e(),
            0x8c => self.adc_a_h(),
            0x8d => self.adc_a_l(),
            0x8e => self.adc_a_hlp(),
            0x8f => self.adc_a_a(),
            0x90 => self.sub_a_b(),
            0x91 => self.sub_a_c(),
            0x92 => self.sub_a_d(),
            0x93 => self.sub_a_e(),
            0x94 => self.sub_a_h(),
            0x95 => self.sub_a_l(),
            0x96 => self.sub_a_hlp(),
            0x97 => self.sub_a_a(),
            0x98 => self.sbc_a_b(),
            0x99 => self.sbc_a_c(),
            0x9a => self.sbc_a_d(),
            0x9b => self.sbc_a_e(),
            0x9c => self.sbc_a_h(),
            0x9d => self.sbc_a_l(),
            0x9e => self.sbc_a_hlp(),
            0x9f => self.sbc_a_a(),
            0xC6 => self.add_a_u8(),
            0xD6 => self.sub_a_u8(),
            0xE6 => self.and_u8(),
            0xF6 => self.or_u8(),
            0xa0 => self.and_b(),
            0xa1 => self.and_c(),
            0xa2 => self.and_d(),
            0xa3 => self.and_e(),
            0xa4 => self.and_h(),
            0xa5 => self.and_l(),
            0xa6 => self.and_hlp(),
            0xa7 => self.and_a(),
            0xa8 => self.xor_b(),
            0xa9 => self.xor_c(),
            0xaa => self.xor_d(),
            0xab => self.xor_e(),
            0xac => self.xor_h(),
            0xad => self.xor_l(),
            0xae => self.xor_hlp(),
            0xaf => self.xor_a(),
            0xb0 => self.or_b(),
            0xb1 => self.or_c(),
            0xb2 => self.or_d(),
            0xb3 => self.or_e(),
            0xb4 => self.or_h(),
            0xb5 => self.or_l(),
            0xb6 => self.or_hlp(),
            0xb7 => self.or_a(),
            0xb8 => self.cp_b(),
            0xb9 => self.cp_c(),
            0xba => self.cp_d(),
            0xbb => self.cp_e(),
            0xbc => self.cp_h(),
            0xbd => self.cp_l(),
            0xbe => self.cp_hlp(),
            0xbf => self.cp_a(),
            0xc1 => self.pop_bc(),
            0xc3 => self.jp_a16(),
            0xc5 => self.push_bc(),
            0xcb => self.exec_cb_instruction(),
            0xce => self.adc_a_u8(),
            0xcf => self.rst_08h(),
            0xd1 => self.pop_de(),
            0xd5 => self.push_de(),
            0xde => self.sbc_a_u8(),
            0xdf => self.rst_18h(),
            0xe1 => self.pop_hl(),
            0xe5 => self.push_hl(),
            0xee => self.xor_u8(),
            0xef => self.rst_28h(),
            0xf1 => self.pop_af(),
            0xf3 => self.di(),
            0xf5 => self.push_af(),
            0xfa => self.ei(),
            0xfe => self.cp_u8(),
            0xff => self.rst_38h(),
            _ => panic!("Unknown instruction: 0x{opcode:x}"),
        }
    }

    /// OP-Code: `0x00`
    /// Mnemonic: `NOP`
    pub fn nop() -> u8 {
        1
    }

    /// OP-Code: `0x03`
    /// Mnemonic: `INC BC`
    pub fn inc_bc(&mut self) -> u8 {
        let r = self.registers.get_bc();
        let res = Self::inc16(r);
        self.registers.set_bc(res);
        2
    }

    /// OP-Code: `0x0b`
    /// Mnemonic: `DEC BC`
    pub fn dec_bc(&mut self) -> u8 {
        let r = self.registers.get_bc();
        let res = Self::dec16(r);
        self.registers.set_bc(res);
        2
    }

    /// OP-Code: `0x1b`
    /// Mnemonic: `DEC DE`
    pub fn dec_de(&mut self) -> u8 {
        let r = self.registers.get_de();
        let res = Self::dec16(r);
        self.registers.set_de(res);
        2
    }

    /// OP-Code: `0x2b`
    /// Mnemonic: `DEC HL`
    pub fn dec_hl(&mut self) -> u8 {
        let r = self.registers.get_hl();
        let res = Self::dec16(r);
        self.registers.set_hl(res);
        2
    }

    /// OP-Code: `0x3b`
    /// Mnemonic: `DEC SP`
    pub fn dec_sp(&mut self) -> u8 {
        let r = self.registers.get_sp();
        let res = Self::dec16(r);
        self.registers.set_sp(res);
        2
    }

    /// OP-Code: `0x04`
    /// Mnemonic: `INC B`
    pub fn inc_b(&mut self) -> u8 {
        self.registers.b = self.inc8(self.registers.b);
        1
    }

    /// OP-Code: `0x05`
    /// Mnemonic: `DEC B`
    pub fn dec_b(&mut self) -> u8 {
        self.registers.b = self.dec8(self.registers.b);
        1
    }

    /// OP-Code: `0x09`
    /// Mnemonic: `ADD HL, BC`
    pub fn add_hl_bc(&mut self) -> u8 {
        let bc = self.registers.get_bc();
        self.add16(bc);
        2
    }

    /// OP-Code: `0x13`
    /// Mnemonic: `INC DE`
    pub fn inc_de(&mut self) -> u8 {
        let r = self.registers.get_de();
        let res = Self::inc16(r);
        self.registers.set_de(res);
        2
    }

    /// OP-Code: `0x14`
    /// Mnemonic: `INC D`
    pub fn inc_d(&mut self) -> u8 {
        self.registers.d = self.inc8(self.registers.d);
        2
    }

    /// OP-Code: `0x15`
    /// Mnemonic: `DEC D`
    pub fn dec_d(&mut self) -> u8 {
        self.registers.d = self.dec8(self.registers.d);
        2
    }

    /// OP-Code: `0x18`
    /// Mnemonic: `JR r8`
    pub fn jr_r8(&mut self) -> u8 {
        let val = self.read_u8_at_pc_and_increase();
        self.jr(val);
        3
    }

    /// OP-Code: `0x19`
    /// Mnemonic: `ADD HL, DE`
    pub fn add_hl_de(&mut self) -> u8 {
        let de = self.registers.get_de();
        self.add16(de);
        2
    }

    /// OP-Code: `0x0D`
    /// Mnemonic: `DEC C`
    pub fn dec_c(&mut self) -> u8 {
        self.registers.c = self.dec8(self.registers.c);
        1
    }

    /// OP-Code: `0x1D`
    /// Mnemonic: `DEC E`
    pub fn dec_e(&mut self) -> u8 {
        self.registers.e = self.dec8(self.registers.e);
        1
    }

    /// OP-Code: `0x1F`
    /// Mnemonic: `RRA`
    pub fn rra(&mut self) -> u8 {
        //todo: do this properly.
        tracing::warn!("do this properly");

        let a = self.registers.a;
        self.registers.set_flag_c(((a & 0b1000_0000) >> 7) == 1);
        self.registers.a = a.rotate_right(1);
        self.registers.set_flag_z(false);
        self.registers.set_flag_n(false);
        self.registers.set_flag_h(false);
        1
    }

    /// OP-Code: `0x07`
    /// Mnemonic: `RLA`
    pub fn rla(&mut self) -> u8 {
        //todo: do this properly.
        tracing::warn!("do this properly");

        let a = self.registers.a;
        self.registers.set_flag_c(((a & 0b1000_0000) << 7) == 1);
        self.registers.a = a.rotate_left(1);
        self.registers.set_flag_z(false);
        self.registers.set_flag_n(false);
        self.registers.set_flag_h(false);
        1
    }

    /// OP-Code: `0x07`
    /// Mnemonic: `RLCA`
    pub fn rlca(&mut self) -> u8 {
        //todo: do this properly.
        tracing::warn!("do this properly");

        let a = self.registers.a;
        self.registers.set_flag_c(((a & 0b1000_0000) >> 7) == 1);
        self.registers.a = a.rotate_left(1);
        self.registers.set_flag_z(self.registers.a == 0);
        self.registers.set_flag_n(false);
        self.registers.set_flag_h(false);
        self.registers.set_flag_c(false);
        1
    }

    /// OP-Code: `0x0F`
    /// Mnemonic: `RRCA`
    pub fn rrca(&mut self) -> u8 {
        //todo: do this properly.
        tracing::warn!("do this properly");

        let a = self.registers.a;
        self.registers.set_flag_c(((a & 0b1000_0000) << 7) == 1);
        self.registers.a = a.rotate_right(1);
        self.registers.set_flag_z(self.registers.a == 0);
        self.registers.set_flag_n(false);
        self.registers.set_flag_h(false);
        self.registers.set_flag_c(false);
        1
    }

    /// OP-Code: `0x24`
    /// Mnemonic: `INC H`
    pub fn inc_h(&mut self) -> u8 {
        self.registers.h = self.inc8(self.registers.h);
        1
    }

    /// OP-Code: `0x25`
    /// Mnemonic: `DEC H`
    pub fn dec_h(&mut self) -> u8 {
        self.registers.h = self.dec8(self.registers.h);
        1
    }

    /// OP-Code: `0x2D`
    /// Mnemonic: `DEC L`
    pub fn dec_l(&mut self) -> u8 {
        self.registers.h = self.dec8(self.registers.l);
        1
    }

    /// OP-Code: `0x3D`
    /// Mnemonic: `DEC A`
    pub fn dec_a(&mut self) -> u8 {
        self.registers.a = self.dec8(self.registers.a);
        1
    }

    /// OP-Code: `0x34`
    /// Mnemonic: `INC (HL)`
    pub fn inc_hlp(&mut self) -> u8 {
        let address = self.registers.get_hl();
        let val = self.read(address);
        let res = self.inc8(*val);
        self.write_u8(address, res);
        3
    }

    /// OP-Code: `0x35`
    /// Mnemonic: `DEC (HL)`
    pub fn dec_hlp(&mut self) -> u8 {
        let address = self.registers.get_hl();
        let val = self.read(address);
        let res = self.dec8(*val);
        self.write_u8(address, res);
        3
    }

    /// OP-Code: `0x20`
    /// Mnemonic: `JR NZ, r8`
    pub fn jr_nz_r8(&mut self) -> u8 {
        let val = self.read_u8_at_pc_and_increase();
        if !self.registers.get_flag_z() {
            self.jr(val);
            return 3;
        }
        2
    }

    /// OP-Code: `0x23`
    /// Mnemonic: `INC HL`
    pub fn inc_hl(&mut self) -> u8 {
        let r = self.registers.get_hl();
        let res = Self::inc16(r);
        self.registers.set_hl(res);
        2
    }

    /// OP-Code: `0x29`
    /// Mnemonic: `ADD HL, HL`
    pub fn add_hl_hl(&mut self) -> u8 {
        let hl = self.registers.get_hl();
        self.add16(hl);
        2
    }

    /// OP-Code: `0x39`
    /// Mnemonic: `ADD HL, SP`
    pub fn add_hl_sp(&mut self) -> u8 {
        let hl = self.registers.get_sp();
        self.add16(hl);
        2
    }

    /// OP-Code: `0x0C`
    /// Mnemonic: `INC C`
    pub fn inc_c(&mut self) -> u8 {
        self.registers.c = self.inc8(self.registers.c);
        1
    }

    /// OP-Code: `0x1C`
    /// Mnemonic: `INC E`
    pub fn inc_e(&mut self) -> u8 {
        self.registers.e = self.inc8(self.registers.e);
        1
    }

    /// OP-Code: `0x2C`
    /// Mnemonic: `INC L`
    pub fn inc_l(&mut self) -> u8 {
        self.registers.l = self.inc8(self.registers.l);
        1
    }

    /// OP-Code: `0x3C`
    /// Mnemonic: `INC A`
    pub fn inc_a(&mut self) -> u8 {
        self.registers.a = self.inc8(self.registers.a);
        1
    }

    /// OP-Code: `0x33`
    /// Mnemonic: `INC Sd16P`
    pub fn inc_sp(&mut self) -> u8 {
        let r = self.registers.get_sp();
        let res = Self::inc16(r);
        self.registers.set_sp(res);
        2
    }

    /// OP-Code: `0xA8`
    /// Mnemonic: `AND B`
    pub fn and_b(&mut self) -> u8 {
        self.and(self.registers.b);
        1
    }

    /// OP-Code: `0xA9`
    /// Mnemonic: `AND C`
    pub fn and_c(&mut self) -> u8 {
        self.and(self.registers.c);
        1
    }

    /// OP-Code: `0xAA`
    /// Mnemonic: `AND D`
    pub fn and_d(&mut self) -> u8 {
        self.and(self.registers.d);
        1
    }

    /// OP-Code: `0xAB`
    /// Mnemonic: `AND E`
    pub fn and_e(&mut self) -> u8 {
        self.and(self.registers.e);
        1
    }

    /// OP-Code: `0xAC`
    /// Mnemonic: `AND H`
    pub fn and_h(&mut self) -> u8 {
        self.and(self.registers.h);
        1
    }

    /// OP-Code: `0xAD`
    /// Mnemonic: `AND L`
    pub fn and_l(&mut self) -> u8 {
        self.and(self.registers.l);
        1
    }

    /// OP-Code: `0xAE`
    /// Mnemonic: `AND (HL)`
    pub fn and_hlp(&mut self) -> u8 {
        self.and(*self.read(self.registers.get_hl()));
        2
    }

    /// OP-Code: `0xAF`
    /// Mnemonic: `AND A`
    pub fn and_a(&mut self) -> u8 {
        self.and(self.registers.a);
        1
    }

    /// OP-Code: `0xE6`
    /// Mnemonic: `AND u8`
    pub fn and_u8(&mut self) -> u8 {
        let val = self.read_u8_at_pc_and_increase();
        self.and(val);
        2
    }

    /// OP-Code: `0xA8`
    /// Mnemonic: `XOR B`
    pub fn xor_b(&mut self) -> u8 {
        self.xor(self.registers.b);
        1
    }

    /// OP-Code: `0xA9`
    /// Mnemonic: `XOR C`
    pub fn xor_c(&mut self) -> u8 {
        self.xor(self.registers.c);
        1
    }

    /// OP-Code: `0xAA`
    /// Mnemonic: `XOR D`
    pub fn xor_d(&mut self) -> u8 {
        self.xor(self.registers.d);
        1
    }

    /// OP-Code: `0xAB`
    /// Mnemonic: `XOR E`
    pub fn xor_e(&mut self) -> u8 {
        self.xor(self.registers.e);
        1
    }

    /// OP-Code: `0xAC`
    /// Mnemonic: `XOR H`
    pub fn xor_h(&mut self) -> u8 {
        self.xor(self.registers.h);
        1
    }

    /// OP-Code: `0xAD`
    /// Mnemonic: `XOR L`
    pub fn xor_l(&mut self) -> u8 {
        self.xor(self.registers.l);
        1
    }

    /// OP-Code: `0xAE`
    /// Mnemonic: `XOR (HL)`
    pub fn xor_hlp(&mut self) -> u8 {
        self.xor(*self.read(self.registers.get_hl()));
        2
    }

    /// OP-Code: `0xAF`
    /// Mnemonic: `XOR A`
    pub fn xor_a(&mut self) -> u8 {
        self.xor(self.registers.a);
        1
    }

    /// OP-Code: `0xEE`
    /// Mnemonic: `XOR u8`
    pub fn xor_u8(&mut self) -> u8 {
        let val = self.read_u8_at_pc_and_increase();
        self.xor(val);
        2
    }

    /// OP-Code: `0xB0`
    /// Mnemonic: `OR B`
    pub fn or_b(&mut self) -> u8 {
        let val = self.registers.b;
        self.or(val);
        1
    }

    /// OP-Code: `0xB1`
    /// Mnemonic: `OR C`
    pub fn or_c(&mut self) -> u8 {
        self.or(self.registers.c);
        1
    }

    /// OP-Code: `0xB2`
    /// Mnemonic: `OR D`
    pub fn or_d(&mut self) -> u8 {
        self.or(self.registers.d);
        1
    }

    /// OP-Code: `0xB3`
    /// Mnemonic: `OR E`
    pub fn or_e(&mut self) -> u8 {
        self.or(self.registers.e);
        1
    }

    /// OP-Code: `0xB4`
    /// Mnemonic: `OR H`
    pub fn or_h(&mut self) -> u8 {
        self.or(self.registers.h);
        1
    }

    /// OP-Code: `0xB5`
    /// Mnemonic: `OR L`
    pub fn or_l(&mut self) -> u8 {
        self.or(self.registers.l);
        1
    }

    /// OP-Code: `0xB6`
    /// Mnemonic: `OR (HL)`
    pub fn or_hlp(&mut self) -> u8 {
        self.or(*self.read(self.registers.get_hl()));
        2
    }

    /// OP-Code: `0xB7`
    /// Mnemonic: `OR A`
    pub fn or_a(&mut self) -> u8 {
        self.or(self.registers.a);
        1
    }

    /// OP-Code: `0xF6`
    /// Mnemonic: `OR u8`
    pub fn or_u8(&mut self) -> u8 {
        let val = self.read_u8_at_pc_and_increase();
        self.or(val);
        2
    }

    /// OP-Code: `0xB8`
    /// Mnemonic: `CP B`
    pub fn cp_b(&mut self) -> u8 {
        self.cp(self.registers.b);
        1
    }

    /// OP-Code: `0xB9`
    /// Mnemonic: `CP C`
    pub fn cp_c(&mut self) -> u8 {
        self.cp(self.registers.c);
        1
    }

    /// OP-Code: `0xBA`
    /// Mnemonic: `CP D`
    pub fn cp_d(&mut self) -> u8 {
        self.cp(self.registers.d);
        1
    }

    /// OP-Code: `0xBB`
    /// Mnemonic: `CP E`
    pub fn cp_e(&mut self) -> u8 {
        self.cp(self.registers.e);
        1
    }

    /// OP-Code: `0xBC`
    /// Mnemonic: `CP H`
    pub fn cp_h(&mut self) -> u8 {
        self.cp(self.registers.h);
        1
    }

    /// OP-Code: `0xBD`
    /// Mnemonic: `CP L`
    pub fn cp_l(&mut self) -> u8 {
        self.cp(self.registers.l);
        1
    }

    /// OP-Code: `0xBE`
    /// Mnemonic: `CP (HL)`
    pub fn cp_hlp(&mut self) -> u8 {
        let address = self.registers.get_hl();
        self.cp(*self.read(address));
        2
    }

    /// OP-Code: `0xBF`
    /// Mnemonic: `CP A`
    pub fn cp_a(&mut self) -> u8 {
        self.cp(self.registers.a);
        1
    }

    /// OP-Code: `0xFE`
    /// Mnemonic: `CP u8`
    pub fn cp_u8(&mut self) -> u8 {
        let val = self.read_u8_at_pc_and_increase();
        self.cp(val);
        2
    }

    /// OP-Code: `0x80`
    /// Mnemonic: `ADD A B`
    pub fn add_a_b(&mut self) -> u8 {
        self.add8(self.registers.b);
        1
    }

    /// OP-Code: `0x81`
    /// Mnemonic: `ADD A C`
    pub fn add_a_c(&mut self) -> u8 {
        self.add8(self.registers.c);
        1
    }

    /// OP-Code: `0x82`
    /// Mnemonic: `ADD A D`
    pub fn add_a_d(&mut self) -> u8 {
        self.add8(self.registers.d);
        1
    }

    /// OP-Code: `0x83`
    /// Mnemonic: `ADD A E`
    pub fn add_a_e(&mut self) -> u8 {
        self.add8(self.registers.e);
        1
    }

    /// OP-Code: `0x84`
    /// Mnemonic: `ADD A H`
    pub fn add_a_h(&mut self) -> u8 {
        self.add8(self.registers.h);
        1
    }

    /// OP-Code: `0x85`
    /// Mnemonic: `ADD A L`
    pub fn add_a_l(&mut self) -> u8 {
        self.add8(self.registers.l);
        1
    }

    /// OP-Code: `0x86`
    /// Mnemonic: `ADD A (HL)`
    pub fn add_a_hlp(&mut self) -> u8 {
        let address = self.registers.get_hl();
        self.add8(*self.read(address));
        2
    }

    /// OP-Code: `0x87`
    /// Mnemonic: `ADD A A`
    pub fn add_a_a(&mut self) -> u8 {
        self.add8(self.registers.a);
        1
    }

    /// OP-Code: `0xC6`
    /// Mnemonic: `ADD A A`
    pub fn add_a_u8(&mut self) -> u8 {
        self.add8(self.registers.a);
        2
    }

    /// OP-Code: `0x88`
    /// Mnemonic: `ADC A B`
    pub fn adc_a_b(&mut self) -> u8 {
        self.add8c(self.registers.b);
        1
    }

    /// OP-Code: `0x89`
    /// Mnemonic: `ADC A C`
    pub fn adc_a_c(&mut self) -> u8 {
        self.add8c(self.registers.c);
        1
    }

    /// OP-Code: `0x8A`
    /// Mnemonic: `ADC A D`
    pub fn adc_a_d(&mut self) -> u8 {
        self.add8c(self.registers.d);
        1
    }

    /// OP-Code: `0x8B`
    /// Mnemonic: `ADC A E`
    pub fn adc_a_e(&mut self) -> u8 {
        self.add8c(self.registers.e);
        1
    }

    /// OP-Code: `0x8C`
    /// Mnemonic: `ADC A H`
    pub fn adc_a_h(&mut self) -> u8 {
        self.add8c(self.registers.h);
        1
    }

    /// OP-Code: `0x8D`
    /// Mnemonic: `ADC A L`
    pub fn adc_a_l(&mut self) -> u8 {
        self.add8c(self.registers.l);
        1
    }

    /// OP-Code: `0x8E`
    /// Mnemonic: `ADC A L`
    pub fn adc_a_hlp(&mut self) -> u8 {
        let address = self.registers.get_hl();
        self.add8c(*self.read(address));
        2
    }

    /// OP-Code: `0x8F`
    /// Mnemonic: `ADC A A`
    pub fn adc_a_a(&mut self) -> u8 {
        self.add8c(self.registers.a);
        1
    }

    /// OP-Code: `0xce`
    /// Mnemonic: `ADC A u8`
    pub fn adc_a_u8(&mut self) -> u8 {
        let val = self.read_u8_at_pc_and_increase();
        self.add8c(val);
        2
    }

    /// OP-Code: `0x90`
    /// Mnemonic: `SUB A B`
    pub fn sub_a_b(&mut self) -> u8 {
        self.sub8(self.registers.b);
        1
    }

    /// OP-Code: `0x91`
    /// Mnemonic: `SUB A C`
    pub fn sub_a_c(&mut self) -> u8 {
        self.sub8(self.registers.c);
        1
    }

    /// OP-Code: `0x92`
    /// Mnemonic: `SUB A D`
    pub fn sub_a_d(&mut self) -> u8 {
        self.sub8(self.registers.d);
        1
    }

    /// OP-Code: `0x93`
    /// Mnemonic: `SUB A E`
    pub fn sub_a_e(&mut self) -> u8 {
        self.sub8(self.registers.e);
        1
    }

    /// OP-Code: `0x94`
    /// Mnemonic: `SUB A H`
    pub fn sub_a_h(&mut self) -> u8 {
        self.sub8(self.registers.h);
        1
    }

    /// OP-Code: `0x95`
    /// Mnemonic: `SUB A L`
    pub fn sub_a_l(&mut self) -> u8 {
        self.sub8(self.registers.l);
        1
    }

    /// OP-Code: `0x96`
    /// Mnemonic: `SUB A (HL)`
    pub fn sub_a_hlp(&mut self) -> u8 {
        let address = self.registers.get_hl();
        self.sub8(*self.read(address));
        2
    }

    /// OP-Code: `0x97`
    /// Mnemonic: `SUB A A`
    pub fn sub_a_a(&mut self) -> u8 {
        self.sub8(self.registers.a);
        1
    }

    /// OP-Code: `0xD6`
    /// Mnemonic: `SUB A u8`
    pub fn sub_a_u8(&mut self) -> u8 {
        let val = self.read_u8_at_pc_and_increase();
        self.sub8(val);
        1
    }

    /// OP-Code: `0x98`
    /// Mnemonic: `SBC A B`
    pub fn sbc_a_b(&mut self) -> u8 {
        self.sub8c(self.registers.b);
        1
    }

    /// OP-Code: `0x99`
    /// Mnemonic: `SBC A C`
    pub fn sbc_a_c(&mut self) -> u8 {
        self.sub8c(self.registers.c);
        1
    }

    /// OP-Code: `0x9A`
    /// Mnemonic: `SBC A D`
    pub fn sbc_a_d(&mut self) -> u8 {
        self.sub8c(self.registers.d);
        1
    }

    /// OP-Code: `0x9B`
    /// Mnemonic: `SBC A E`
    pub fn sbc_a_e(&mut self) -> u8 {
        self.sub8c(self.registers.e);
        1
    }

    /// OP-Code: `0x9C`
    /// Mnemonic: `SBC A H`
    pub fn sbc_a_h(&mut self) -> u8 {
        self.sub8c(self.registers.h);
        1
    }

    /// OP-Code: `0x9D`
    /// Mnemonic: `SBC A L`
    pub fn sbc_a_l(&mut self) -> u8 {
        self.sub8c(self.registers.l);
        1
    }

    /// OP-Code: `0x9E`
    /// Mnemonic: `SBC A L`
    pub fn sbc_a_hlp(&mut self) -> u8 {
        let address = self.registers.get_hl();
        self.sub8c(*self.read(address));
        2
    }

    /// OP-Code: `0x9F`
    /// Mnemonic: `SBC A A`
    pub fn sbc_a_a(&mut self) -> u8 {
        self.sub8c(self.registers.a);
        1
    }

    /// OP-Code: `0xDE`
    /// Mnemonic: `SBC A u8`
    pub fn sbc_a_u8(&mut self) -> u8 {
        let val = self.read_u8_at_pc_and_increase();
        self.sub8c(val);
        2
    }

    /// OP-Code: `0xC3`
    /// Mnemonic: `JP`
    pub fn jp_a16(&mut self) -> u8 {
        let address = self.read_u16_at_pc_and_increase();
        self.jp(address);
        4
    }

    /// OP-Code: `0xCF`
    /// Mnemonic: `RST 0x08`
    pub fn rst_08h(&mut self) -> u8 {
        self.rst(0x08);
        4
    }

    /// OP-Code: `0xDF`
    /// Mnemonic: `RST 0x18`
    pub fn rst_18h(&mut self) -> u8 {
        self.rst(0x18);
        4
    }

    /// OP-Code: `0xEF`
    /// Mnemonic: `RST 0x28`
    pub fn rst_28h(&mut self) -> u8 {
        self.rst(0x28);
        4
    }

    /// OP-Code: `0xFF`
    /// Mnemonic: `RST 0x38`
    pub fn rst_38h(&mut self) -> u8 {
        self.rst(0x38);
        4
    }

    /// OP-Code: `0xF3`
    /// Mnemonic: `DI`
    pub fn di(&mut self) -> u8 {
        self.registers.ime = false;
        // TODO: cancel scheduled interrupts.
        tracing::warn!("todo: cancel scheduled interrupts");
        1
    }

    /// OP-Code: `0xFB`
    /// Mnemonic: `EI`
    pub fn ei(&mut self) -> u8 {
        self.registers.ime = true;
        tracing::warn!("todo: schedule interrupts");
        // TODO: schedule interrupts.
        1
    }

    /// OP-Code: `0xC1`
    /// Mnemonic: `POP BC`
    pub fn pop_bc(&mut self) -> u8 {
        let val = self.pop_stack_u16();
        self.registers.set_bc(val);
        3
    }

    /// OP-Code: `0xD1`
    /// Mnemonic: `POP DE`
    pub fn pop_de(&mut self) -> u8 {
        let val = self.pop_stack_u16();
        self.registers.set_de(val);
        3
    }

    /// OP-Code: `0xE1`
    /// Mnemonic: `POP HL`
    pub fn pop_hl(&mut self) -> u8 {
        let val = self.pop_stack_u16();
        self.registers.set_hl(val);
        3
    }

    /// OP-Code: `0xF1`
    /// Mnemonic: `POP AF`
    pub fn pop_af(&mut self) -> u8 {
        let val = self.pop_stack_u16();
        self.registers.set_af(val);
        3
    }

    /// OP-Code: `0xC5`
    /// Mnemonic: `PUSH BC`
    pub fn push_bc(&mut self) -> u8 {
        self.push_stack_u16(self.registers.get_bc());
        4
    }

    /// OP-Code: `0xD5`
    /// Mnemonic: `PUSH DE`
    pub fn push_de(&mut self) -> u8 {
        self.push_stack_u16(self.registers.get_de());
        4
    }

    /// OP-Code: `0xE5`
    /// Mnemonic: `PUSH HL`
    pub fn push_hl(&mut self) -> u8 {
        self.push_stack_u16(self.registers.get_hl());
        4
    }

    /// OP-Code: `0xF5`
    /// Mnemonic: `PUSH AF`
    pub fn push_af(&mut self) -> u8 {
        self.push_stack_u16(self.registers.get_af());
        4
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::Cpu;

    #[test]
    fn test_exec_instruction() {
        let unused_opcodes = [
            0xd3, 0xdb, 0xdd, 0xe3, 0xe4, 0xeb, 0xec, 0xed, 0xf4, 0xfc, 0xfd,
        ];
        for opcode in 0x00..0xff {
            if unused_opcodes.contains(&opcode) {
                continue;
            }
            let mut cpu: Cpu = Cpu::new();
            cpu.exec_instruction(opcode);
        }
    }

    #[test]
    fn nop() {
        let cpu: Cpu = Cpu::new();
        let expected_cpu = cpu;
        let cycles_needed = Cpu::nop();
        assert_eq!(cycles_needed, 1);
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    fn rlca() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0b1000_0001;
        cpu.rlca();
        assert_eq!(cpu.registers.a, 0b0000_0011);
        assert!(cpu.registers.get_flag_c());
        cpu.registers.a = 0b0000_0001;
        cpu.rlca();
        assert_eq!(cpu.registers.a, 0b0000_0010);
        assert!(!cpu.registers.get_flag_c());
        assert!(!cpu.registers.get_flag_z());
        assert!(!cpu.registers.get_flag_n());
        assert!(!cpu.registers.get_flag_h());
    }
}
