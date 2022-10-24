
/** Working RAM **/
pub const WRAM_SIZE: usize = 0x20 * 0x400;
pub type WRam = [u8; WRAM_SIZE];

/** Registers **/
#[derive(Copy, Clone)]
pub struct Register {
    pub r0: u8,
    pub r1: u8,
}

pub struct Registers {
    pub af: Register,
    pub bc: Register,
    pub de: Register,
    pub hl: Register,
    pub sp: u16,
    pub pc: u16,
}

pub struct Cpu {
    pub registers: Registers,
    pub wram: WRam,
}

impl From<u16> for Register {
    fn from(u: u16) -> Register {
        let r0: u8 = (u >> 8) as u8;
        let r1: u8 = u as u8;
        return Register { r0, r1 };
    }
}

impl From<Register> for u16 {
    fn from(reg: Register) -> u16 {
        let r0 = (reg.r0 as u16) << 8;
        let r1 = reg.r1 as u16;
        return r0 | r1;
    }
}

