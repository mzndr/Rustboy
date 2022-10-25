use std::process;
use crate::helpers;
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
struct Registers {
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

    /// Reads from wram at pc and increases pc.
    fn read_at_pc_and_increase(&mut self) -> u8 {
        let val = self.read(self.registers.pc);
        self.registers.pc += 1;
        return val;
    }

    /// Unknown instruction.
    /// TODO: Dump cpu state to log file.
    fn opcode_unknown(&mut self, opcode: u8 ) -> u8 {
        println!("Unknown instruction {opcode}!");
        return 0;
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
        };
    }

    /// Handles an instruction according to specifications.
    /// For specifications see: https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html
    pub fn execute_current_instruction(&mut self) -> u8 {
        let instruction = self.read_at_pc_and_increase();
        let cycled_needed = match instruction {
            _ => self.opcode_unknown(instruction),
        };

        if !cycled_needed == 0 {
            process::exit(-1);
        }

        return cycled_needed;
    }


}

impl Registers {
    pub fn get_a(&self) -> u8 { return self.a; }
    pub fn set_a(&mut self, val: u8) { self.a = val; }

    pub fn get_f(&self) -> u8 { return self.f; }
    pub fn set_f(&mut self, val: u8) { self.f = val; }

    pub fn get_af(&self) -> u16 { 
        return helpers::merge_u8s(
        self.f,
        self.a, 
        ); 
    }

    pub fn set_af(&self, val: u16) {
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

    pub fn set_bc(&self, val: u16) {
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

    pub fn set_de(&self, val: u16) {
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

    pub fn set_hl(&self, val: u16) {
        let split = helpers::split_u16(val);
        self.h = split.0;
        self.l = split.1; }

    pub fn get_sp(&self) -> u16 { return self.sp; }
    pub fn set_sp(&self, val: u16) { self.sp = val; }

    pub fn get_pc(&self) -> u16 { return self.pc; }
    pub fn set_pc(&self, val: u16) { self.pc = val; }

}


