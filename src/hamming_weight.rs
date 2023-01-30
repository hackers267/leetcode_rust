#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn count_is_3() {
        let s = 0b1011;
        assert_eq!(hamming_weight(s), 3)
    }
    #[test]
    fn count_is_1() {
        let s = 0b10000000;
        assert_eq!(hamming_weight(s), 1)
    }
    #[test]
    fn count_is_31() {
        let s = 0b11111111111111111111111111111101;
        assert_eq!(hamming_weight(s), 31);
    }
}

pub fn hamming_weight(n: u32) -> i32 {
    n.count_ones() as i32
}
