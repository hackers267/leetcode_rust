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

/// 利用和1进行与运算，为1则最后一位为1(奇数),为0则最后一位为0(偶数),
/// 和向右移动的次数在统计出现1的次数
pub fn hamming_weight(mut n: u32) -> i32 {
    let mut r = 0;
    while n != 0 {
        if n & 1 == 1 {
            r += 1
        };
        n >>= 1;
    }
    r
}
