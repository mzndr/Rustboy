use std::process;

use super::Cpu;
pub type InstructionInfo = (u8, fn(&mut Cpu) -> u8, &'static str);
/// INST DEST, SRC
pub const INSTRUCTIONS: [InstructionInfo; 0x6] = [
    (0x00, nop, "NOP"),
    (0x01, ld_bc, "LD BC, d16"),
    (0x13, inc_bc, "INC BC"),
    (0x23, inc_de, "INC DE"),
    (0x33, inc_hl, "INC HL"),
    (0x43, inc_sp, "INC SP"),
];

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
pub fn ld_bc(cpu: &mut Cpu) -> u8 {
    let val = cpu.read_u16_at_pc_and_increase();
    cpu.registers.set_bc(val);
    return 3;
}

/// OPCode: 0x13
/// Mnenonic: INC BC
pub fn inc_bc(cpu: &mut Cpu) -> u8 {
    let u = cpu.registers.get_bc() + 1;
    cpu.registers.set_bc(u);
    return 2;
}

/// OPCode: 0x23
/// Mnenonic: INC DE
pub fn inc_de(cpu: &mut Cpu) -> u8 {
    let u = cpu.registers.get_de() + 1;
    cpu.registers.set_de(u);
    return 2;
}

/// OPCode: 0x33
/// Mnenonic: INC HL
pub fn inc_hl(cpu: &mut Cpu) -> u8 {
    let u = cpu.registers.get_hl() + 1;
    cpu.registers.set_hl(u);
    return 2;
}

/// OPCode: 0x43
/// Mnenonic: INC SP
pub fn inc_sp(cpu: &mut Cpu) -> u8 {
    let u = cpu.registers.get_sp() + 1;
    cpu.registers.set_sp(u);
    return 2;
}
