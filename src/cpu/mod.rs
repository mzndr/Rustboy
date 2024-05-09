use self::registers::Registers;
mod extended_instructions;
mod instructions;

pub mod registers;
pub mod utils;

/**
* Emulating the LR35902 CPU
*
* For Opcodes see: <https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html>
*/

/** Working RAM **/
// TODO: Fixen, es ist nicht WRAM sondern der gesammte memory.
const WRAM_SIZE: usize = 0x10000; //0x20 * 0x400;
type WRam = [u8; WRAM_SIZE];

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Cpu {
    pub registers: Registers,

    busy_for: u8,
    wram: WRam,
}

impl Cpu {
    /// Checks if an address is in valid space,
    /// prints an error message and quits if not.
    fn check_address(address: u16) {
        assert!((address as usize).lt(&WRAM_SIZE));
    }

    /// Needs to be changed for bigger games, since they
    /// are too big to fit into ram, so banking has to be
    /// implemented.
    pub fn load_rom(&mut self, rom: &[u8]) {
        for (address, byte) in rom.iter().enumerate() {
            self.wram[address] = *byte;
        }
    }

    /// Initialize cpu memory
    pub fn new() -> Cpu {
        tracing::info!("initializing cpu");
        Cpu {
            registers: registers::Registers::new(),
            wram: [0x00; WRAM_SIZE],
            busy_for: 0x00,
        }
    }

    // Execute a machine cycle.
    #[tracing::instrument(skip(self), fields(regs = %self.registers))]
    pub fn cycle(&mut self) {
        if self.busy_for == 0 {
            self.busy_for = self.exec_instruction();
        } else {
            self.busy_for -= 1;
        }
    }

    /// Push a u8 value onto the stack.
    pub fn push_stack_u8(&mut self, val: u8) {
        self.registers.sp -= 1;
        self.write_u8(self.registers.sp, val);
    }

    /// Pop a u8 value from the stack.
    pub fn pop_stack_u8(&mut self) -> u8 {
        let val = *self.read(self.registers.sp);
        self.registers.sp += 1;
        val
    }

    /// Push a u16 value onto the stack.
    pub fn push_stack_u16(&mut self, val: u16) {
        let (l, h) = utils::split_u16(val);
        self.push_stack_u8(h);
        self.push_stack_u8(l);
    }

    /// Pop a u16 value from the stack.
    pub fn pop_stack_u16(&mut self) -> u16 {
        let l = self.pop_stack_u8();
        let h = self.pop_stack_u8();
        utils::merge_u8s(l, h)
    }

    /// Reads from wram at address.
    pub fn read_mut(&mut self, address: u16) -> &mut u8 {
        let u_addr = address as usize;
        Self::check_address(address);
        &mut self.wram[u_addr]
    }

    /// Reads from wram at address.
    pub fn read(&self, address: u16) -> &u8 {
        let u_addr = address as usize;
        Self::check_address(address);
        &self.wram[u_addr]
    }

    /// Writes u8 to wram at address.
    pub fn write_u8(&mut self, address: u16, val: u8) {
        let u_addr = address as usize;
        Self::check_address(address);
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
        let val = *self.read(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        val
    }

    /// Reads two bytes from wram at pc and increases pc by two.
    pub fn read_u16_at_pc_and_increase(&mut self) -> u16 {
        let a = self.read_u8_at_pc_and_increase();
        let b = self.read_u8_at_pc_and_increase();

        // Little endian in memory
        utils::merge_u8s(b, a)
    }

    /// Do nothing.
    pub fn nop() -> u8 {
        1
    }

    /// Return from function
    pub fn ret(&mut self) -> u8 {
        self.registers.sp = self.pop_stack_u16();
        4
    }

    /// Wrappingly increase a 16 bit value by one.
    pub fn rst(&mut self, address: u16) -> u8 {
        self.push_stack_u16(self.registers.get_sp() + 1);
        self.registers.set_sp(address);
        3
    }

    /// Wrappingly increase a 16 bit value by one.
    pub fn inc16(val: u16) -> u16 {
        val.wrapping_add(1)
    }

    /// Wrappingly decrease a 16 bit value by one.
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
        self.call(addr);
        6
    }

    pub fn call_nz_a16(&mut self) -> u8 {
        if self.registers.get_flag_z() {
            return 4;
        }
        self.call_a16()
    }

    pub fn call_z_a16(&mut self) -> u8 {
        if !self.registers.get_flag_z() {
            return 4;
        }
        self.call_a16()
    }

    pub fn call_nc_a16(&mut self) -> u8 {
        if self.registers.get_flag_c() {
            return 4;
        }
        self.call_a16()
    }

    pub fn call_c_a16(&mut self) -> u8 {
        if !self.registers.get_flag_c() {
            return 4;
        }
        self.call_a16()
    }

    /// Load src into dst.
    pub fn ld(src: u8, dst: &mut u8) -> u8 {
        *dst = src;
        1
    }

    pub fn inc8(&mut self, val: u8) -> u8 {
        let w = val.wrapping_add(1);
        self.registers.set_flag_h(Self::check_add_u8_hc(w, 1));
        self.registers.set_flag_z(w == 0);
        self.registers.set_flag_n(false);
        w
    }

    /// Flip all bytes in `a` register.
    pub fn cpl(&mut self) -> u8 {
        self.registers.set_flag_n(true);
        self.registers.set_flag_h(true);
        self.registers.a ^= 0xFF;
        1
    }

    /// Flip carry flag and unset `n` and `h` flags.
    pub fn ccf(&mut self) -> u8 {
        self.registers.set_flag_n(false);
        self.registers.set_flag_h(false);
        self.registers.set_flag_c(!self.registers.get_flag_c());
        1
    }

    /// Set carry flag and unset `n` and `h` flags.
    pub fn scf(&mut self) -> u8 {
        self.registers.set_flag_n(false);
        self.registers.set_flag_h(false);
        self.registers.set_flag_c(true);
        1
    }

    pub fn dec8(&mut self, val: u8) -> u8 {
        let w = val.wrapping_sub(1);
        self.registers.set_flag_h(Self::check_sub_u8_hc(w, 1));
        self.registers.set_flag_z(w == 0);
        self.registers.set_flag_n(true);
        w
    }

    /// Adds a value with HL and stores it in HL.
    pub fn add16(&mut self, val: u16) {
        let hl = self.registers.get_hl();
        let sum = val.wrapping_add(hl);
        self.registers.set_flag_h(Self::check_add_u16_hc(val, hl));
        self.registers
            .set_flag_c(u32::from(val) + u32::from(hl) > 0xFFFF);
        self.registers.set_flag_n(false);
        self.registers.set_hl(sum);
    }

    // 8 bit addition with carry flag and value
    pub fn add8c(&mut self, val: u8) -> u8 {
        self.add8(val.wrapping_add(self.registers.get_flag_c().into()));

        1
    }

    // 8 bit sub with carry flag and value
    pub fn sub8c(&mut self, val: u8) -> u8 {
        self.sub8(val.wrapping_sub(self.registers.get_flag_c().into()));

        1
    }

    /// Adds two value with A, sets flags, and stores result in A
    pub fn add8(&mut self, val: u8) -> u8 {
        let a = self.registers.a;
        let result = a.wrapping_add(val);
        self.registers.set_flag_h(result & 0xf == 0xf);
        self.registers.set_flag_z(result == 0);
        self.registers.set_flag_n(false);
        self.registers
            .set_flag_c(u16::from(val) + u16::from(a) > 0xFF);
        self.registers.a = result;

        1
    }

    /// Subs two 8 bit integers and sets flags and sotres result in A
    pub fn sub8(&mut self, val: u8) -> u8 {
        let a = self.registers.a;
        let result = a.wrapping_sub(val);
        self.registers.set_flag_h(Self::check_sub_u8_hc(a, val));
        self.registers.set_flag_z(result == 0);
        self.registers.set_flag_n(true);
        self.registers.set_flag_c(u16::from(val) < u16::from(a));
        self.registers.a = result;

        1
    }

    /// Absolute jump by setting PC to address
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

    /// Relative jump by adding val to PC
    pub fn jr(&mut self, val: u8) {
        self.registers.pc = self.registers.pc.wrapping_add(val.into());
    }

    /// Xors value with a register and sets flags.
    pub fn xor(&mut self, val: u8) -> u8 {
        self.registers.a ^= val;
        self.registers.set_flag_z(self.registers.a == 0);
        self.registers.set_flag_n(false);
        self.registers.set_flag_h(false);
        self.registers.set_flag_c(false);

        1
    }

    /// And value with a register and sets flags.
    pub fn and(&mut self, val: u8) -> u8 {
        self.registers.a &= val;
        self.registers.set_flag_z(self.registers.a == 0);
        self.registers.set_flag_n(false);
        self.registers.set_flag_h(true);
        self.registers.set_flag_c(false);

        1
    }

    /// Or value with a register and sets flags.
    pub fn or(&mut self, val: u8) -> u8 {
        self.registers.a |= val;
        self.registers.set_flag_z(self.registers.a == 0);
        self.registers.set_flag_n(false);
        self.registers.set_flag_h(false);
        self.registers.set_flag_c(false);

        1
    }

    /// Compare with `a`, basicly a sub operation without setting `a`.
    pub fn cp(&mut self, val: u8) -> u8 {
        let a = self.registers.a;
        let result = a.wrapping_sub(val);
        self.registers.set_flag_h(Self::check_sub_u8_hc(a, val));
        self.registers.set_flag_z(result == 0);
        self.registers.set_flag_n(true);
        self.registers.set_flag_c(u16::from(val) < u16::from(a));

        1
    }

    /// Check for u8 half carries on additions. (carry from 3rd to 4th bit).
    fn check_add_u8_hc(left: u8, right: u8) -> bool {
        ((left & 0xf).wrapping_add(right & 0xf)) & 0x10 == 0x10
    }

    /// Check for u8 half carries on additions. (carry from 7th to 8th bit).
    fn check_add_u16_hc(left: u16, right: u16) -> bool {
        ((left & 0xff).wrapping_add(right & 0xff)) & 0x100 == 0x100
    }

    /// Check for u8 half carries on subtractions. (carry from 3rd to 4th bit).
    fn check_sub_u8_hc(left: u8, right: u8) -> bool {
        ((left & 0xf).wrapping_sub(right & 0xf)) & 0x10 == 0x10
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::Cpu;

    #[test]
    fn test_check_add_u8_hc() {
        assert!(Cpu::check_add_u8_hc(1, 0xF));
        assert!(!Cpu::check_add_u8_hc(1, 0xE));
    }

    #[test]
    fn test_check_add_u16_hc() {
        assert!(Cpu::check_add_u16_hc(0xFF, 1));
        assert!(!Cpu::check_add_u16_hc(0xFE, 1));
    }

    #[test]
    fn test_check_sub_u8_hc() {
        assert!(Cpu::check_sub_u8_hc(1, 0xF));
        assert!(!Cpu::check_sub_u8_hc(0xF, 0xE));
    }

}
