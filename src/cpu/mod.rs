use std::rc::Rc;

use crate::mmu::Mmu;

use self::registers::Registers;
pub mod disassembler;
mod extended_instructions;
mod instructions;

pub mod registers;
pub mod utils;

/**
* Emulating the LR35902 CPU
*
* For Opcodes see: <https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html>
*/

#[derive(Debug, Clone)]
pub struct Cpu {
    pub registers: Registers,

    pub busy_for: u8,
    pub mmu: Mmu,
    pub halted: bool,
}

impl Cpu {
    /// Initialize cpu memory
    pub fn new() -> Cpu {
        tracing::info!("initializing cpu");
        Cpu {
            registers: registers::Registers::new(),
            mmu: Mmu::new(),
            busy_for: 0x00,
            halted: false,
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
        self.mmu.cycle(); // Not sure if this is the right place
    }

    /// Push a u8 value onto the stack.
    pub fn push_stack_u8(&mut self, val: u8) {
        self.registers.sp = self.registers.sp.wrapping_sub(1);
        self.mmu.write_u8(self.registers.sp, val);
    }

    /// Pop a u8 value from the stack.
    pub fn pop_stack_u8(&mut self) -> u8 {
        let val = self.mmu.read(self.registers.sp);
        self.registers.sp = self.registers.sp.wrapping_add(1);
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

    /// Reads a byte from memory at pc and increases pc by one.
    pub fn read_u8_at_pc_and_increase(&mut self) -> u8 {
        let val = self.mmu.read(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        val
    }

    /// Reads a byte from memory at pc.
    pub fn read_u8_at_pc(&self) -> u8 {
        self.mmu.read(self.registers.pc)
    }

    /// Reads two bytes from memory at pc.
    pub fn read_u16_at_pc(&self) -> u16 {
        let a = self.read_u8_at_pc();
        let b = self.read_u8_at_pc();
        // Little endian in memory
        utils::merge_u8s(b, a)
    }

    /// Reads two bytes from memory at pc and increases pc by two.
    pub fn read_u16_at_pc_and_increase(&mut self) -> u16 {
        let a = self.read_u8_at_pc_and_increase();
        let b = self.read_u8_at_pc_and_increase();

        // Little endian in memory
        utils::merge_u8s(b, a)
    }

    /// Check for u8 half carries on additions. (carry from 3rd to 4th bit).
    pub fn check_add_u8_hc(left: u8, right: u8) -> bool {
        ((left & 0xf).wrapping_add(right & 0xf)) & 0x10 == 0x10
    }

    /// Check for u8 half carries on additions. (carry from 7th to 8th bit).
    pub fn check_add_u16_hc(left: u16, right: u16) -> bool {
        ((left & 0xff).wrapping_add(right & 0xff)) & 0x100 == 0x100
    }

    /// Check for u8 half carries on subtractions. (carry from 3rd to 4th bit).
    pub fn check_sub_u8_hc(left: u8, right: u8) -> bool {
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
