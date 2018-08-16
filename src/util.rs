pub fn u8s_to_u16(lo: u8, hi: u8) -> u16 {
    u16::from(hi) << 8 | u16::from(lo)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u8s_to_u16() {
        let lo: u8 = 0x34;
        let hi: u8 = 0x12;
        assert_eq!(u8s_to_u16(lo, hi), 0x1234);
    }
}
