use super::types::Cpu;
/// Mnenonic: NOP 
pub fn instruction_0x00() -> u8 {
    nop();
    return 0x04;
}

/// Mnenonic: INC BC 
pub fn instruction_0x03(cpu: &mut Cpu) -> u8 {
    u16 bc = 
    return 0x08;
}


/** Instruction Implementations **/

/// No Operation.
fn nop() {
    println!("nop");
}
