/// Split a 16 bit unsigned integer into two
/// 8 bit integers.
pub fn split_u16(to_split: u16) -> (u8, u8) {
    let split = to_split.to_be_bytes();
    (split[0], split[1])
}

/// Merges two 8 bit unsigned integers into
/// one 16 bit integer.
pub fn merge_u8s(left: u8, right: u8) -> u16 {
    u16::from_be_bytes([left, right])
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
