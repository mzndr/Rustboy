use std::process;
mod instructions;
pub mod types;
use types::{Cpu, Register, Registers, WRAM_SIZE};


/**
* Emulating the LR35902 CPU
*
* For Opcodes see: https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html
*/


/** Private functions **/

/// Reads from wram at pc and increases pc.
fn read(cpu: &mut Cpu, address: u16) -> u8 {
    let u_addr = address as usize;
    if u_addr >= WRAM_SIZE {
        println!("Memory access at 0x{:x} out of bounds.", address);
        process::exit(-1);
    }
    return cpu.wram[u_addr];
}

fn read_at_pc_and_increase(cpu: &mut Cpu) -> u8 {
    let val = read(cpu,cpu.registers.pc);
    cpu.registers.pc += 1;
    return val;
}

/** Opcode functions **/

/// Unknown instruction.
/// TODO: Dump cpu state to log file.
fn opcode_unknown(opcode: u8, cpu: &mut Cpu) -> u8 {
    println!("Unknown instruction {opcode}!");
    return 0;
}

/// Initialize cpu memory
pub fn init_cpu() -> Cpu {
    return Cpu { 
        registers: Registers {
            af: Register { r0: 0x00, r1: 0x00 },
            bc: Register { r0: 0x00, r1: 0x000 },
            de: Register { r0: 0x00, r1: 0x00 },
            hl: Register { r0: 0x00, r1: 0x00 },
            sp: 0x0000,
            pc: 0x0100,
        },
        wram: [0x00; WRAM_SIZE],
    };
}

/// Handles an instruction according to specifications.
/// For specifications see: https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html
pub fn execute_current_instruction(cpu: &mut Cpu) -> u8 {
    let instruction = read_at_pc_and_increase(cpu);
    let cycled_needed = match instruction {
        0x00 => instructions::instruction_0x00(),
        0x03 => instructions::instruction_0x03(cpu),
        0x13 => instructions::instruction_0x13(cpu),
        0x23 => instructions::instruction_0x23(cpu),
        0x33 => instructions::instruction_0x33(cpu),
        _ => opcode_unknown(instruction, cpu),
    };

    if !cycled_needed == 0 {
        process::exit(-1);
    }

    return cycled_needed;
}
