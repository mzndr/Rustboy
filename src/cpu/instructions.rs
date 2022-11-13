use super::Cpu;

impl Cpu {
    pub fn exec_instruction(&mut self, opcode: u8) -> u8 {
        return match opcode {
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

            0x13 => self.inc_de(),
            0x23 => self.inc_hl(),
            0x33 => self.inc_sp(),

            _ => 0,
        };
    }

    /// OPCode: 0x00
    /// Mnenonic: NOP
    pub fn nop(&mut self) -> u8 {
        return 1;
    }

    /// OPCode: 0x01
    /// Mnenonic: LD BC, d16
    pub fn ld_bc_d16(&mut self) -> u8 {
        let val = self.read_u16_at_pc_and_increase();
        self.registers.set_bc(val);
        return 3;
    }

    /// OPCode: 0x02
    /// Mnenonic: LD (BC), A
    pub fn ld_bcp_a(&mut self) -> u8 {
        let address = self.registers.get_bc();
        let val = self.registers.a;
        self.write_u8(address, val);
        return 2;
    }

    /// OPCode: 0x03
    /// Mnenonic: INC BC
    pub fn inc_bc(&mut self) -> u8 {
        let r = self.registers.get_bc();
        let res = self.inc16(r);
        self.registers.set_bc(res);
        return 2;
    }

    /// OPCode: 0x04
    /// Mnenonic: INC B
    pub fn inc_b(&mut self) -> u8 {
        self.registers.b = self.inc8(self.registers.b);
        return 1;
    }

    /// OPCode: 0x05
    /// Mnenonic: DEC B
    pub fn dec_b(&mut self) -> u8 {
        self.registers.b = self.dec8(self.registers.b);
        return 1;
    }

    /// OPCode: 0x06
    /// Mnenonic: LD B, d8
    pub fn ld_b_d8(&mut self) -> u8 {
        let val = self.read_u8_at_pc_and_increase();
        self.registers.b = val;
        return 2;
    }

    /// OPCode: 0x07
    /// Mnenonic: RLCA
    pub fn rlca(&mut self) -> u8 {
        let a = self.registers.a;
        self.registers.set_flag_c(((a & 0b10000000) >> 7) == 1);
        self.registers.a = a.rotate_left(1);
        self.registers.set_flag_z(false);
        self.registers.set_flag_n(false);
        self.registers.set_flag_h(false);
        return 2;
    }

    /// OPCode: 0x08
    /// Mnenonic: LD (a16), SP
    pub fn ld_a16p_sp(&mut self) -> u8 {
        let address = self.read_u16_at_pc_and_increase();
        let sp = self.registers.get_sp();
        self.write_u16(address, sp);
        return 5;
    }

    /// OPCode: 0x09
    /// Mnenonic: ADD HL, BC
    pub fn add_hl_bc(&mut self) -> u8 {
        self.add16(self.registers.get_bc());
        return 2;
    }

    /// OPCode: 0x13
    /// Mnenonic: INC DE
    pub fn inc_de(&mut self) -> u8 {
        let r = self.registers.get_de();
        let res = self.inc16(r);
        self.registers.set_de(res);
        return 2;
    }

    /// OPCode: 0x23
    /// Mnenonic: INC HL
    pub fn inc_hl(&mut self) -> u8 {
        let r = self.registers.get_hl();
        let res = self.inc16(r);
        self.registers.set_hl(res);
        return 2;
    }

    /// OPCode: 0x33
    /// Mnenonic: INC SP
    pub fn inc_sp(&mut self) -> u8 {
        let r = self.registers.get_sp();
        let res = self.inc16(r);
        self.registers.set_sp(res);
        return 2;
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::Cpu;

    #[test]
    fn nop() {
        let mut cpu: Cpu = Cpu::new();
        let expected_cpu = cpu.clone();
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

        let mut expected_cpu = cpu.clone();
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
