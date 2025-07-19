/// Split a 16 bit unsigned integer into two
/// 8 bit integers.
pub fn split_u16(to_split: u16) -> (u8, u8) {
    let h = (to_split >> 8) as u8;
    let l = ((to_split << 8) >> 8) as u8;
    (h, l)
}

/// Set the bit on `source` at `index` to `val`.
pub fn set_bit(source: u8, index: u8, val: bool) -> u8 {
    let mask: u8 = 1 << index;
    let mut ret = source;
    ret &= !mask;
    ret |= (u8::from(val)) << index;

    ret
}

/// Merges two 8 bit unsigned integers into
/// one 16 bit integer.
pub fn merge_u8s(h: u8, l: u8) -> u16 {
    ((h as u16) << 8) | (l as u16)
}

#[cfg(test)]
mod tests {

    use super::merge_u8s;
    use super::split_u16;

    #[test]
    fn split() {
        let to_split: u16 = 0xEAFE;
        let expect_left: u8 = 0xEA;
        let expect_right: u8 = 0xFE;
        let split = split_u16(to_split);
        let left: u8 = split.0;
        let right: u8 = split.1;
        assert_eq!(expect_left, left);
        assert_eq!(expect_right, right);
    }

    #[test]
    fn merge() {
        let to_split: u16 = 0xEAFE;
        let split: (u8, u8) = split_u16(to_split);
        let left: u8 = split.0;
        let right: u8 = split.1;
        let merged: u16 = merge_u8s(left, right);
        assert_eq!(merged, to_split);
    }
}
