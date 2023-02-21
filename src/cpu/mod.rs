use crate::helpers::{self, split_u16};
use std::process;

use self::registers::Registers;
mod instructions;
pub mod registers;

/**
* Emulating the LR35902 CPU
*
* For Opcodes see: https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html
*/

/** Working RAM **/
const WRAM_SIZE: usize = 0xFFFF;//0x20 * 0x400;
type WRam = [u8; WRAM_SIZE];

#[derive(Debug ,Copy, Clone, PartialEq)]
pub struct Cpu {
    pub registers: Registers,

    busy_for: u8,
    wram: WRam, // Eventuell in memory umbennen und 0xFFFF groesse geben
}

impl Cpu {
    /// Checks if an address is in valid space,
    /// prints an error message and quits if not.
    fn check_address(&self, address: u16) {
        if address as usize >= WRAM_SIZE {
            println!(
                "Memory access at 0x{:x} out of bounds. Valid address space: (0x0000-0x{:x}).",
                address,
                WRAM_SIZE - 1
            );
            process::exit(-1);
        }
    }

    /// Reads from wram at address.
    fn read(&self, address: u16) -> u8 {
        let u_addr = address as usize;
        self.check_address(address);
        return self.wram[u_addr];
    }

    /// Writes u8 to wram at address.
    fn write_u8(&mut self, address: u16, val: u8) {
        let u_addr = address as usize;
        self.check_address(address);
        self.wram[u_addr] = val;
    }

    /// Writes u16 to wram at address.
    fn write_u16(&mut self, address: u16, val: u16) {
        let split = split_u16(val);
        self.write_u8(address, split.1);
        self.write_u8(address + 1, split.0);
    }

    /// Reads a byte from wram at pc and increases pc by one.
    fn read_u8_at_pc_and_increase(&mut self) -> u8 {
        let val = self.read(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        return val;
    }

    /// Reads two bytes from wram at pc and increases pc by two.
    fn read_u16_at_pc_and_increase(&mut self) -> u16 {
        let a = self.read_u8_at_pc_and_increase();
        let b = self.read_u8_at_pc_and_increase();

        // Little endian in memory
        return helpers::merge_u8s(b, a);     
    }

    /// Unknown instruction.
    /// TODO: Dump cpu state to log file.
    fn opcode_unknown(&mut self, opcode: u8) -> u8 {
        println!("Unknown instruction {opcode}!");
        return 0;
    }

    /// Needs to be changed for bigger games, since they
    /// are too big to fit into ram, so banking has to be
    /// implemented.
    pub fn load_rom(&mut self, rom: Vec<u8>) {
        for (address, byte) in rom.iter().enumerate() {
            self.wram[address] = *byte;
        }
    }

    /// Initialize cpu memory
    pub fn new() -> Cpu {
        return Cpu {
            registers: registers::Registers::new(),
            wram: [0x00; WRAM_SIZE],
            busy_for: 0x00,
        };
    }

    /// Handles an instruction according to specifications.
    /// For specifications see: https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html
    fn execute_current_instruction(&mut self) -> u8 {
        let instruction = self.read_u8_at_pc_and_increase();
        let instruction_info = instructions::decode_instruction(instruction);

        let opcode = instruction_info.0;
        let f = instruction_info.1;
        let mnemonic = instruction_info.2;
        let pc = self.registers.get_pc();
        println!("[0x{pc:x}] Executing 0x{opcode:x} {mnemonic}");
        let cycled_needed = f(self);

        // Something went wrong when no cycles were needed.
        if cycled_needed == 0 {
            process::exit(-1);
        }

        return cycled_needed;
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

#[cfg(test)] 
mod tests;
