//! # 231. 2的幂
//!
//!给你一个整数 n，请你判断该整数是否是 2 的幂次方。如果是，返回 true ；否则，返回 false 。
//!
//! 如果存在一个整数 x 使得 n == 2x ，则认为 n 是 2 的幂次方。
//!
//! 来源：力扣（LeetCode）
//!
//! 链接：<https://leetcode.cn/problems/power-of-two>
//!
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_1() {
        assert!(is_power_of_two(1))
    }
    #[test]
    fn is_16() {
        assert!(is_power_of_two(16))
    }

    #[test]
    fn not_3() {
        assert!(!is_power_of_two(3))
    }

    #[test]
    fn is_4() {
        assert!(is_power_of_two(4))
    }

    #[test]
    fn not_5() {
        assert!(!is_power_of_two(5))
    }
}

pub fn is_power_of_two(n: i32) -> bool {
    n > 0 && n.count_ones() == 1
}
