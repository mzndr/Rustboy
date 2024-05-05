use super::{utils::merge_u8s, Cpu};

impl Cpu {
    /// XXX dst, src
    #[allow(clippy::too_many_lines)]
    pub fn exec_instruction(&mut self, opcode: u8) -> u8 {
        let dst_idx = opcode & 0b1111_0000 >> 4;
        let src_idx = opcode & 0b1111;

        match opcode {
            0x00 => Self::nop(),
            0x01 => self.ld_bc_d16(),
            0x02 => self.ld_bc(),
            0x03 => self.inc_bc(),
            0x04 => self.inc_b(),
            0x05 => self.dec_b(),
            0x06 => self.ld_b_d8(),
            0x07 => self.rlca(),
            0x08 => self.ld_a16_sp(),
            0x09 => self.add_hl_bc(),
            0x0a => self.ld_a_bc_ptr(),
            0x0b => self.dec_bc(),
            0x0c => self.inc_c(),
            0x0d => self.dec_c(),
            0x0e => self.ld_c_d8(),
            0x0f => self.rrca(),
            0x10 => todo!("STOP"),
            0x11 => self.ld_de_d16(),
            0x12 => self.ld_de_ptr_a(),
            0x13 => self.inc_de(),
            0x14 => self.inc_d(),
            0x15 => self.dec_d(),
            0x16 => self.ld_d_d8(),
            0x17 => self.rla(),
            0x18 => self.jr_r8(),
            0x19 => self.add_hl_de(),
            0x1a => self.ld_a_de_ptr(),
            0x1b => self.dec_de(),
            0x1c => self.inc_e(),
            0x1d => self.dec_e(),
            0x1e => self.ld_e_d8(),
            0x1f => self.rra(),
            0x20 => self.jr_nz_r8(),
            0x21 => self.ld_hl_d16(),
            0x22 => self.ld_hl_inc_ptr_a(),
            0x23 => self.inc_hl(),
            0x24 => self.inc_h(),
            0x25 => self.dec_h(),
            0x26 => self.ld_h_d8(),
            0x27 => todo!("DAA"),
            0x28 => self.jr_z_r8(),
            0x29 => self.add_hl_hl(),
            0x2a => self.ld_a_hl_inc_ptr(),
            0x2b => self.dec_hl(),
            0x2c => self.inc_l(),
            0x2d => self.dec_l(),
            0x2e => self.ld_l_d8(),
            0x2f => self.cpl(),
            0x30 => self.jr_nc_r8(),
            0x31 => self.ld_sp_d16(),
            0x32 => self.ld_hl_dec_ptr_a(),
            0x33 => self.inc_sp(),
            0x34 => self.inc_hlp(),
            0x35 => self.dec_hlp(),
            0x36 => self.ld_hl_ptr_d8(),
            0x37 => self.scf(),
            0x38 => self.jr_c_r8(),
            0x39 => self.add_hl_sp(),
            0x3a => self.ld_a_hl_dec_ptr(),
            0x3b => self.dec_sp(),
            0x3c => self.inc_a(),
            0x3d => self.dec_a(),
            0x3e => self.ld_a_d8(),
            0x3f => self.ccf(),
            0x46 => self.ld_b_hl_ptr(),
            0x4e => self.ld_c_hl_ptr(),
            0x56 => self.ld_d_hl_ptr(),
            0x5e => self.ld_e_hl_ptr(),
            0x66 => self.ld_h_hl_ptr(),
            0x6e => self.ld_l_hl_ptr(),
            0x76 => todo!("HLT"),
            0x7e => self.ld_a_hl_ptr(),
            0x86 => self.add_hl_ptr(),
            0x8e => self.adc_hl_ptr(),
            0x96 => self.sub_hl_ptr(),
            0x9e => self.sbc_hl_ptr(),
            0xa6 => self.and_hl_ptr(),
            0xae => self.xor_hl_ptr(),
            0xb6 => self.or_hl_ptr(),
            0xbe => self.cp_hl(),
            0xc0 => self.ret_nz(),
            0xc1 => self.pop_bc(),
            0xc2 => self.jp_nz_a16(),
            0xc3 => self.jp_a16(),
            0xc4 => self.call_nz_a16(),
            0xc5 => self.push_bc(),
            0xc6 => self.add_d8(),
            0xc7 => self.call(0x00),
            0xc8 => self.ret_z(),
            0xc9 => self.ret(),
            0xca => self.jp_z_a16(),
            0xcb => self.exec_cb_instruction(),
            0xcc => self.call_z_a16(),
            0xcd => self.call_a16(),
            0xce => self.adc_d8(),
            0xcf => self.call(0x08),
            0xd0 => self.ret_nc(),
            0xd1 => self.pop_de(),
            0xd2 => self.jp_nc_a16(),
            0xd4 => self.call_nc_a16(),
            0xd5 => self.push_de(),
            0xd6 => self.sub_d8(),
            0xd7 => self.call(0x10),
            0xd8 => self.ret_c(),
            0xd9 => self.reti(),
            0xda => self.jp_c_a16(),
            0xdc => self.call_c_a16(),
            0xde => self.sbc_d8(),
            0xdf => self.call(0x18),
            0xe0 => self.ldh_a8_ptr_a(),
            0xe1 => self.pop_hl(),
            0xe2 => self.ld_c_ptr_a(),
            0xe5 => self.push_hl(),
            0xe6 => self.and_d8(),
            0xe7 => self.call(0x20),
            0xe8 => self.add_sp_r8(),
            0xe9 => self.jp(self.registers.get_hl()),
            0xea => self.ld_a16_ptr_a(),
            0xee => self.xor_d8(),
            0xef => self.call(0x28),
            0xf0 => self.ldh_a_a8_ptr(),
            0xf1 => self.pop_af(),
            0xf2 => self.ld_a_c_ptr(),
            0xf3 => self.di(),
            0xf5 => self.push_af(),
            0xf6 => self.or_d8(),
            0xf7 => self.call(0x30),
            0xf8 => self.ld_hl_sp_r8(),
            0xf9 => self.ld_sp_hl(),
            0xfa => self.ei(),
            0xfb => self.ld_a_a16_ptr(),
            0xfe => self.cp_d8(),
            0xff => self.call(0x38),

            0x70..=0x77 => self.ld_hl_ptr_n(dst_idx),
            0x40..=0x7f => Self::ld(self.registers[src_idx], &mut self.registers[dst_idx]),
            0x80..=0x87 => self.add8(self.registers[src_idx]),
            0x88..=0x8f => self.add8c(self.registers[src_idx]),
            0x90..=0x97 => self.sub8(self.registers[src_idx]),
            0x98..=0x9f => self.sub8c(self.registers[src_idx]),
            0xa0..=0xa7 => self.and(self.registers[src_idx]),
            0xa8..=0xaf => self.xor(self.registers[src_idx]),
            0xb0..=0xb7 => self.or(self.registers[src_idx]),
            0xb8..=0xbf => self.cp(self.registers[src_idx]),

            0xd3 | 0xdb | 0xdd | 0xe3 | 0xe4 | 0xeb | 0xec | 0xed | 0xf4 | 0xfc | 0xfd => {
                tracing::warn!("unused opcode called {:x},", opcode);
                Self::nop()
            }
        }
    }

    pub fn ld_b_hl_ptr(&mut self) -> u8 {
        self.registers.b = *self.read(self.registers.get_hl());
        2
    }

    pub fn ld_d_hl_ptr(&mut self) -> u8 {
        self.registers.d = *self.read(self.registers.get_hl());
        2
    }

    pub fn ld_h_hl_ptr(&mut self) -> u8 {
        self.registers.h = *self.read(self.registers.get_hl());
        2
    }

    pub fn ld_c_hl_ptr(&mut self) -> u8 {
        self.registers.c = *self.read(self.registers.get_hl());
        2
    }

    pub fn ld_e_hl_ptr(&mut self) -> u8 {
        self.registers.e = *self.read(self.registers.get_hl());
        2
    }

    pub fn ld_l_hl_ptr(&mut self) -> u8 {
        self.registers.l = *self.read(self.registers.get_hl());
        2
    }

    pub fn ld_a_hl_ptr(&mut self) -> u8 {
        self.registers.a = *self.read(self.registers.get_hl());
        2
    }

    pub fn ld_hl_sp_r8(&mut self) -> u8 {
        let e = self.read_u8_at_pc_and_increase();
        let result = (e as u16) + self.registers.sp;
        self.registers.set_flag_z(false);
        self.registers.set_flag_n(false);
        self.registers
            .set_flag_h(Self::check_add_u16_hc(e as u16, self.registers.get_sp()));
        self.registers
            .set_flag_c(u32::from(e) + u32::from(self.registers.get_sp()) > 0xFFFF);
        self.registers.set_hl(result);

        3
    }

    pub fn ld_sp_hl(&mut self) -> u8 {
        self.registers.sp = self.registers.get_hl();
        2
    }

    pub fn ld_a16_ptr_a(&mut self) -> u8 {
        let addr = self.read_u16_at_pc_and_increase();
        self.write_u8(addr, self.registers.a);
        4
    }

    pub fn ld_a_a16_ptr(&mut self) -> u8 {
        let addr = self.read_u16_at_pc_and_increase();
        let val = *self.read(addr);
        self.registers.a = val;
        4
    }

    pub fn add_sp_r8(&mut self) -> u8 {
        let e = self.read_u8_at_pc_and_increase();
        self.registers.set_flag_z(false);
        self.registers.set_flag_n(false);
        self.registers
            .set_flag_h(Self::check_add_u16_hc(e as u16, self.registers.get_sp()));
        self.registers
            .set_flag_c(u32::from(e) + u32::from(self.registers.get_sp()) > 0xFFFF);

        let result = self.registers.get_sp() + (e as u16);

        self.registers.set_sp(result);
        4
    }

    pub fn ld_c_ptr_a(&mut self) -> u8 {
        let addr = merge_u8s(self.registers.c, 0xff);
        self.write_u8(addr, self.registers.a);
        2
    }

    pub fn ld_a_c_ptr(&mut self) -> u8 {
        let addr = merge_u8s(self.registers.c, 0xff);
        self.registers.a = *self.read(addr);
        2
    }

    pub fn ldh_a8_ptr_a(&mut self) -> u8 {
        let addr = merge_u8s(self.read_u8_at_pc_and_increase(), 0xff);
        self.write_u8(addr, self.registers.a);
        3
    }

    pub fn ldh_a_a8_ptr(&mut self) -> u8 {
        let addr = merge_u8s(self.registers.a, 0xff);
        let val = self.read_u8_at_pc_and_increase();
        self.write_u8(addr, val);
        3
    }

    pub fn reti(&mut self) -> u8 {
        self.ret();
        self.registers.ime = true;
        4
    }

    pub fn ld_hl_ptr_d8(&mut self) -> u8 {
        Self::ld(
            self.read_u8_at_pc_and_increase(),
            self.read_mut(self.registers.get_hl()),
        );
        3
    }

    pub fn ld_a_bc_ptr(&mut self) -> u8 {
        Self::ld(*self.read(self.registers.get_bc()), &mut self.registers.a);
        2
    }

    pub fn ld_a_de_ptr(&mut self) -> u8 {
        Self::ld(*self.read(self.registers.get_de()), &mut self.registers.a);
        2
    }

    pub fn ld_b_d8(&mut self) -> u8 {
        Self::ld(self.read_u8_at_pc_and_increase(), &mut self.registers.b);
        2
    }

    pub fn ld_d_d8(&mut self) -> u8 {
        Self::ld(self.read_u8_at_pc_and_increase(), &mut self.registers.d);
        2
    }

    pub fn ld_h_d8(&mut self) -> u8 {
        Self::ld(self.read_u8_at_pc_and_increase(), &mut self.registers.h);
        2
    }

    pub fn ld_hl_inc_ptr_a(&mut self) -> u8 {
        let hl = self.registers.get_hl();
        Self::ld(self.registers.a, self.read_mut(hl));
        self.registers.set_hl(hl.wrapping_add(1));
        2
    }

    pub fn ld_hl_dec_ptr_a(&mut self) -> u8 {
        let hl = self.registers.get_hl();
        Self::ld(self.registers.a, self.read_mut(hl));
        self.registers.set_hl(hl.wrapping_sub(1));
        2
    }

    pub fn ld_a_hl_inc_ptr(&mut self) -> u8 {
        let hl = self.registers.get_hl();
        Self::ld(*self.read(hl), &mut self.registers.a);
        self.registers.set_hl(hl.wrapping_add(1));
        2
    }

    pub fn ld_a_hl_dec_ptr(&mut self) -> u8 {
        let hl = self.registers.get_hl();
        Self::ld(*self.read(hl), &mut self.registers.a);
        self.registers.set_hl(hl.wrapping_sub(1));
        2
    }

    pub fn ld_de_ptr_a(&mut self) -> u8 {
        Self::ld(self.registers.a, self.read_mut(self.registers.get_de()));
        2
    }

    pub fn ld_c_d8(&mut self) -> u8 {
        Self::ld(self.read_u8_at_pc_and_increase(), &mut self.registers.c);
        2
    }

    pub fn ld_e_d8(&mut self) -> u8 {
        Self::ld(self.read_u8_at_pc_and_increase(), &mut self.registers.e);
        2
    }

    pub fn ld_l_d8(&mut self) -> u8 {
        Self::ld(self.read_u8_at_pc_and_increase(), &mut self.registers.l);
        2
    }

    pub fn ld_a_d8(&mut self) -> u8 {
        Self::ld(self.read_u8_at_pc_and_increase(), &mut self.registers.a);
        2
    }

    pub fn ld_n_hl_ptr(&mut self, register_idx: u8) -> u8 {
        let val = *self.read(self.registers.get_hl());
        let register = &mut self.registers[register_idx];
        Self::ld(val, register);
        2
    }

    pub fn ld_hl_ptr_n(&mut self, register_idx: u8) -> u8 {
        Self::ld(
            self.registers[register_idx],
            self.read_mut(self.registers.get_hl()),
        );
        2
    }

    pub fn add_hl_ptr(&mut self) -> u8 {
        self.add8(*self.read(self.registers.get_hl()));
        2
    }

    pub fn adc_hl_ptr(&mut self) -> u8 {
        self.sub8c(*self.read(self.registers.get_hl()));
        2
    }

    pub fn adc_d8(&mut self) -> u8 {
        let d8 = self.read_u8_at_pc_and_increase();
        self.add8c(d8);
        1
    }

    pub fn sub_hl_ptr(&mut self) -> u8 {
        self.sub8(*self.read(self.registers.get_hl()));
        2
    }

    pub fn sbc_hl_ptr(&mut self) -> u8 {
        self.sub8c(*self.read(self.registers.get_hl()));
        2
    }

    pub fn sbc_d8(&mut self) -> u8 {
        let d8 = self.read_u8_at_pc_and_increase();
        self.sub8c(d8);
        1
    }

    pub fn xor_d8(&mut self) -> u8 {
        let d8 = self.read_u8_at_pc_and_increase();
        self.xor(d8);
        2
    }

    pub fn and_d8(&mut self) -> u8 {
        let d8 = self.read_u8_at_pc_and_increase();
        self.and(d8);
        2
    }

    pub fn or_hl_ptr(&mut self) -> u8 {
        self.or(*self.read(self.registers.get_hl()));
        2
    }

    pub fn xor_hl_ptr(&mut self) -> u8 {
        self.xor(*self.read(self.registers.get_hl()));
        2
    }

    pub fn and_hl_ptr(&mut self) -> u8 {
        self.and(*self.read(self.registers.get_hl()));
        2
    }

    pub fn or_d8(&mut self) -> u8 {
        let d8 = self.read_u8_at_pc_and_increase();
        self.or(d8);
        2
    }

    pub fn add_d8(&mut self) -> u8 {
        let d8 = self.read_u8_at_pc_and_increase();
        self.add8c(d8);
        2
    }

    pub fn sub_d8(&mut self) -> u8 {
        let d8 = self.read_u8_at_pc_and_increase();
        self.sub8c(d8);
        2
    }

    pub fn cp_hl(&mut self) -> u8 {
        self.cp(*self.read(self.registers.get_hl()));
        2
    }

    pub fn cp_d8(&mut self) -> u8 {
        let d8 = self.read_u8_at_pc_and_increase();
        self.cp(d8);
        2
    }

    pub fn ld_bc_d16(&mut self) -> u8 {
        let val = self.read_u16_at_pc_and_increase();
        self.registers.set_bc(val);
        3
    }

    pub fn ld_de_d16(&mut self) -> u8 {
        let val = self.read_u16_at_pc_and_increase();
        self.registers.set_de(val);
        3
    }

    pub fn ld_hl_d16(&mut self) -> u8 {
        let val = self.read_u16_at_pc_and_increase();
        self.registers.set_hl(val);
        3
    }

    pub fn ld_sp_d16(&mut self) -> u8 {
        let val = self.read_u16_at_pc_and_increase();
        self.registers.set_sp(val);
        3
    }

    pub fn ld_a16_sp(&mut self) -> u8 {
        let addr = self.read_u16_at_pc_and_increase();
        self.write_u16(addr, self.registers.pc.wrapping_sub(2));
        5
    }

    pub fn ld_bc(&mut self) -> u8 {
        Self::ld(self.registers.a, self.read_mut(self.registers.get_bc()));
        2
    }

    pub fn inc_bc(&mut self) -> u8 {
        let r = self.registers.get_bc();
        let res = Self::inc16(r);
        self.registers.set_bc(res);
        2
    }

    pub fn dec_bc(&mut self) -> u8 {
        let r = self.registers.get_bc();
        let res = Self::dec16(r);
        self.registers.set_bc(res);
        2
    }

    pub fn ret_nz(&mut self) -> u8 {
        if self.registers.get_flag_z() {
            return 2;
        }
        self.ret() + 1
    }

    pub fn ret_nc(&mut self) -> u8 {
        if self.registers.get_flag_c() {
            return 2;
        }
        self.ret() + 1
    }

    pub fn ret_z(&mut self) -> u8 {
        if !self.registers.get_flag_z() {
            return 2;
        }
        self.ret() + 1
    }

    pub fn ret_c(&mut self) -> u8 {
        if !self.registers.get_flag_c() {
            return 2;
        }
        self.ret() + 1
    }

    pub fn dec_de(&mut self) -> u8 {
        let r = self.registers.get_de();
        let res = Self::dec16(r);
        self.registers.set_de(res);
        2
    }

    pub fn dec_hl(&mut self) -> u8 {
        let r = self.registers.get_hl();
        let res = Self::dec16(r);
        self.registers.set_hl(res);
        2
    }

    pub fn dec_sp(&mut self) -> u8 {
        let r = self.registers.get_sp();
        let res = Self::dec16(r);
        self.registers.set_sp(res);
        2
    }

    pub fn inc_b(&mut self) -> u8 {
        self.registers.b = self.inc8(self.registers.b);
        1
    }

    pub fn dec_b(&mut self) -> u8 {
        self.registers.b = self.dec8(self.registers.b);
        1
    }

    pub fn add_hl_bc(&mut self) -> u8 {
        let bc = self.registers.get_bc();
        self.add16(bc);
        2
    }

    pub fn inc_de(&mut self) -> u8 {
        let r = self.registers.get_de();
        let res = Self::inc16(r);
        self.registers.set_de(res);
        2
    }

    pub fn inc_d(&mut self) -> u8 {
        self.registers.d = self.inc8(self.registers.d);
        2
    }

    pub fn dec_d(&mut self) -> u8 {
        self.registers.d = self.dec8(self.registers.d);
        2
    }

    pub fn jr_r8(&mut self) -> u8 {
        let val = self.read_u8_at_pc_and_increase();
        self.jr(val);
        3
    }

    pub fn add_hl_de(&mut self) -> u8 {
        let de = self.registers.get_de();
        self.add16(de);
        2
    }

    pub fn dec_c(&mut self) -> u8 {
        self.registers.c = self.dec8(self.registers.c);
        1
    }

    pub fn dec_e(&mut self) -> u8 {
        self.registers.e = self.dec8(self.registers.e);
        1
    }

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

    pub fn inc_h(&mut self) -> u8 {
        self.registers.h = self.inc8(self.registers.h);
        1
    }

    pub fn dec_h(&mut self) -> u8 {
        self.registers.h = self.dec8(self.registers.h);
        1
    }

    pub fn dec_l(&mut self) -> u8 {
        self.registers.h = self.dec8(self.registers.l);
        1
    }

    pub fn dec_a(&mut self) -> u8 {
        self.registers.a = self.dec8(self.registers.a);
        1
    }

    pub fn inc_hlp(&mut self) -> u8 {
        let address = self.registers.get_hl();
        let val = self.read(address);
        let res = self.inc8(*val);
        self.write_u8(address, res);
        3
    }

    pub fn dec_hlp(&mut self) -> u8 {
        let address = self.registers.get_hl();
        let val = self.read(address);
        let res = self.dec8(*val);
        self.write_u8(address, res);
        3
    }

    pub fn jr_nz_r8(&mut self) -> u8 {
        let val = self.read_u8_at_pc_and_increase();
        if !self.registers.get_flag_z() {
            self.jr(val);
            return 3;
        }
        2
    }

    pub fn jr_z_r8(&mut self) -> u8 {
        let val = self.read_u8_at_pc_and_increase();
        if self.registers.get_flag_z() {
            self.jr(val);
            return 3;
        }
        2
    }

    pub fn jr_nc_r8(&mut self) -> u8 {
        let val = self.read_u8_at_pc_and_increase();
        if !self.registers.get_flag_c() {
            self.jr(val);
            return 3;
        }
        2
    }

    pub fn jr_c_r8(&mut self) -> u8 {
        let val = self.read_u8_at_pc_and_increase();
        if self.registers.get_flag_c() {
            self.jr(val);
            return 3;
        }
        2
    }

    pub fn inc_hl(&mut self) -> u8 {
        let r = self.registers.get_hl();
        let res = Self::inc16(r);
        self.registers.set_hl(res);
        2
    }

    pub fn add_hl_hl(&mut self) -> u8 {
        let hl = self.registers.get_hl();
        self.add16(hl);
        2
    }

    pub fn add_hl_sp(&mut self) -> u8 {
        let hl = self.registers.get_sp();
        self.add16(hl);
        2
    }

    pub fn inc_c(&mut self) -> u8 {
        self.registers.c = self.inc8(self.registers.c);
        1
    }

    pub fn inc_e(&mut self) -> u8 {
        self.registers.e = self.inc8(self.registers.e);
        1
    }

    pub fn inc_l(&mut self) -> u8 {
        self.registers.l = self.inc8(self.registers.l);
        1
    }

    pub fn inc_a(&mut self) -> u8 {
        self.registers.a = self.inc8(self.registers.a);
        1
    }

    pub fn inc_sp(&mut self) -> u8 {
        let r = self.registers.get_sp();
        let res = Self::inc16(r);
        self.registers.set_sp(res);
        2
    }

    pub fn jp_a16(&mut self) -> u8 {
        let address = self.read_u16_at_pc_and_increase();
        self.jp(address);
        4
    }

    pub fn rst_08h(&mut self) -> u8 {
        self.rst(0x08);
        4
    }

    pub fn rst_18h(&mut self) -> u8 {
        self.rst(0x18);
        4
    }

    pub fn rst_28h(&mut self) -> u8 {
        self.rst(0x28);
        4
    }

    pub fn rst_38h(&mut self) -> u8 {
        self.rst(0x38);
        4
    }

    pub fn di(&mut self) -> u8 {
        self.registers.ime = false;
        // TODO: cancel scheduled interrupts.
        tracing::warn!("todo: cancel scheduled interrupts");
        1
    }

    pub fn ei(&mut self) -> u8 {
        self.registers.ime = true;
        tracing::warn!("todo: schedule interrupts");
        // TODO: schedule interrupts.
        1
    }

    pub fn pop_bc(&mut self) -> u8 {
        let val = self.pop_stack_u16();
        self.registers.set_bc(val);
        3
    }

    pub fn pop_de(&mut self) -> u8 {
        let val = self.pop_stack_u16();
        self.registers.set_de(val);
        3
    }

    pub fn pop_hl(&mut self) -> u8 {
        let val = self.pop_stack_u16();
        self.registers.set_hl(val);
        3
    }

    pub fn pop_af(&mut self) -> u8 {
        let val = self.pop_stack_u16();
        self.registers.set_af(val);
        3
    }

    pub fn push_bc(&mut self) -> u8 {
        self.push_stack_u16(self.registers.get_bc());
        4
    }

    pub fn push_de(&mut self) -> u8 {
        self.push_stack_u16(self.registers.get_de());
        4
    }

    pub fn push_hl(&mut self) -> u8 {
        self.push_stack_u16(self.registers.get_hl());
        4
    }

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
