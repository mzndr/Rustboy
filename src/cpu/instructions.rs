use super::Cpu;
pub type InstructionInfo = (u8, fn(&mut Cpu) -> u8, &'static str);
pub const INSTRUCTIONS: [InstructionInfo; 0x5] = [
    (0x00, nop, "NOP"), 

    (0x13, inc_bc, "INC BC"),
    (0x23, inc_de, "INC DE"),
    (0x33, inc_hl, "INC HL"),
    (0x43, inc_sp, "INC SP"),
    
];

/// OPCode: 0x00 
/// Mnenonic: NOP 
pub fn nop(_cpu: &mut Cpu) -> u8 {
    return 1;
}

/// OPCode: 0x13 
/// Mnenonic: INC BC 
pub fn inc_bc(cpu: &mut Cpu) -> u8  {
    let u = cpu.registers.get_bc() + 1;
    cpu.registers.set_bc(u);
    return 2;
}

/// OPCode: 0x23 
/// Mnenonic: INC DE
pub fn inc_de(cpu: &mut Cpu) -> u8  {
    let u = cpu.registers.get_de() + 1;
    cpu.registers.set_de(u);
    return 2;
}

/// OPCode: 0x33 
/// Mnenonic: INC HL
pub fn inc_hl(cpu: &mut Cpu) -> u8  {
    let u = cpu.registers.get_hl() + 1;
    cpu.registers.set_hl(u);
    return 2;
}

/// OPCode: 0x43 
/// Mnenonic: INC SP
pub fn inc_sp(cpu: &mut Cpu) -> u8  {
    let u = cpu.registers.get_sp() + 1;
    cpu.registers.set_sp(u);
    return 2;
}
