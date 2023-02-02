//! 367.有效的完全平方数
//!
//! 给你一个正整数 num 。如果 num 是一个完全平方数，则返回 true ，否则返回 false 。
//!
//! 完全平方数 是一个可以写成某个整数的平方的整数。换句话说，它可以写成某个整数和自身的乘积。
//!
//! 不能使用任何内置的库函数，如  sqrt 。
//!
//! 来源：力扣（LeetCode）
//!
//! 链接：<https://leetcode.cn/problems/valid-perfect-square>

use std::cmp::Ordering;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn num_16_is_valid() {
        assert!(is_perfect_square(16))
    }

    #[test]
    fn num_32_not_valid() {
        assert!(!is_perfect_square(32))
    }

    #[test]
    fn num_64_is_valid() {
        assert!(is_perfect_square(64))
    }

    #[test]
    fn num_7921_is_valid() {
        assert!(is_perfect_square(7921))
    }

    #[test]
    fn num_9921_not_valid() {
        assert!(!is_perfect_square(9921))
    }

    #[test]
    fn num_9_is_valid() {
        assert!(is_perfect_square(9))
    }

    #[test]
    fn num_25_is_valid() {
        assert!(is_perfect_square(25));
    }

    #[test]
    fn num_1_is_valid() {
        assert!(is_perfect_square(1))
    }
}

pub fn is_perfect_square(n: i32) -> bool {
    let mut left = 0;
    let mut right = n;
    while left <= right {
        let v = left + (right - left) / 2;
        let square = v * v;
        match square.cmp(&n) {
            Ordering::Equal => return true,
            Ordering::Greater => right = v - 1,
            Ordering::Less => left = v + 1,
        }
    }
    false
}
