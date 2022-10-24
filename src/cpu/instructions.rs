use super::types::{Cpu, Register};
/// Mnenonic: NOP 
pub fn instruction_0x00() -> u8 {
    nop();
    return 0x04;
}

/// Mnenonic: INC BC 
pub fn instruction_0x03(cpu: &mut Cpu) -> u8 {
    let u = u16::from(cpu.registers.bc) + 1;
    cpu.registers.bc = Register::from(u);
    return 0x08;
}

/// Mnenonic: INC DE
pub fn instruction_0x13(cpu: &mut Cpu) -> u8 {
    let u = u16::from(cpu.registers.de) + 1;
    cpu.registers.bc = Register::from(u);
    return 0x08;
}

/// Mnenonic: INC HL
pub fn instruction_0x23(cpu: &mut Cpu) -> u8 {
    let u = u16::from(cpu.registers.hl) + 1;
    cpu.registers.bc = Register::from(u);
    return 0x08;
}

/// Mnenonic: INC SP
pub fn instruction_0x33(cpu: &mut Cpu) -> u8 {
    let u = u16::from(cpu.registers.sp) + 1;
    cpu.registers.bc = Register::from(u);
    return 0x08;
}



/** Instruction Implementations **/

/// No Operation.
fn nop() {
    println!("nop");
}
