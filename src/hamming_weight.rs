//! # 191.位1的个数
//!
//! 编写一个函数，输入是一个无符号整数（以二进制串的形式），返回其二进制表达式中数字位数为 '1' 的个数（也被称为汉明重量）。
//!
//!编写一个函数，输入是一个无符号整数（以二进制串的形式），返回其二进制表达式中数字位数为 '1' 的个数（也被称为汉明重量）。
//!
//! 来源：力扣（LeetCode）
//!
//! 链接：<https://leetcode.cn/problems/number-of-1-bits>

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
