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

fn calc_next(cur: f64, n: f64) -> f64 {
    (cur + n / cur) / 2.0
}

/// 使用 [牛顿迭代法](https://baike.baidu.com/item/%E7%89%9B%E9%A1%BF%E8%BF%AD%E4%BB%A3%E6%B3%95/10887580) 计算一个值是否是完全平方数
pub fn is_perfect_square(n: i32) -> bool {
    let mut cur = n as f64;
    let min = (1.0_f64).powf(-6.0);
    loop {
        let next = calc_next(cur, n as f64);
        if cur - next < min {
            break;
        }
        cur = next;
    }
    n == (cur as i32) * (cur as i32)
}
