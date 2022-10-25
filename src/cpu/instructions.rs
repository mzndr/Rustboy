use super::Cpu;

pub const OPCODES: [(fn(&mut Cpu), u8); 0x001]  = [
    (nop, 255),         // 0x00
];

/// OPCode: 0x00 
/// Mnenonic: NOP 
pub fn nop(cpu: &mut Cpu) {
}

/// OPCode: 0x13 
/// Mnenonic: INC BC 
pub fn inc_bc(cpu: &mut Cpu) {
    let u = cpu.registers.get_bc() + 1;
    cpu.registers.set_bc(u);
}

/// OPCode: 0x23 
/// Mnenonic: INC DE
pub fn inc_de(cpu: &mut Cpu) {
    let u = cpu.registers.get_de() + 1;
    cpu.registers.set_de(u);
}

/// OPCode: 0x33 
/// Mnenonic: INC HL
pub fn inc_hl(cpu: &mut Cpu) {
    let u = cpu.registers.get_hl() + 1;
    cpu.registers.set_hl(u);
}

/// OPCode: 0x43 
/// Mnenonic: INC SP
pub fn inc_sp(cpu: &mut Cpu) {
    let u = cpu.registers.get_sp() + 1;
    cpu.registers.set_sp(u);
}
