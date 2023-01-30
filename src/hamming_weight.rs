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

/// 利用n和n-1可以消除二进制中从右往左遇到的第一个1的性质，统计1的个数
pub fn hamming_weight(mut n: u32) -> i32 {
    let mut r = 0;
    while n != 0 {
        n = n & (n - 1);
        r += 1;
    }
    r
}
