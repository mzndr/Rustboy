use crate::helpers;
use std::process;
mod instructions;
mod registers;

/**
* Emulating the LR35902 CPU
*
* For Opcodes see: https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html
*/

/** Working RAM **/
const WRAM_SIZE: usize = 0x20 * 0x400;
type WRam = [u8; WRAM_SIZE];

#[derive(Debug, Copy, Clone)]
pub struct Cpu {
    pub registers: registers::Registers,

    busy_for: u8,
    wram: WRam,
}

impl Cpu {
    /// Reads from wram at address.
    fn read(&self, address: u16) -> u8 {
        let u_addr = address as usize;
        if u_addr >= WRAM_SIZE {
            println!(
                "Memory access at 0x{:x} out of bounds. Valid address space: (0x0000-0x{:x}).",
                address,
                WRAM_SIZE - 1
            );
            process::exit(-1);
        }
        return self.wram[u_addr];
    }

    /// Reads a byte from wram at pc and increases pc by one.
    fn read_u8_at_pc_and_increase(&mut self) -> u8 {
        let val = self.read(self.registers.pc);
        self.registers.pc += 1;
        return val;
    }

    /// Reads two bytes from wram at pc and increases pc by two.
    fn read_u16_at_pc_and_increase(&mut self) -> u16 {
        let a = self.read_u8_at_pc_and_increase();
        let b = self.read_u8_at_pc_and_increase();
        return helpers::merge_u8s(a, b);
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
            registers: registers::Registers {
                a: 0x00,
                f: 0x00,
                b: 0x00,
                c: 0x00,
                d: 0x00,
                e: 0x00,
                h: 0x00,
                l: 0x00,
                sp: 0x0000,
                pc: 0x0100,
            },
            wram: [0x00; WRAM_SIZE],
            busy_for: 0x00,
        };
    }

    /// Handles an instruction according to specifications.
    /// For specifications see: https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html
    fn execute_current_instruction(&mut self) -> u8 {
        let instruction = self.read_u8_at_pc_and_increase();
        let instruction_info = instructions::decode_instruction(instruction);

        let f = instruction_info.1;
        let pc = self.registers.get_pc();
        let mnemonic = instruction_info.2;
        println!("[0x{:x}] Executing {mnemonic}", pc);
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
