use std::process;
use super::Cpu;

pub type InstructionInfo = (u8, fn(&mut Cpu) -> u8, &'static str, u8);

/// INST DEST, SRC
pub const INSTRUCTIONS: [InstructionInfo; 0xE] = [
    (0x00, nop,             "NOP",              1),
    (0x01, ld_bc_d16,       "LD  BC,  d16",     3),
    (0x02, ld_at_bc_a,      "LD (BC), A",       1),
    (0x03, inc_bc,          "INC BC",           1),
    (0x05, dec_b,           "DEC  B",           1),
    (0x06, ld_b_d8,         "LD  B d8",         2),
    (0x0E, ld_c_d8,         "LD  C d8",         2),
    (0x13, inc_de,          "INC DE",           1),
    (0x21, ld_hl_d16,       "LD  HL d16",       3),
    (0x23, inc_hl,          "INC HL",           1),
    (0x32, ld_at_hl_dec_a,  "LD (HL-) A",       1),
    (0x33, inc_sp,          "INC SP",           1),
    (0xAF, xor_a,           "XOR A",            1),
    (0xC3, jp_a16,          "JP a16",           3),
];

/// Returns InstructionInfo for a given opcode,
/// or exits the program if the instruction wasn't 
/// found.
pub fn decode_instruction(opcode: u8) -> InstructionInfo {
    for element in INSTRUCTIONS.iter() {
        if element.0 == opcode {
            return *element;
        }
    }
    println!("Unknown instruction 0x{:x}!", opcode);
    process::exit(-1);
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

/// OPCode: 0x05
/// Mnenonic: DEC B
pub fn dec_b(cpu: &mut Cpu) -> u8 {
    // TODO: Hier weiter machen
    let u = cpu.registers.get_b() - 1;
    cpu.registers.set_b(u);
    return 2;
}


/// OPCode: 0x06
/// Mnenonic: LD B d8
pub fn ld_b_d8(cpu: &mut Cpu) -> u8 {
    let val = cpu.read_u8_at_pc_and_increase();
    cpu.registers.set_b(val);
    return 2;
}


/// OPCode: 0x0E
/// Mnenonic: LD C, d8
pub fn ld_c_d8(cpu: &mut Cpu) -> u8 {
    let val = cpu.read_u8_at_pc_and_increase();
    cpu.registers.set_c(val);
    return 2;
}


/// OPCode: 0x13
/// Mnenonic: INC DE
pub fn inc_de(cpu: &mut Cpu) -> u8 {
    let u = cpu.registers.get_de() + 1;
    cpu.registers.set_de(u);
    return 2;
}

/// OPCode: 0x21
/// Mnenonic: LD HL d16
pub fn ld_hl_d16(cpu: &mut Cpu) -> u8 {
    let val = cpu.read_u16_at_pc_and_increase();
    cpu.registers.set_hl(val);
    return 3;
}

/// OPCode: 0x23
/// Mnenonic: INC HL
pub fn inc_hl(cpu: &mut Cpu) -> u8 {
    let u = cpu.registers.get_hl() + 1;
    cpu.registers.set_hl(u);
    return 2;
}

/// OPCode: 0x32
/// Mnenonic: LD (HL-), A
pub fn ld_at_hl_dec_a(cpu: &mut Cpu) -> u8 {
    let hl = cpu.registers.get_hl();
    let val = cpu.registers.a;
    cpu.write_u8(hl, val);
    cpu.registers.set_hl(hl - 1);
    return 2;
}

/// OPCode: 0x33
/// Mnenonic: INC SP
pub fn inc_sp(cpu: &mut Cpu) -> u8 {
    let u = cpu.registers.get_sp() + 1;
    cpu.registers.set_sp(u);
    return 2;
}

/// OPCode: 0xAF
/// Mnenonic: XOR A
pub fn xor_a(cpu: &mut Cpu) -> u8 {
    // XORing A with itself will always result in 0.
    cpu.registers.set_a(0);
    cpu.registers.set_flag_z(1);
    return 1;
}

/// OPCode: 0xC3
/// Mnenonic: JP a16
pub fn jp_a16(cpu: &mut Cpu) -> u8 {
    let address = cpu.read_u16_at_pc_and_increase();
    cpu.registers.set_pc(address);
    return 4;
}

