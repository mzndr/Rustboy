pub fn split_u16(to_split: u16) -> (u8, u8) {
    let s0: u8 = (to_split >> 8) as u8;
    let s1: u8 = to_split as u8;
    return (s0, s1);
}

pub fn merge_u8s(a: u8, b: u8) -> u16 {
    let r1 = (a as u16) << 8;
    let r0 = b as u16;
    return r0 | r1;
}
