use super::Cpu;

impl Cpu {
    pub fn exec_instruction(&mut self, opcode: u8) -> u8 {
        match opcode {
            0x00 => self.nop(),
            0x01 => self.ld_bc_d16(),
            0x02 => self.ld_bcp_a(),
            0x03 => self.inc_bc(),
            0x04 => self.inc_b(),
            0x05 => self.dec_b(),
            0x06 => self.ld_b_d8(),
            0x07 => self.rlca(),
            0x08 => self.ld_a16p_sp(),
            0x09 => self.add_hl_bc(),
            0x0E => self.ld_c_d8(),
            0x13 => self.inc_de(),
            0x14 => self.inc_d(),
            0x15 => self.dec_d(),
            0x19 => self.add_hl_de(),
            0x1D => self.dec_e(),
            0x20 => self.jr_nz_r8(),
            0x21 => self.ld_hl_d16(),
            0x22 => self.ld_hlp_inc_a(),
            0x23 => self.inc_hl(),
            0x24 => self.inc_h(),
            0x25 => self.dec_h(),
            0x29 => self.add_hl_hl(),
            0x2C => self.inc_l(),
            0x32 => self.ld_hlp_dec_a(),
            0x33 => self.inc_sp(),
            0x3E => self.ld_a_d8(),
            0x40 => self.ld_b_b(),
            0x41 => self.ld_b_c(),
            0x42 => self.ld_b_d(),
            0x43 => self.ld_b_e(),
            0x44 => self.ld_b_h(),
            0x45 => self.ld_b_l(),
            0x47 => self.ld_b_a(),
            0x48 => self.ld_c_b(),
            0x49 => self.ld_c_c(),
            0x4A => self.ld_c_d(),
            0x4B => self.ld_c_e(),
            0x4C => self.ld_c_h(),
            0x4D => self.ld_c_l(),
            0x4E => self.ld_c_hlp(),
            0x4F => self.ld_c_a(),
            0x50 => self.ld_d_b(),
            0x51 => self.ld_d_c(),
            0x52 => self.ld_d_d(),
            0x53 => self.ld_d_e(),
            0x54 => self.ld_d_h(),
            0x55 => self.ld_d_l(),
            0x56 => self.ld_d_hlp(),
            0x57 => self.ld_d_a(),
            0x58 => self.ld_e_b(),
            0x59 => self.ld_e_c(),
            0x5A => self.ld_e_d(),
            0x5B => self.ld_e_e(),
            0x5C => self.ld_e_h(),
            0x5D => self.ld_e_l(),
            0x5E => self.ld_e_hlp(),
            0x5F => self.ld_e_a(),
            0x60 => self.ld_h_b(),
            0x61 => self.ld_h_c(),
            0x62 => self.ld_h_d(),
            0x63 => self.ld_h_e(),
            0x64 => self.ld_h_h(),
            0x65 => self.ld_h_l(),
            0x66 => self.ld_h_hlp(),
            0x67 => self.ld_h_a(),
            0x68 => self.ld_l_b(),
            0x69 => self.ld_l_c(),
            0x6A => self.ld_l_d(),
            0x6B => self.ld_l_e(),
            0x6C => self.ld_l_h(),
            0x6D => self.ld_l_l(),
            0x6E => self.ld_l_hlp(),
            0x6F => self.ld_l_a(),
            0x70 => self.ld_hlp_b(),
            0x71 => self.ld_hlp_c(),
            0x72 => self.ld_hlp_d(),
            0x73 => self.ld_hlp_e(),
            0x74 => self.ld_hlp_h(),
            0x75 => self.ld_hlp_l(),
            0x77 => self.ld_hlp_a(),
            0x78 => self.ld_a_b(),
            0x79 => self.ld_a_c(),
            0x7A => self.ld_a_d(),
            0x7B => self.ld_a_e(),
            0x7C => self.ld_a_h(),
            0x7D => self.ld_a_l(),
            0x7E => self.ld_a_hlp(),
            0x7F => self.ld_a_a(),
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
            0x8A => self.adc_a_d(),
            0x8B => self.adc_a_e(),
            0x8C => self.adc_a_h(),
            0x8D => self.adc_a_l(),
            0x8E => self.adc_a_hlp(),
            0x8F => self.adc_a_a(),
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
            0x9A => self.sbc_a_d(),
            0x9B => self.sbc_a_e(),
            0x9C => self.sbc_a_h(),
            0x9D => self.sbc_a_l(),
            0x9E => self.sbc_a_hlp(),
            0x9F => self.sbc_a_a(),
            0xAF => self.xor_a(),
            0xC3 => self.jp_a16(),
            0xCB => self.exec_cb_instruction(),
            

            _ => self.opcode_unknown(opcode),
        }
    }

    pub fn exec_cb_instruction(&mut self) -> u8 {
        let opcode = self.read_u8_at_pc_and_increase();
        match opcode {
            0xFE => self.set_7_hlp(),
            _ => self.cb_opcode_unknown(opcode),
        }
    }

    /// OPCode: 0x80
    /// Mnemonic: ADD A B
    pub fn add_a_b(&mut self) -> u8 {
        self.add8(self.registers.b);
        1
    }

    /// OPCode: 0x81
    /// Mnemonic: ADD A C
    pub fn add_a_c(&mut self) -> u8 {
        self.add8(self.registers.c);
        1
    }

    /// OPCode: 0x82
    /// Mnemonic: ADD A D
    pub fn add_a_d(&mut self) -> u8 {
        self.add8(self.registers.d);
        1
    }

    /// OPCode: 0x83
    /// Mnemonic: ADD A E
    pub fn add_a_e(&mut self) -> u8 {
        self.add8(self.registers.e);
        1
    }

    /// OPCode: 0x84
    /// Mnemonic: ADD A H
    pub fn add_a_h(&mut self) -> u8 {
        self.add8(self.registers.h);
        1
    }

    /// OPCode: 0x85
    /// Mnemonic: ADD A L
    pub fn add_a_l(&mut self) -> u8 {
        self.add8(self.registers.l);
        1
    }

    /// OPCode: 0x86
    /// Mnemonic: ADD A (HL)
    pub fn add_a_hlp(&mut self) -> u8 {
        let address = self.registers.get_hl();
        self.add8(self.read(address));
        2
    }

    /// OPCode: 0x87
    /// Mnemonic: ADD A A
    pub fn add_a_a(&mut self) -> u8 {
        self.add8(self.registers.a);
        1
    }

    /// OPCode: 0x88
    /// Mnemonic: ADC A B
    pub fn adc_a_b(&mut self) -> u8 {
        self.add8c(self.registers.b);
        1
    }

    /// OPCode: 0x89
    /// Mnemonic: ADC A C
    pub fn adc_a_c(&mut self) -> u8 {
        self.add8c(self.registers.c);
        1
    }

    /// OPCode: 0x8A
    /// Mnemonic: ADC A D
    pub fn adc_a_d(&mut self) -> u8 {
        self.add8c(self.registers.d);
        1
    }

    /// OPCode: 0x8B
    /// Mnemonic: ADC A E
    pub fn adc_a_e(&mut self) -> u8 {
        self.add8c(self.registers.e);
        1
    }

    /// OPCode: 0x8C
    /// Mnemonic: ADC A H
    pub fn adc_a_h(&mut self) -> u8 {
        self.add8c(self.registers.h);
        1
    }

    /// OPCode: 0x8D
    /// Mnemonic: ADC A L
    pub fn adc_a_l(&mut self) -> u8 {
        self.add8c(self.registers.l);
        1
    }

    /// OPCode: 0x8E
    /// Mnemonic: ADC A L
    pub fn adc_a_hlp(&mut self) -> u8 {
        let address = self.registers.get_hl();
        self.add8c(self.read(address));
        2
    }

    /// OPCode: 0x8F
    /// Mnemonic: ADC A A
    pub fn adc_a_a(&mut self) -> u8 {
        self.add8c(self.registers.a);
        1
    }

    /// OPCode: 0x90
    /// Mnemonic: SUB A B
    pub fn sub_a_b(&mut self) -> u8 {
        self.sub8(self.registers.b);
        1
    }

    /// OPCode: 0x91
    /// Mnemonic: SUB A C
    pub fn sub_a_c(&mut self) -> u8 {
        self.sub8(self.registers.c);
        1
    }

    /// OPCode: 0x92
    /// Mnemonic: SUB A D
    pub fn sub_a_d(&mut self) -> u8 {
        self.sub8(self.registers.d);
        1
    }

    /// OPCode: 0x93
    /// Mnemonic: SUB A E
    pub fn sub_a_e(&mut self) -> u8 {
        self.sub8(self.registers.e);
        1
    }

    /// OPCode: 0x94
    /// Mnemonic: SUB A H
    pub fn sub_a_h(&mut self) -> u8 {
        self.sub8(self.registers.h);
        1
    }

    /// OPCode: 0x95
    /// Mnemonic: SUB A L
    pub fn sub_a_l(&mut self) -> u8 {
        self.sub8(self.registers.l);
        1
    }

    /// OPCode: 0x96
    /// Mnemonic: SUB A (HL)
    pub fn sub_a_hlp(&mut self) -> u8 {
        let address = self.registers.get_hl();
        self.sub8(self.read(address));
        2
    }

    /// OPCode: 0x97
    /// Mnemonic: SUB A A
    pub fn sub_a_a(&mut self) -> u8 {
        self.sub8(self.registers.a);
        1
    }

    /// OPCode: 0x98
    /// Mnemonic: SBC A B
    pub fn sbc_a_b(&mut self) -> u8 {
        self.sub8c(self.registers.b);
        1
    }

    /// OPCode: 0x99
    /// Mnemonic: SBC A C
    pub fn sbc_a_c(&mut self) -> u8 {
        self.sub8c(self.registers.c);
        1
    }

    /// OPCode: 0x9A
    /// Mnemonic: SBC A D
    pub fn sbc_a_d(&mut self) -> u8 {
        self.sub8c(self.registers.d);
        1
    }

    /// OPCode: 0x9B
    /// Mnemonic: SBC A E
    pub fn sbc_a_e(&mut self) -> u8 {
        self.sub8c(self.registers.e);
        1
    }

    /// OPCode: 0x9C
    /// Mnemonic: SBC A H
    pub fn sbc_a_h(&mut self) -> u8 {
        self.sub8c(self.registers.h);
        1
    }

    /// OPCode: 0x9D
    /// Mnemonic: SBC A L
    pub fn sbc_a_l(&mut self) -> u8 {
        self.sub8c(self.registers.l);
        1
    }

    /// OPCode: 0x9E
    /// Mnemonic: SBC A L
    pub fn sbc_a_hlp(&mut self) -> u8 {
        let address = self.registers.get_hl();
        self.sub8c(self.read(address));
        2
    }

    /// OPCode: 0x9F
    /// Mnemonic: SBC A A
    pub fn sbc_a_a(&mut self) -> u8 {
        self.sub8c(self.registers.a);
        1
    }

    /// OPCode: 0x00
    /// Mnemonic: NOP
    pub fn nop(&mut self) -> u8 {
        1
    }

    /// OPCode: 0x01
    /// Mnemonic: LD BC, d16
    pub fn ld_bc_d16(&mut self) -> u8 {
        let val = self.read_u16_at_pc_and_increase();
        self.registers.set_bc(val);
        3
    }

    /// OPCode: 0x02
    /// Mnemonic: LD (BC), A
    pub fn ld_bcp_a(&mut self) -> u8 {
        let address = self.registers.get_bc();
        let val = self.registers.a;
        self.write_u8(address, val);
        2
    }

    /// OPCode: 0x03
    /// Mnemonic: INC BC
    pub fn inc_bc(&mut self) -> u8 {
        let r = self.registers.get_bc();
        let res = self.inc16(r);
        self.registers.set_bc(res);
        2
    }

    /// OPCode: 0x05
    /// Mnemonic: DEC B
    pub fn dec_b(&mut self) -> u8 {
        self.registers.b = self.dec8(self.registers.b);
        1
    }

    /// OPCode: 0x15
    /// Mnemonic: DEC D
    pub fn dec_d(&mut self) -> u8 {
        self.registers.d = self.dec8(self.registers.d);
        2
    }

    /// OPCode: 0x25
    /// Mnemonic: DEC H
    pub fn dec_h(&mut self) -> u8 {
        self.registers.h = self.dec8(self.registers.h);
        1
    }

    /// OPCode: 0x35
    /// Mnemonic: DEC (HL)
    pub fn dec_hlp(&mut self) -> u8 {
        let address = self.registers.get_hl();
        let val = self.read(address);
        let res = self.dec8(val);
        self.write_u8(address, res);
        3
    }

    /// OPCode: 0x04
    /// Mnemonic: INC B
    pub fn inc_b(&mut self) -> u8 {
        self.registers.b = self.inc8(self.registers.b);
        1
    }

    /// OPCode: 0x14
    /// Mnemonic: INC D
    pub fn inc_d(&mut self) -> u8 {
        self.registers.d = self.inc8(self.registers.d);
        2
    }

    /// OPCode: 0x24
    /// Mnemonic: INC H
    pub fn inc_h(&mut self) -> u8 {
        self.registers.h = self.inc8(self.registers.h);
        1
    }

    /// OPCode: 0x34
    /// Mnemonic: INC (HL)
    pub fn inc_hlp(&mut self) -> u8 {
        let address = self.registers.get_hl();
        let val = self.read(address);
        let res = self.inc8(val);
        self.write_u8(address, res);
        3
    }



    /// OPCode: 0x06
    /// Mnemonic: LD B, d8
    pub fn ld_b_d8(&mut self) -> u8 {
        let val = self.read_u8_at_pc_and_increase();
        self.registers.b = val;
        2
    }

    /// OPCode: 0x07
    /// Mnemonic: RLCA
    pub fn rlca(&mut self) -> u8 {
        let a = self.registers.a;
        self.registers.set_flag_c(((a & 0b10000000) >> 7) == 1);
        self.registers.a = a.rotate_left(1);
        self.registers.set_flag_z(false);
        self.registers.set_flag_n(false);
        self.registers.set_flag_h(false);
        2
    }

    /// OPCode: 0x08
    /// Mnemonic: LD (a16), SP
    pub fn ld_a16p_sp(&mut self) -> u8 {
        let address = self.read_u16_at_pc_and_increase();
        let sp = self.registers.get_sp();
        self.write_u16(address, sp);
        5
    }

    /// OPCode: 0x09
    /// Mnemonic: ADD HL, BC
    pub fn add_hl_bc(&mut self) -> u8 {
        let bc = self.registers.get_bc();
        self.add16(bc);
        2
    }

    /// OPCode: 0x13
    /// Mnemonic: INC DE
    pub fn inc_de(&mut self) -> u8 {
        let r = self.registers.get_de();
        let res = self.inc16(r);
        self.registers.set_de(res);
        2
    }
    
    /// OPCode: 0x19
    /// Mnemonic: ADD HL, DE
    pub fn add_hl_de(&mut self) -> u8 {
        let de = self.registers.get_de();
        self.add16(de);
        2
    }

    /// OPCode: 0x1D
    /// Mnemonic: DEC E
    pub fn dec_e(&mut self) -> u8 {
        self.registers.e = self.dec8(self.registers.e);
        1
    }

    /// OPCode: 0x20
    /// Mnemonic: JR NZ, r8
    pub fn jr_nz_r8(&mut self) -> u8 {
        let val = self.read_u16_at_pc_and_increase();
        if !self.registers.get_flag_z() {
            self.jr(val);
            return 3;
        }
        2
    }



    /// OPCode: 0x21
    /// Mnemonic: LD HL, d16
    pub fn ld_hl_d16(&mut self) -> u8 {
        let val = self.read_u16_at_pc_and_increase();
        self.registers.set_hl(val);
        3
    }

    /// OPCode: 0x22
    /// Mnemonic: LD (HL+), A
    pub fn ld_hlp_inc_a(&mut self) -> u8 {
        let hl = self.registers.get_hl();
        self.write_u8(hl, self.registers.a);
        self.registers.set_hl(hl + 1);
        2
    }




    /// OPCode: 0x23
    /// Mnemonic: INC HL
    pub fn inc_hl(&mut self) -> u8 {
        let r = self.registers.get_hl();
        let res = self.inc16(r);
        self.registers.set_hl(res);
        2
    }


    /// OPCode: 0x29
    /// Mnemonic: ADD HL, HL
    pub fn add_hl_hl(&mut self) -> u8 {
        let hl = self.registers.get_hl();
        self.add16(hl);
        2
    }

    /// OPCode: 0x2C
    /// Mnemonic: INC L
    pub fn inc_l(&mut self) -> u8 {
        self.registers.l = self.inc8(self.registers.l);
        1
    }


    /// OPCode: 0x32
    /// Mnemonic: INC Sd16P
    pub fn ld_hlp_dec_a(&mut self) -> u8 {
        let hl = self.registers.get_hl();
        self.write_u8(hl, self.registers.a);
        self.registers.set_hl(hl - 1);
        2
    }


    /// OPCode: 0x33
    /// Mnemonic: INC Sd16P
    pub fn inc_sp(&mut self) -> u8 {
        let r = self.registers.get_sp();
        let res = self.inc16(r);
        self.registers.set_sp(res);
        2
    }

    /// OPCode: 0x3E
    /// Mnemonic: LD A, d8 
    pub fn ld_a_d8(&mut self) -> u8 {
        self.registers.a = self.read_u8_at_pc_and_increase();
        2
    }

    /// OPCode: 0x40
    /// Mnemonic: LD B, B
    pub fn ld_b_b(&mut self) -> u8 {
        self.registers.b = self.registers.b;
        1
    }

    /// OPCode: 0x41
    /// Mnemonic: LD B, C
    pub fn ld_b_c(&mut self) -> u8 {
        self.registers.b = self.registers.c;
        1
    }

    /// OPCode: 0x42
    /// Mnemonic: LD B, D
    pub fn ld_b_d(&mut self) -> u8 {
        self.registers.b = self.registers.d;
        1
    }

    /// OPCode: 0x43
    /// Mnemonic: LD B, E
    pub fn ld_b_e(&mut self) -> u8 {
        self.registers.b = self.registers.e;
        1
    }

    /// OPCode: 0x44
    /// Mnemonic: LD B, H
    pub fn ld_b_h(&mut self) -> u8 {
        self.registers.b = self.registers.h;
        1
    }

    /// OPCode: 0x45
    /// Mnemonic: LD B, L
    pub fn ld_b_l(&mut self) -> u8 {
        self.registers.b = self.registers.l;
        1
    }

    /// OPCode: 0x46
    /// Mnemonic: LD B, (HL)
    pub fn ld_b_hlp(&mut self) -> u8 {
        self.registers.b = self.read(self.registers.get_hl());
        2
    }

    /// OPCode: 0x47
    /// Mnemonic: LD B, A
    pub fn ld_b_a(&mut self) -> u8 {
        self.registers.b = self.registers.a;
        1
    }

    /// OPCode: 0x48
    /// Mnemonic: LD C, B
    pub fn ld_c_b(&mut self) -> u8 {
        self.registers.c = self.registers.b;
        1
    }

    /// OPCode: 0x49
    /// Mnemonic: LD C, C
    pub fn ld_c_c(&mut self) -> u8 {
        1
    }

    /// OPCode: 0x4A
    /// Mnemonic: LD C, D
    pub fn ld_c_d(&mut self) -> u8 {
        self.registers.c = self.registers.d;
        1
    }

    /// OPCode: 0x4B
    /// Mnemonic: LD C, E
    pub fn ld_c_e(&mut self) -> u8 {
        self.registers.c = self.registers.e;
        1
    }

    /// OPCode: 0x4C
    /// Mnemonic: LD C, H
    pub fn ld_c_h(&mut self) -> u8 {
        self.registers.c = self.registers.h;
        1
    }

    /// OPCode: 0x4D
    /// Mnemonic: LD C, L
    pub fn ld_c_l(&mut self) -> u8 {
        self.registers.c = self.registers.l;
        1
    }

    /// OPCode: 0x4E
    /// Mnemonic: LD C, (HL)
    pub fn ld_c_hlp(&mut self) -> u8 {
        self.registers.c = self.read(self.registers.get_hl());
        2
    }

    /// OPCode: 0x4F
    /// Mnemonic: LD C, A
    pub fn ld_c_a(&mut self) -> u8 {
        self.registers.c = self.registers.a;
        1
    }

    /// OPCode: 0x50
    /// Mnemonic: LD D, B
    pub fn ld_d_b(&mut self) -> u8 {
        1
    }

    /// OPCode: 0x51
    /// Mnemonic: LD D, C
    pub fn ld_d_c(&mut self) -> u8 {
        self.registers.b = self.registers.c;
        1
    }

    /// OPCode: 0x52
    /// Mnemonic: LD D, D
    pub fn ld_d_d(&mut self) -> u8 {
        self.registers.b = self.registers.d;
        1
    }

    /// OPCode: 0x53
    /// Mnemonic: LD D, E
    pub fn ld_d_e(&mut self) -> u8 {
        self.registers.b = self.registers.e;
        1
    }

    /// OPCode: 0x54
    /// Mnemonic: LD D, H
    pub fn ld_d_h(&mut self) -> u8 {
        self.registers.b = self.registers.h;
        1
    }

    /// OPCode: 0x55
    /// Mnemonic: LD D, L
    pub fn ld_d_l(&mut self) -> u8 {
        self.registers.b = self.registers.l;
        1
    }

    /// OPCode: 0x56
    /// Mnemonic: LD D, (HL)
    pub fn ld_d_hlp(&mut self) -> u8 {
        self.registers.b = self.read(self.registers.get_hl());
        2
    }

    /// OPCode: 0x57
    /// Mnemonic: LD D, A
    pub fn ld_d_a(&mut self) -> u8 {
        self.registers.b = self.registers.a;
        1
    }

    /// OPCode: 0x58
    /// Mnemonic: LD E, B
    pub fn ld_e_b(&mut self) -> u8 {
        self.registers.e = self.registers.b;
        1
    }

    /// OPCode: 0x59
    /// Mnemonic: LD E, C
    pub fn ld_e_c(&mut self) -> u8 {
        self.registers.e = self.registers.c;
        1
    }

    /// OPCode: 0x5A
    /// Mnemonic: LD E, D
    pub fn ld_e_d(&mut self) -> u8 {
        self.registers.e = self.registers.d;
        1
    }

    /// OPCode: 0x5B
    /// Mnemonic: LD E, E
    pub fn ld_e_e(&mut self) -> u8 {
        1
    }

    /// OPCode: 0x5C
    /// Mnemonic: LD E, H
    pub fn ld_e_h(&mut self) -> u8 {
        self.registers.e = self.registers.h;
        1
    }

    /// OPCode: 0x5D
    /// Mnemonic: LD E, L
    pub fn ld_e_l(&mut self) -> u8 {
        self.registers.e = self.registers.l;
        1
    }

    /// OPCode: 0x5E
    /// Mnemonic: LD E, (HL)
    pub fn ld_e_hlp(&mut self) -> u8 {
        self.registers.e = self.read(self.registers.get_hl());
        2
    }

    /// OPCode: 0x5F
    /// Mnemonic: LD E, A
    pub fn ld_e_a(&mut self) -> u8 {
        self.registers.e = self.registers.a;
        1
    }

    /// OPCode: 0xAF
    /// Mnemonic: XOR A
    pub fn xor_a(&mut self) -> u8 {
        let val = self.registers.a;
        self.xor(val);
        1
    }

    /// OPCode: 0x60
    /// Mnemonic: LD H, B
    pub fn ld_h_b(&mut self) -> u8 {
        self.registers.h = self.registers.b;
        1
    }

    /// OPCode: 0x61
    /// Mnemonic: LD H, C
    pub fn ld_h_c(&mut self) -> u8 {
        self.registers.h = self.registers.c;
        1
    }

    /// OPCode: 0x62
    /// Mnemonic: LD H, D
    pub fn ld_h_d(&mut self) -> u8 {
        self.registers.h = self.registers.d;
        1
    }

    /// OPCode: 0x63
    /// Mnemonic: LD H, E
    pub fn ld_h_e(&mut self) -> u8 {
        self.registers.h = self.registers.e;
        1
    }

    /// OPCode: 0x64
    /// Mnemonic: LD H, H
    pub fn ld_h_h(&mut self) -> u8 {
        1
    }

    /// OPCode: 0x65
    /// Mnemonic: LD H, L
    pub fn ld_h_l(&mut self) -> u8 {
        self.registers.h = self.registers.l;
        1
    }

    /// OPCode: 0x66
    /// Mnemonic: LD H, (HL)
    pub fn ld_h_hlp(&mut self) -> u8 {
        self.registers.h = self.read(self.registers.get_hl());
        2
    }

    /// OPCode: 0x67
    /// Mnemonic: LD H, A
    pub fn ld_h_a(&mut self) -> u8 {
        self.registers.h = self.registers.a;
        1
    }

    /// OPCode: 0x68
    /// Mnemonic: LD L, B
    pub fn ld_l_b(&mut self) -> u8 {
        self.registers.l = self.registers.b;
        1
    }

    /// OPCode: 0x69
    /// Mnemonic: LD L, C
    pub fn ld_l_c(&mut self) -> u8 {
        self.registers.l = self.registers.c;
        1
    }

    /// OPCode: 0x6A
    /// Mnemonic: LD L, D
    pub fn ld_l_d(&mut self) -> u8 {
        self.registers.l = self.registers.d;
        1
    }

    /// OPCode: 0x6B
    /// Mnemonic: LD L, E
    pub fn ld_l_e(&mut self) -> u8 {
        self.registers.l = self.registers.e;
        1
    }

    /// OPCode: 0x6C
    /// Mnemonic: LD L, H
    pub fn ld_l_h(&mut self) -> u8 {
        self.registers.l = self.registers.h;
        1
    }

    /// OPCode: 0x6D
    /// Mnemonic: LD L, L
    pub fn ld_l_l(&mut self) -> u8 {
        1
    }

    /// OPCode: 0x6E
    /// Mnemonic: LD L, (HL)
    pub fn ld_l_hlp(&mut self) -> u8 {
        self.registers.l = self.read(self.registers.get_hl());
        2
    }

    /// OPCode: 0x6F
    /// Mnemonic: LD L, A
    pub fn ld_l_a(&mut self) -> u8 {
        self.registers.l = self.registers.a;
        1
    }

    /// OPCode: 0x70
    /// Mnemonic: LD (HL), B
    pub fn ld_hlp_b(&mut self) -> u8 {
        self.write_u8(self.registers.get_hl(), self.registers.b);
        2
    }

    /// OPCode: 0x71
    /// Mnemonic: LD (HL), C
    pub fn ld_hlp_c(&mut self) -> u8 {
        self.write_u8(self.registers.get_hl(), self.registers.c);
        2
    }

    /// OPCode: 0x72
    /// Mnemonic: LD (HL), D
    pub fn ld_hlp_d(&mut self) -> u8 {
        self.write_u8(self.registers.get_hl(), self.registers.d);
        2
    }

    /// OPCode: 0x73
    /// Mnemonic: LD (HL), E
    pub fn ld_hlp_e(&mut self) -> u8 {
        self.write_u8(self.registers.get_hl(), self.registers.e);
        2
    }

    /// OPCode: 0x74
    /// Mnemonic: LD (HL), H
    pub fn ld_hlp_h(&mut self) -> u8 {
        self.write_u8(self.registers.get_hl(), self.registers.h);
        2
    }

    /// OPCode: 0x75
    /// Mnemonic: LD (HL), L
    pub fn ld_hlp_l(&mut self) -> u8 {
        self.write_u8(self.registers.get_hl(), self.registers.l);
        2
    }

    /// OPCode: 0x77
    /// Mnemonic: LD (HL), A
    pub fn ld_hlp_a(&mut self) -> u8 {
        self.write_u8(self.registers.get_hl(), self.registers.a);
        2
    }

    /// OPCode: 0x78
    /// Mnemonic: LD A, B
    pub fn ld_a_b(&mut self) -> u8 {
        self.registers.a = self.registers.b;
        1
    }

    /// OPCode: 0x79
    /// Mnemonic: LD A, C
    pub fn ld_a_c(&mut self) -> u8 {
        self.registers.a = self.registers.c;
        1
    }

    /// OPCode: 0x7A
    /// Mnemonic: LD A, D
    pub fn ld_a_d(&mut self) -> u8 {
        self.registers.a = self.registers.d;
        1
    }

    /// OPCode: 0x7B
    /// Mnemonic: LD A, E
    pub fn ld_a_e(&mut self) -> u8 {
        self.registers.a = self.registers.e;
        1
    }

    /// OPCode: 0x7C
    /// Mnemonic: LD A, H
    pub fn ld_a_h(&mut self) -> u8 {
        self.registers.a = self.registers.h;
        1
    }

    /// OPCode: 0x7D
    /// Mnemonic: LD A, L
    pub fn ld_a_l(&mut self) -> u8 {
        self.registers.a = self.registers.l;
        1
    }

    /// OPCode: 0x7E
    /// Mnemonic: LD A, (HL)
    pub fn ld_a_hlp(&mut self) -> u8 {
        self.registers.a = self.read(self.registers.get_hl());
        2
    }

    /// OPCode: 0x7F
    /// Mnemonic: LD A, A
    pub fn ld_a_a(&mut self) -> u8 {
        1
    }
    /// OPCode: 0xC3
    /// Mnemonic: JP
    pub fn jp_a16(&mut self) -> u8 {
        let address = self.read_u16_at_pc_and_increase();
        self.jp(address);
        4
    }
    
    /// OPCode: 0x0E
    /// Mnemonic: LD C, d8
    pub fn ld_c_d8(&mut self) -> u8 {
        self.registers.c = self.read_u8_at_pc_and_increase();
        4
    }

    // 0xCB extended set:

    /// 0xCBFE
    /// Mnemonic: SET 7, (HL)
    pub fn set_7_hlp(&mut self) -> u8 {
        let hl = self.registers.get_hl();
        let val = self.read(hl);
        let res = self.set_nth_bit(7, val);
        self.write_u8(hl, res);
        3
    }

}

#[cfg(test)]
mod tests {
    use crate::cpu::Cpu;

    #[test]
    fn nop() {
        let mut cpu: Cpu = Cpu::new();
        let expected_cpu = cpu;
        let cycles_needed = cpu.nop();
        assert_eq!(cycles_needed, 1);
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    fn ld_bc() {
        // 0xAEFE
        let expected_b: u8 = 0xAE;
        let expected_c: u8 = 0xFE;
        let mut cpu: Cpu = Cpu::new();

        // Keep big endianess in mind.
        cpu.wram[0x100] = expected_c;
        cpu.wram[0x101] = expected_b;

        let mut expected_cpu = cpu;
        let cycles_needed = cpu.ld_bc_d16();

        expected_cpu.registers.b = expected_b;
        expected_cpu.registers.c = expected_c;
        expected_cpu.registers.pc += 2;

        assert_eq!(cycles_needed, 3);
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    fn ld_b_d8() {
        let mut cpu = Cpu::new();
        cpu.wram[0x100] = 0xAE;
        cpu.ld_b_d8();
        assert_eq!(cpu.registers.b, 0xAE);
    }

    #[test]
    fn rlca() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0b10000001;
        cpu.rlca();
        assert_eq!(cpu.registers.a, 0b00000011);
        assert_eq!(cpu.registers.get_flag_c(), true);
        cpu.registers.a = 0b00000001;
        cpu.rlca();
        assert_eq!(cpu.registers.a, 0b00000010);
        assert_eq!(cpu.registers.get_flag_c(), false);
        assert_eq!(cpu.registers.get_flag_z(), false);
        assert_eq!(cpu.registers.get_flag_n(), false);
        assert_eq!(cpu.registers.get_flag_h(), false);
    }

    #[test]
    fn ld_a16p_sp() {
        let mut cpu = Cpu::new();
        cpu.registers.sp = 0xBEEF;
        cpu.wram[0x100] = 0x20;
        cpu.wram[0x101] = 0x25;
        cpu.ld_a16p_sp();
        assert_eq!(cpu.wram[0x2520], 0xEF);
        assert_eq!(cpu.wram[0x2521], 0xBE);
    }
}
