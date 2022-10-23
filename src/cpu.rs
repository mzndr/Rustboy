/**
* Emulating the LR35902 CPU
*
* For Opcodes see: https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html
*/

/** Working RAM **/
const WRAM_SIZE: usize = 0x20 * 0x400;
type WRam = [u8; WRAM_SIZE];

/** Registers **/
union Register {
    r0: u8,
    r1: u8,
}

struct Registers {
    af: Register,
    bc: Register,
    de: Register,
    hl: Register,
    sp: u16,
    pc: u16,
}

pub struct Cpu {
    registers: Registers,
    wram: WRam,
}

/// Initialize cpu memory
pub fn init_cpu() -> Cpu {
    return Cpu { 
        registers: Registers {
            af: Register { r0: 0x00 },
            bc: Register { r0: 0x00 },
            de: Register { r0: 0x00 },
            hl: Register { r0: 0x00 },
            sp: 0x0000,
            pc: 0x0100,
        },
        wram: [0x00; WRAM_SIZE],
    };
}

pub fn clock_cycle(cpu: &mut Cpu) {
    let instruction = fetch_instruction(cpu);
    handle_instruction(cpu, instruction);
}

/// Fetches an instruction/opcode from memory
/// and increases the program counter.
fn fetch_instruction(cpu: &mut Cpu) -> u8 {
    let pc: u16 = cpu.registers.pc;
    let opcode: u8 = cpu.wram[pc as usize];
    cpu.registers.pc += 1;
    return opcode;
}

/// Handles an instruction according to specifications.
/// For specifications see: https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html
fn handle_instruction(cpu: &mut Cpu, instruction: u8) {
    match instruction {
        0x00 => instruction_nop(),
        _ => instruction_unknown(instruction, cpu),
    };
}

/// Nop instruction.
/// Opcode: 0x00
fn instruction_nop(){
    println!("nop");
}

/// Unknown instruction.
/// TODO: Dump cpu state to log file.
fn instruction_unknown(opcode: u8, cpu: &mut Cpu){
    println!("Unknown instruction {opcode}!");
}


