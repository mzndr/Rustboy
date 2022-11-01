/// Checks if an 8 bit addition will cause a half carry.
pub fn has_8bit_half_carry(a: u8, b: u8) -> bool {
    return (((a & 0xF) + (b & 0xF)) & 0x10) == 0x10;
}

/// Checks if an 16 bit addition will cause a half carry.
pub fn has_16bit_half_carry(a: u16, b: u16) -> bool {
    return (((a & 0xFFF) + (b & 0xFFF)) & 0x1000) == 0x1000;
}

