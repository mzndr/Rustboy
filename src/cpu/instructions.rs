use super::disassembler::decode_instruction;
use super::utils;
use super::{registers::REGISTER_A_INDEX, utils::merge_u8s, Cpu};

impl Cpu {
    /// XXX dst, src
    /// TODO: Streamline everything
    #[allow(clippy::too_many_lines)]
    #[tracing::instrument(name = "exec", target = "", skip(self), fields(c))]
    pub fn exec_instruction(&mut self) -> u8 {
        if self.halted {
            return 0;
        }

        if self.gb_doctor_enable {
            self.gb_doctor_log();
        }
        let opcode = self.read_u8_at_pc_and_increase();
        let left_nibble = opcode >> 4;
        let right_nibble = opcode & 0b1111;
        let mnemonic = decode_instruction(opcode);
        let pc = self.registers.pc;
        let pc_mem = self.read_u16_at_pc();
        tracing::Span::current().record(
            "c",
            format!("PC: {pc:0>4x} (PC):{pc_mem:0>4x} {opcode:0>2x}: {mnemonic}"),
        );

        if self.schedule_ei {
            self.schedule_ei = false;
            self.registers.ime = true;
        }

        tracing::debug!("executing instruction");
        match opcode {
            0x00 => Self::nop(),
            0x01 => self.ld_bc_d16(),
            0x02 => self.ld_bc(),
            0x03 => self.inc_bc(),
            0x04 => self.inc_b(),
            0x05 => self.dec_b(),
            0x06 => self.ld_b_d8(),
            0x07 => self.rlc(REGISTER_A_INDEX),
            0x08 => self.ld_a16_sp(),
            0x09 => self.add_hl_bc(),
            0x0a => self.ld_a_bc_ptr(),
            0x0b => self.dec_bc(),
            0x0c => self.inc_c(),
            0x0d => self.dec_c(),
            0x0e => self.ld_c_d8(),
            0x0f => self.rrc(REGISTER_A_INDEX),
            0x10 => self.stop(),
            0x11 => self.ld_de_d16(),
            0x12 => self.ld_de_ptr_a(),
            0x13 => self.inc_de(),
            0x14 => self.inc_d(),
            0x15 => self.dec_d(),
            0x16 => self.ld_d_d8(),
            0x17 => self.rl(REGISTER_A_INDEX),
            0x18 => self.jr_r8(),
            0x19 => self.add_hl_de(),
            0x1a => self.ld_a_de_ptr(),
            0x1b => self.dec_de(),
            0x1c => self.inc_e(),
            0x1d => self.dec_e(),
            0x1e => self.ld_e_d8(),
            0x1f => self.rr(REGISTER_A_INDEX),
            0x20 => self.jr_nz_r8(),
            0x21 => self.ld_hl_d16(),
            0x22 => self.ld_hl_inc_ptr_a(),
            0x23 => self.inc_hl(),
            0x24 => self.inc_h(),
            0x25 => self.dec_h(),
            0x26 => self.ld_h_d8(),
            0x27 => self.daa(),
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
            0x76 => self.hlt(),
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
            0xc7 => self.rst(0x00),
            0xc8 => self.ret_z(),
            0xc9 => self.ret(),
            0xca => self.jp_z_a16(),
            0xcb => self.exec_cb_instruction(),
            0xcc => self.call_z_a16(),
            0xcd => self.call_a16(),
            0xce => self.adc_d8(),
            0xcf => self.rst(0x08),
            0xd0 => self.ret_nc(),
            0xd1 => self.pop_de(),
            0xd2 => self.jp_nc_a16(),
            0xd4 => self.call_nc_a16(),
            0xd5 => self.push_de(),
            0xd6 => self.sub_d8(),
            0xd7 => self.rst(0x10),
            0xd8 => self.ret_c(),
            0xd9 => self.reti(),
            0xda => self.jp_c_a16(),
            0xdc => self.call_c_a16(),
            0xde => self.sbc_d8(),
            0xdf => self.rst(0x18),
            0xe0 => self.ldh_a8_ptr_a(),
            0xe1 => self.pop_hl(),
            0xe2 => self.ld_c_ptr_a(),
            0xe5 => self.push_hl(),
            0xe6 => self.and_d8(),
            0xe7 => self.rst(0x20),
            0xe8 => self.add_sp_r8(),
            0xe9 => self.jp(self.registers.get_hl()),
            0xea => self.ld_a16_ptr_a(),
            0xee => self.xor_d8(),
            0xef => self.rst(0x28),
            0xf0 => self.ldh_a_a8_ptr(),
            0xf1 => self.pop_af(),
            0xf2 => self.ld_a_c_ptr(),
            0xf3 => self.di(),
            0xf5 => self.push_af(),
            0xf6 => self.or_d8(),
            0xf7 => self.rst(0x30),
            0xf8 => self.ld_hl_sp_r8(),
            0xf9 => self.ld_sp_hl(),
            0xfa => self.ld_a_a16_ptr(),
            0xfb => self.ei(),
            0xfe => self.cp_d8(),
            0xff => self.rst(0x38),

            0x70..=0x77 => self.ld_hl_ptr_n(left_nibble),
            0x40..=0x7f => Self::ld(*self.registers.h_index(right_nibble), self.registers.v_index_mut(left_nibble)),
            0x80..=0x87 => self.add8(*self.registers.h_index(right_nibble)),
            0x88..=0x8f => self.add8c(*self.registers.h_index(right_nibble)),
            0x90..=0x97 => self.sub8(*self.registers.h_index(right_nibble)),
            0x98..=0x9f => self.sub8c(*self.registers.h_index(right_nibble)),
            0xa0..=0xa7 => self.and(*self.registers.h_index(right_nibble)),
            0xa8..=0xaf => self.xor(*self.registers.h_index(right_nibble)),
            0xb0..=0xb7 => self.or(*self.registers.h_index(right_nibble)),
            0xb8..=0xbf => self.cp(*self.registers.h_index(right_nibble)),

            0xd3 | 0xdb | 0xdd | 0xe3 | 0xe4 | 0xeb | 0xec | 0xed | 0xf4 | 0xfd | 0xfc => {
                let msg = format!("unused opcode called: 0x{opcode:x}");
                tracing::error!(msg);
                panic!("{msg}")
            }
        }
    }

    pub fn gb_doctor_log(&self) {
        println!("A:{:0>2X} F:{:0>2X} B:{:0>2X} C:{:0>2X} D:{:0>2X} E:{:0>2X} H:{:0>2X} L:{:0>2X} SP:{:0>2X} PC:{:0>4X} PCMEM:{:0>2X},{:0>2X},{:0>2X},{:0>2X}",
            self.registers.a,
            self.registers.f,
            self.registers.b,
            self.registers.c,
            self.registers.d,
            self.registers.e,
            self.registers.h,
            self.registers.l,
            self.registers.sp,
            self.registers.pc,
            self.mmu.read(self.registers.pc),
            self.mmu.read(self.registers.pc + 1),
            self.mmu.read(self.registers.pc + 2),
            self.mmu.read(self.registers.pc + 3),
        );
    }

    pub fn nop() -> u8 {
        1
    }

    pub fn stop(&mut self) -> u8 {
        self.halted = true;
        let after_stop = self.read_u8_at_pc_and_increase();
        if after_stop != 0x00 {
            tracing::warn!("corrupted stop");
        }

        1
    }

    pub fn hlt(&mut self) -> u8 {
        tracing::debug!("halting");
        self.halted = true;
        4
    }

    pub fn daa(&mut self) -> u8 {
        if !self.registers.get_flag_n() {
            if self.registers.get_flag_c() || self.registers.a > 0x99 {
                self.registers.a = self.registers.a.wrapping_add(0x60);
                self.registers.set_flag_c(true);
            }
            if self.registers.get_flag_h() || self.registers.a & 0xf > 0x9 {
                self.registers.a = self.registers.a.wrapping_add(0x06);
                self.registers.set_flag_h(false);
            }
        } else if self.registers.get_flag_c() && self.registers.get_flag_h() {
            self.registers.a = self.registers.a.wrapping_add(0x9a);
            self.registers.set_flag_h(false);
        } else if self.registers.get_flag_c() {
            self.registers.a = self.registers.a.wrapping_add(0xa0);
        } else if self.registers.get_flag_h() {
            self.registers.a = self.registers.a.wrapping_add(0xfa);
            self.registers.set_flag_h(false);
        }
        self.registers.set_flag_z(self.registers.a == 0);
        2
    }

    pub fn ret(&mut self) -> u8 {
        self.registers.pc = self.pop_stack_u16();
        4
    }

    pub fn rst(&mut self, address: u16) -> u8 {
        self.push_stack_u16(self.registers.get_pc().wrapping_add(1));
        self.registers.set_pc(address);
        3
    }

    pub fn inc16(val: u16) -> u16 {
        val.wrapping_add(1)
    }

    pub fn dec16(val: u16) -> u16 {
        val.wrapping_sub(1)
    }

    pub fn call(&mut self, addr: u16) -> u8 {
        self.push_stack_u16(self.registers.pc);
        self.registers.pc = addr;
        4
    }

    pub fn call_a16(&mut self) -> u8 {
        let addr = self.read_u16_at_pc_and_increase();
        self.call(addr)
    }

    pub fn call_nz_a16(&mut self) -> u8 {
        let addr = self.read_u16_at_pc_and_increase();
        if self.registers.get_flag_z() {
            return 4;
        }
        self.call(addr)
    }

    pub fn call_z_a16(&mut self) -> u8 {
        let addr = self.read_u16_at_pc_and_increase();
        if !self.registers.get_flag_z() {
            return 4;
        }
        self.call(addr)
    }

    pub fn call_nc_a16(&mut self) -> u8 {
        let addr = self.read_u16_at_pc_and_increase();
        if self.registers.get_flag_c() {
            return 4;
        }
        self.call(addr)
    }

    pub fn call_c_a16(&mut self) -> u8 {
        let addr = self.read_u16_at_pc_and_increase();
        if !self.registers.get_flag_c() {
            return 4;
        }
        self.call(addr)
    }

    pub fn ld(src: u8, dst: &mut u8) -> u8 {
        *dst = src;
        1
    }

    pub fn inc8(&mut self, val: u8) -> u8 {
        let w = val.wrapping_add(1);
        self.registers.set_flag_h(Self::check_add_u8_hc(val, 1));
        self.registers.set_flag_z(w == 0);
        self.registers.set_flag_n(false);
        w
    }

    pub fn cpl(&mut self) -> u8 {
        self.registers.set_flag_n(true);
        self.registers.set_flag_h(true);
        self.registers.a ^= 0xFF;
        1
    }

    pub fn ccf(&mut self) -> u8 {
        self.registers.set_flag_n(false);
        self.registers.set_flag_h(false);
        self.registers.set_flag_c(!self.registers.get_flag_c());
        1
    }

    pub fn scf(&mut self) -> u8 {
        self.registers.set_flag_n(false);
        self.registers.set_flag_h(false);
        self.registers.set_flag_c(true);
        1
    }

    pub fn dec8(&mut self, val: u8) -> u8 {
        let w = val.wrapping_sub(1);
        self.registers.set_flag_h(Self::check_sub_u8_hc(val, 1));
        self.registers.set_flag_z(w == 0);
        self.registers.set_flag_n(true);
        w
    }

    pub fn add16(&mut self, val: u16) {
        let hl = self.registers.get_hl();
        let sum = val.wrapping_add(hl);
        self.registers.set_flag_h(Self::check_add_u16_hc(val, hl));
        self.registers
            .set_flag_c(u32::from(val) + u32::from(hl) > 0xFFFF);
        self.registers.set_flag_n(false);
        self.registers.set_hl(sum);
    }

    pub fn add8c(&mut self, val: u8) -> u8 {
        self.add8(val.wrapping_add(self.registers.get_flag_c().into()));

        1
    }

    pub fn sub8c(&mut self, val: u8) -> u8 {
        self.sub8(val.wrapping_sub(self.registers.get_flag_c().into()));
        1
    }

    pub fn add8(&mut self, val: u8) -> u8 {
        let a = self.registers.a;
        let result = a.wrapping_add(val);
        self.registers.set_flag_h(Self::check_add_u8_hc(a, val));
        self.registers.set_flag_z(result == 0);
        self.registers.set_flag_n(false);
        self.registers
            .set_flag_c(u16::from(val) + u16::from(a) > 0xFF);
        self.registers.a = result;

        1
    }

    pub fn sub8(&mut self, val: u8) -> u8 {
        let a = self.registers.a;
        let result = a.wrapping_sub(val);
        self.registers.set_flag_h(Self::check_sub_u8_hc(a, val));
        self.registers.set_flag_z(result == 0);
        self.registers.set_flag_n(true);
        self.registers.set_flag_c(u16::from(val) > u16::from(a));
        self.registers.a = result;

        1
    }

    pub fn jp(&mut self, address: u16) -> u8 {
        self.registers.set_pc(address);
        4
    }

    pub fn jp_nz_a16(&mut self) -> u8 {
        if self.registers.get_flag_z() {
            return 3;
        }
        self.jp_a16()
    }

    pub fn jp_nc_a16(&mut self) -> u8 {
        if self.registers.get_flag_c() {
            return 3;
        }
        self.jp_a16()
    }

    pub fn jp_z_a16(&mut self) -> u8 {
        if !self.registers.get_flag_z() {
            return 3;
        }
        self.jp_a16()
    }

    pub fn jp_c_a16(&mut self) -> u8 {
        if !self.registers.get_flag_c() {
            return 3;
        }
        self.jp_a16()
    }

    #[allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
    pub fn jr(&mut self, val: u8) {
        let n = val as i8;
        self.registers.pc = ((self.registers.pc as u32 as i32) + (n as i32)) as u16;
    }

    pub fn xor(&mut self, val: u8) -> u8 {
        self.registers.a ^= val;
        self.registers.set_flag_z(self.registers.a == 0);
        self.registers.set_flag_n(false);
        self.registers.set_flag_h(false);
        self.registers.set_flag_c(false);

        1
    }

    pub fn and(&mut self, val: u8) -> u8 {
        self.registers.a &= val;
        self.registers.set_flag_z(self.registers.a == 0);
        self.registers.set_flag_n(false);
        self.registers.set_flag_h(true);
        self.registers.set_flag_c(false);

        1
    }

    pub fn or(&mut self, val: u8) -> u8 {
        self.registers.a |= val;
        self.registers.set_flag_z(self.registers.a == 0);
        self.registers.set_flag_n(false);
        self.registers.set_flag_h(false);
        self.registers.set_flag_c(false);

        1
    }

    pub fn cp(&mut self, val: u8) -> u8 {
        let a = self.registers.a;
        let result = a.wrapping_sub(val);
        self.registers.set_flag_h(Self::check_sub_u8_hc(a, val));
        self.registers.set_flag_z(result == 0);
        self.registers.set_flag_n(true);
        self.registers.set_flag_c(u16::from(val) > u16::from(a));

        1
    }

    pub fn ld_b_hl_ptr(&mut self) -> u8 {
        self.registers.b = self.mmu.read(self.registers.get_hl());
        2
    }

    pub fn ld_d_hl_ptr(&mut self) -> u8 {
        self.registers.d = self.mmu.read(self.registers.get_hl());
        2
    }

    pub fn ld_h_hl_ptr(&mut self) -> u8 {
        self.registers.h = self.mmu.read(self.registers.get_hl());
        2
    }

    pub fn ld_c_hl_ptr(&mut self) -> u8 {
        self.registers.c = self.mmu.read(self.registers.get_hl());
        2
    }

    pub fn ld_e_hl_ptr(&mut self) -> u8 {
        self.registers.e = self.mmu.read(self.registers.get_hl());
        2
    }

    pub fn ld_l_hl_ptr(&mut self) -> u8 {
        self.registers.l = self.mmu.read(self.registers.get_hl());
        2
    }

    pub fn ld_a_hl_ptr(&mut self) -> u8 {
        self.registers.a = self.mmu.read(self.registers.get_hl());
        2
    }

    pub fn ld_hl_sp_r8(&mut self) -> u8 {
        let e = self.read_u8_at_pc_and_increase();
        let result = u16::from(e) + self.registers.sp;
        self.registers.set_flag_z(false);
        self.registers.set_flag_n(false);
        self.registers.set_flag_h(Self::check_add_u16_hc(
            u16::from(e),
            self.registers.get_sp(),
        ));
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
        self.mmu.write_u8(addr, self.registers.a);
        4
    }

    pub fn ld_a_a16_ptr(&mut self) -> u8 {
        let addr = self.read_u16_at_pc_and_increase();
        let val = self.mmu.read(addr);
        self.registers.a = val;
        4
    }

    pub fn add_sp_r8(&mut self) -> u8 {
        let e = self.read_u8_at_pc_and_increase();
        self.registers.set_flag_z(false);
        self.registers.set_flag_n(false);
        self.registers.set_flag_h(Self::check_add_u16_hc(
            u16::from(e),
            self.registers.get_sp(),
        ));
        self.registers
            .set_flag_c(u32::from(e) + u32::from(self.registers.get_sp()) > 0xFFFF);

        let result = self.registers.get_sp() + u16::from(e);

        self.registers.set_sp(result);
        4
    }

    pub fn ld_c_ptr_a(&mut self) -> u8 {
        let addr = merge_u8s(0xff, self.registers.c);
        self.mmu.write_u8(addr, self.registers.a);
        2
    }

    pub fn ld_a_c_ptr(&mut self) -> u8 {
        let addr = merge_u8s(0xff, self.registers.c);
        self.registers.a = self.mmu.read(addr);
        2
    }

    pub fn ldh_a8_ptr_a(&mut self) -> u8 {
        let addr = merge_u8s(0xff, self.read_u8_at_pc_and_increase());
        self.mmu.write_u8(addr, self.registers.a);
        3
    }

    pub fn ldh_a_a8_ptr(&mut self) -> u8 {
        let n = self.read_u8_at_pc_and_increase();
        let addr = 0xFF00 | (n as u16); 
        self.registers.a = self.mmu.read(addr);
        3
    }

    pub fn reti(&mut self) -> u8 {
        self.ret();
        self.registers.ime = true;
        4
    }

    pub fn ld_hl_ptr_d8(&mut self) -> u8 {
        let val = self.read_u8_at_pc_and_increase();
        let hl = self.registers.get_hl();
        self.mmu.write_u8(hl, val);
        3
    }

    pub fn ld_a_bc_ptr(&mut self) -> u8 {
        Self::ld(
            self.mmu.read(self.registers.get_bc()),
            &mut self.registers.a,
        );
        2
    }

    pub fn ld_a_de_ptr(&mut self) -> u8 {
        Self::ld(
            self.mmu.read(self.registers.get_de()),
            &mut self.registers.a,
        );
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
        self.mmu.write_u8(hl, self.registers.a);
        self.registers.set_hl(hl.wrapping_add(1));
        2
    }

    pub fn ld_hl_dec_ptr_a(&mut self) -> u8 {
        let hl = self.registers.get_hl();
        self.mmu.write_u8(hl, self.registers.a);
        self.registers.set_hl(hl.wrapping_sub(1));
        2
    }

    pub fn ld_a_hl_inc_ptr(&mut self) -> u8 {
        let hl = self.registers.get_hl();
        Self::ld(self.mmu.read(hl), &mut self.registers.a);
        self.registers.set_hl(hl.wrapping_add(1));
        2
    }

    pub fn ld_a_hl_dec_ptr(&mut self) -> u8 {
        let hl = self.registers.get_hl();
        Self::ld(self.mmu.read(hl), &mut self.registers.a);
        self.registers.set_hl(hl.wrapping_sub(1));
        2
    }

    pub fn ld_de_ptr_a(&mut self) -> u8 {
        let val = self.registers.a;
        self.mmu.write_u8(self.registers.get_de(), val);
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

    pub fn ld_hl_ptr_n(&mut self, register_idx: u8) -> u8 {
        let val = *self.registers.h_index(register_idx);
        self.mmu.write_u8(self.registers.get_hl(), val);
        2
    }

    pub fn add_hl_ptr(&mut self) -> u8 {
        self.add8(self.mmu.read(self.registers.get_hl()));
        2
    }

    pub fn adc_hl_ptr(&mut self) -> u8 {
        self.sub8c(self.mmu.read(self.registers.get_hl()));
        2
    }

    pub fn adc_d8(&mut self) -> u8 {
        let d8 = self.read_u8_at_pc_and_increase();
        self.add8c(d8);
        1
    }

    pub fn sub_hl_ptr(&mut self) -> u8 {
        self.sub8(self.mmu.read(self.registers.get_hl()));
        2
    }

    pub fn sbc_hl_ptr(&mut self) -> u8 {
        self.sub8c(self.mmu.read(self.registers.get_hl()));
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
        self.or(self.mmu.read(self.registers.get_hl()));
        2
    }

    pub fn xor_hl_ptr(&mut self) -> u8 {
        self.xor(self.mmu.read(self.registers.get_hl()));
        2
    }

    pub fn and_hl_ptr(&mut self) -> u8 {
        self.and(self.mmu.read(self.registers.get_hl()));
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
        self.sub8(d8);
        2
    }

    pub fn cp_hl(&mut self) -> u8 {
        self.cp(self.mmu.read(self.registers.get_hl()));
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
        self.mmu.write_u16(addr, self.registers.pc.wrapping_sub(2));
        5
    }

    pub fn ld_bc(&mut self) -> u8 {
        let val = self.registers.a;
        let hl = self.registers.get_bc();
        self.mmu.write_u8(hl, val);
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

    pub fn rr_val(&mut self, val: u8) -> u8 {
        let prev_carry = u8::from(self.registers.get_flag_c());
        self.registers.set_flag_c((val & 0b0000_0001) == 1);
        let mut result = val.rotate_right(1);
        result |= prev_carry << 7;
        self.registers.set_flag_n(false);
        self.registers.set_flag_h(false);
        result
    }

    pub fn rr(&mut self, register_idx: u8) -> u8 {
        *self.registers.h_index_mut(register_idx) = self.rr_val(*self.registers.h_index(register_idx));
        1
    }

    pub fn rl_val(&mut self, val: u8) -> u8 {
        let prev_carry = u8::from(self.registers.get_flag_c());
        self.registers.set_flag_c((val >> 7) == 1);
        let mut result = val.rotate_left(1);
        result |= prev_carry;
        self.registers.set_flag_n(false);
        self.registers.set_flag_h(false);
        result
    }

    pub fn rl(&mut self, register_idx: u8) -> u8 {
        *self.registers.h_index_mut(register_idx) = self.rl_val(*self.registers.h_index(register_idx));
        1
    }

    pub fn rlc_val(&mut self, val: u8) -> u8 {
        let result = val.rotate_left(1);
        // Right most bit, that has wrapped around gets copied to the carry flag.
        self.registers.set_flag_c((result & 1) == 1);
        self.registers.set_flag_n(false);
        self.registers.set_flag_h(false);
        result
    }

    pub fn rlc(&mut self, reg_idx: u8) -> u8 {
        *self.registers.h_index_mut(reg_idx) = self.rlc_val(*self.registers.h_index(reg_idx));
        1
    }

    pub fn rrc_val(&mut self, val: u8) -> u8 {
        // Right most bit, that will wrap around gets copied to the carry flag.
        self.registers.set_flag_c((val & 1) == 1);
        let result = val.rotate_right(1);
        self.registers.set_flag_n(false);
        self.registers.set_flag_h(false);
        result
    }

    pub fn rrc(&mut self, reg_idx: u8) -> u8 {
        // Right most bit, that will wrap around gets copied to the carry flag.
        *self.registers.h_index_mut(reg_idx) = self.rrc_val(*self.registers.h_index(reg_idx));
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
        self.registers.l = self.dec8(self.registers.l);
        1
    }

    pub fn dec_a(&mut self) -> u8 {
        self.registers.a = self.dec8(self.registers.a);
        1
    }

    pub fn inc_hlp(&mut self) -> u8 {
        let address = self.registers.get_hl();
        let val = self.mmu.read(address);
        let res = self.inc8(val);
        self.mmu.write_u8(address, res);
        3
    }

    pub fn dec_hlp(&mut self) -> u8 {
        let address = self.registers.get_hl();
        let val = self.mmu.read(address);
        let res = self.dec8(val);
        self.mmu.write_u8(address, res);
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

    pub fn di(&mut self) -> u8 {
        self.registers.ime = false;
        1
    }

    pub fn ei(&mut self) -> u8 {
        self.schedule_ei = true;
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
