use super::{utils, Cpu};

/// Different kinds of interrupt(-sources).
#[derive(Debug, Clone, Copy)]
pub enum Interrupt {
    VBlank,
    LCD,
    Timer,
    Serial,
    Joypad,
}

impl Interrupt {
    /// Checks if the interrupt bit is set in the given `u8`.
    fn is_set(self, reg_val: u8) -> bool {
        ((reg_val >> self.bit_index()) & 1) == 1
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
    fn address(self) -> u16 {
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
    /// Request an interrupt  by setting its bit in IF.
    pub fn request_interrupt(&mut self, source: Interrupt) {
        self.interrupt_flag = utils::set_bit(self.interrupt_flag, source.bit_index(), true);
    }

    /// Acknowledge an interrupt by unsetting its bit in IF.
    fn acknowledge_interrupt(&mut self, source: Interrupt) {
        self.interrupt_flag = utils::set_bit(self.interrupt_flag, source.bit_index(), false);
    }

    /// Handle interrupts.
    pub fn handle_interrupts(&mut self) -> bool {
        if !self.interrupt_master_enable
            || self.mmu.interrupt_enable == 0
            || self.interrupt_flag == 0
        {
            // Interrupts master disabled,
            // no interrupt enabled or no
            // interrupt requested.
            return false;
        }

        self.interrupt_master_enable = false;
        for source in Interrupt::enumerate() {
            if source.is_set(self.mmu.interrupt_enable) {
                self.halted = false;
                self.acknowledge_interrupt(source);
                tracing::debug!("handling interrupt: {source:?}");
                self.call(source.address());
                break;
            }
        }
        self.busy_for += 5;
        true
    }
}
