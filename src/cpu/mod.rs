use crate::{mmu::Mmu, ppu};

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

/// Offset to interrupt enable register in WRAM.
pub const WRAM_IE_OFFSET: u16 = 0xFFFF;

/// Offset to interrupt flag register in WRAM.
pub const WRAM_IF_OFFSET: u16 = 0xFF0F;

/// Struct representing the CPU, holding its state and implementation.
pub struct Cpu {
    pub registers: Registers,

    pub busy_for: u8,
    pub mmu: Mmu,
    pub halted: bool,
    debug: crate::debug::Debug,

    schedule_ei: bool,

    pub cycle: u128,
}

/// Different kinds of interrupt(-sources).
#[derive(Debug, Clone, Copy)]
enum Interrupt {
    VBlank,
    LCD,
    Timer,
    Serial,
    Joypad,
}

impl Interrupt {
    /// Checks if the interrupt bit is set in the given `u8`.
    fn is_set(self, reg_val: u8) -> bool {
        (reg_val >> self.bit_index() & 1) == 1
    }

    /// Gets this interrupts bit index.
    fn bit_index(self) -> u8 {
        match self {
            Self::VBlank => 0,
            Self::LCD => 1,
            Self::Timer => 2,
            Self::Serial => 3,
            Self::Joypad => 4,
        }
    }

    /// Gets this interrupts handler address.
    fn handler_address(self) -> u16 {
        match self {
            Self::VBlank => 0x40,
            Self::LCD => 0x48,
            Self::Timer => 0x50,
            Self::Serial => 0x58,
            Self::Joypad => 0x60,
        }
    }

    fn enumerate() -> [Self; 5] {
        [
            Self::VBlank,
            Self::LCD,
            Self::Timer,
            Self::Serial,
            Self::Joypad,
        ]
    }
}

impl Cpu {
    /// Initialize cpu memory
    pub fn new(rom: &[u8], debug: crate::debug::Debug) -> Cpu {
        tracing::info!("initializing cpu");
        Cpu {
            registers: registers::Registers::new(),
            mmu: Mmu::new(rom, debug.clone()),
            busy_for: 0x00,
            halted: false,
            schedule_ei: false,

            cycle: 0,

            debug,
        }
    }

    /// Request interrupt for an interrupt source.
    fn request_interrupt(&mut self, source: Interrupt, val: bool) {
        let old_if = self.mmu.read_u8(WRAM_IF_OFFSET);
        let new_if = utils::set_bit(old_if, source.bit_index(), val);
        self.mmu.write_u8(WRAM_IF_OFFSET, new_if);
    }

    /// Enable interrupt for an interrupt source.
    fn enable_interrupt(&mut self, source: Interrupt, val: bool) {
        let old_if = self.mmu.read_u8(WRAM_IE_OFFSET);
        let new_if = utils::set_bit(old_if, source.bit_index(), val);
        self.mmu.write_u8(WRAM_IE_OFFSET, new_if);
    }

    /// Handle interrupts.
    fn handle_interrupts(&mut self) {
        if !self.mmu.ime {
            return;
        }

        self.mmu.ime = false;
        for source in Interrupt::enumerate() {
            if source.is_set(self.mmu.ie) && source.is_set(self.mmu.read_u8(WRAM_IF_OFFSET)) {
                self.halted = false;
                self.mmu.write_u8(
                    WRAM_IF_OFFSET,
                    utils::set_bit(self.mmu.read_u8(WRAM_IF_OFFSET), source.bit_index(), false),
                );

                tracing::debug!("handling interrupt: {source:?}");
                self.call(source.handler_address());
                break;
            }
        }
        self.busy_for += 5;
    }

    // Execute a machine cycle.
    #[tracing::instrument(skip(self), fields(regs = %self.registers))]
    pub fn cycle(&mut self) {
        if self.busy_for == 0 {
            self.busy_for = self.exec_instruction();
            self.handle_interrupts();
        } else {
            self.busy_for -= 1;
        }
        self.mmu.cycle(); // Not sure if this is the right place
        if self.mmu.ppu_ref().ly == ppu::LY_VBLANK_START {
            self.request_interrupt(Interrupt::VBlank, true);
        }
    }

    /// Push a u8 value onto the stack.
    pub fn push_stack_u8(&mut self, val: u8) {
        self.mmu.write_u8(self.registers.sp, val);
        self.registers.sp = self.registers.sp.wrapping_sub(1);
    }

    /// Pop a u8 value from the stack.
    pub fn pop_stack_u8(&mut self) -> u8 {
        self.registers.sp = self.registers.sp.wrapping_add(1);
        self.mmu.read_u8(self.registers.sp)
    }

    /// Push a u16 value onto the stack.
    pub fn push_stack_u16(&mut self, val: u16) {
        self.mmu.write_u16(self.registers.sp, val);
        self.registers.sp = self.registers.sp.wrapping_sub(2);
    }

    /// Pop a u16 value from the stack.
    pub fn pop_stack_u16(&mut self) -> u16 {
        self.registers.sp = self.registers.sp.wrapping_add(2);
        self.mmu.read_u16(self.registers.sp)
    }

    /// Reads a byte from memory at pc and increases pc by one.
    pub fn read_u8_at_pc_and_increase(&mut self) -> u8 {
        let val = self.mmu.read_u8(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        val
    }

    /// Reads two bytes from memory at pc.
    pub fn read_u16_at_pc(&self) -> u16 {
        let l = self.mmu.read_u8(self.registers.pc);
        let h = self.mmu.read_u8(self.registers.pc + 1);
        // Little endian in memory
        utils::merge_u8s(h, l)
    }

    /// Reads two bytes from memory at pc and increases pc by two.
    pub fn read_u16_at_pc_and_increase(&mut self) -> u16 {
        let val = self.mmu.read_u16(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(2);
        val
    }

    /// Check for u8 half carries on additions. (carry from 3rd to 4th bit).
    pub fn check_add_u8_hc(left: u8, right: u8) -> bool {
        (left & 0x0F) + right > 0x0F
    }

    /// Check for u8 half carries on additions. (carry from 7th to 8th bit).
    pub fn check_add_u16_hc(left: u16, right: u16) -> bool {
        (left & 0x0FFF) + (right & 0x0FFF) > 0x0FFF
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
