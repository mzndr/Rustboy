use std::process;

use self::registers::Registers;
mod instructions;

pub mod operations;
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
        if address as usize >= WRAM_SIZE {
            tracing::error!(
                "Memory access at 0x{:x} out of bounds. Valid address space: (0x0000-0x{:x}).",
                address,
                WRAM_SIZE - 1
            );
            process::exit(-1);
        }
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

    /// Handles an instruction according to specifications.
    /// For specifications see: <https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html>
    fn execute_current_instruction(&mut self) -> u8 {
        let pc = self.registers.get_pc();
        let instruction = self.read_u8_at_pc_and_increase();
        tracing::trace!("[0x{pc:x}] Executing instruction 0x{instruction:x}... ");
        let cycled_needed = self.exec_instruction(instruction);

        // Something went wrong when no cycles were needed.
        if cycled_needed == 0 {
            tracing::error!(
                "Something went wrong while executing instruction 0x{instruction:x}! Exiting..."
            );
            process::exit(-1);
        }

        tracing::trace!("Needed {cycled_needed} cycles.");
        cycled_needed
    }

    // Execute a machine cycle.
    pub fn cycle(&mut self) {
        if self.busy_for == 0 {
            self.busy_for = self.execute_current_instruction();
        } else {
            self.busy_for -= 1;
        }
    }
}
