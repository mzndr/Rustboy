use std::process;
use super::Cpu;

pub type InstructionInfo = (u8, fn(&mut Cpu) -> u8, &'static str, u8);

/// INST DEST, SRC
pub const INSTRUCTIONS: [InstructionInfo; 0x7] = [
    (0x00, nop,         "NOP", 1),
    (0x01, ld_bc_d16,   "LD  BC,  d16", 3),
    (0x02, ld_at_bc_a,  "LD (BC), A", 1),
    (0x03, inc_bc,      "INC BC", 1),

    (0x13, inc_de,      "INC DE", 1),
    (0x23, inc_hl,      "INC HL", 1),
    (0x33, inc_sp,      "INC SP", 1),
];

/// Returns InstructionInfo for a given opcode,
/// or exits the program if the instruction wasn't 
/// found.
pub fn decode_instruction(opcode: u8) -> InstructionInfo {
    if opcode > INSTRUCTIONS.len() as u8 {
        println!("Unknown instruction 0x{:x}!", opcode);
        process::exit(-1);
    }
    return INSTRUCTIONS[opcode as usize];
}

/// OPCode: 0x00
/// Mnenonic: NOP
pub fn nop(_cpu: &mut Cpu) -> u8 {
    return 1;
}

/// OPCode: 0x01
/// Mnenonic: LD BC, d16
pub fn ld_bc_d16(cpu: &mut Cpu) -> u8 {
    let val = cpu.read_u16_at_pc_and_increase();
    cpu.registers.set_bc(val);
    return 3;
}

/// OPCode: 0x02
/// Mnenonic: LD (BC), A
pub fn ld_at_bc_a(cpu: &mut Cpu) -> u8 {
    let address = cpu.registers.get_bc();
    let val = cpu.registers.get_a();
    cpu.write_u8(address, val);
    return 2;
}


/// OPCode: 0x03
/// Mnenonic: INC BC
pub fn inc_bc(cpu: &mut Cpu) -> u8 {
    let u = cpu.registers.get_bc() + 1;
    cpu.registers.set_bc(u);
    return 2;
}

/// OPCode: 0x13
/// Mnenonic: INC DE
pub fn inc_de(cpu: &mut Cpu) -> u8 {
    let u = cpu.registers.get_de() + 1;
    cpu.registers.set_de(u);
    return 2;
}

/// OPCode: 0x23
/// Mnenonic: INC HL
pub fn inc_hl(cpu: &mut Cpu) -> u8 {
    let u = cpu.registers.get_hl() + 1;
    cpu.registers.set_hl(u);
    return 2;
}

/// OPCode: 0x33
/// Mnenonic: INC SP
pub fn inc_sp(cpu: &mut Cpu) -> u8 {
    let u = cpu.registers.get_sp() + 1;
    cpu.registers.set_sp(u);
    return 2;
}
