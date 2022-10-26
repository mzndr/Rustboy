use std::process;
use crate::helpers::{self, merge_u8s};
mod instructions;


/**
* Emulating the LR35902 CPU
*
* For Opcodes see: https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html
*/

/** Working RAM **/
const WRAM_SIZE: usize = 0x20 * 0x400;
type WRam = [u8; WRAM_SIZE];


#[derive(Debug, Copy, Clone)]
pub struct Registers {
    a: u8,
    f: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,

    sp: u16,
    pc: u16,
}

#[derive(Debug, Copy, Clone)]
pub struct Cpu {
    pub registers: Registers,

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
    fn opcode_unknown(&mut self, opcode: u8 ) -> u8 {
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
            registers: Registers {
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

impl Registers {
    pub fn get_a(&mut self) -> u8 { return self.a; }
    pub fn set_a(&mut self, val: u8) { self.a = val; }

    pub fn get_f(&self) -> u8 { return self.f; }
    pub fn set_f(&mut self, val: u8) { self.f = val; }

    pub fn get_af(&self) -> u16 { 
        return helpers::merge_u8s(
        self.f,
        self.a, 
        ); 
    }


    pub fn set_af(&mut self, val: u16) { 
        let split = helpers::split_u16(val);
        self.a = split.0;
        self.f = split.1;
    }

    pub fn get_b(&self) -> u8 { return self.b; }
    pub fn set_b(&mut self, val: u8) { self.b = val; }

    pub fn get_c(&self) -> u8 { return self.c; }
    pub fn set_c(&mut self, val: u8) { self.c = val; }

    pub fn get_bc(&self) -> u16 { 
        return helpers::merge_u8s(
        self.b,
        self.c, 
        ); 
    }

    pub fn set_bc(&mut self, val: u16) {
        let split = helpers::split_u16(val);
        self.b = split.0;
        self.c = split.1;
    }

    pub fn get_d(&self) -> u8 { return self.d; }
    pub fn set_d(&mut self, val: u8) { self.d = val; }

    pub fn get_e(&self) -> u8 { return self.e; }
    pub fn set_e(&mut self, val: u8) { self.e = val; }

    pub fn get_de(&self) -> u16 { 
        return helpers::merge_u8s(
        self.d,
        self.e, 
        ); 
    }

    pub fn set_de(&mut self, val: u16) {
        let split = helpers::split_u16(val);
        self.d = split.0;
        self.e = split.1;
    }

    pub fn get_h(&self) -> u8 { return self.h; }
    pub fn set_h(&mut self, val: u8) { self.h = val; }

    pub fn get_l(&self) -> u8 { return self.l; }
    pub fn set_l(&mut self, val: u8) { self.l = val; }

    pub fn get_hl(&self) -> u16 { 
        return helpers::merge_u8s(
        self.h,
        self.l, 
        ); 
    }

    pub fn set_hl(&mut self, val: u16) {
        let split = helpers::split_u16(val);
        self.h = split.0;
        self.l = split.1; }

    pub fn get_sp(&mut self) -> u16 { return self.sp; }
    pub fn set_sp(&mut self, val: u16) { self.sp = val; }

    pub fn get_pc(&mut self) -> u16 { return self.pc; }
    pub fn set_pc(&mut self, val: u16) { self.pc = val; }

}
