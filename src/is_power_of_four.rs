//! 342. 4的幂
//!
//! 给定一个整数，写一个函数来判断它是否是 4 的幂次方。如果是，返回 true ；否则，返回 false 。
//
//! 整数 n 是 4 的幂次方需满足：存在整数 x 使得 n == 4x
//
//! 来源：力扣（LeetCode）
//! 链接：https://leetcode.cn/problems/power-of-four

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn is_16() {
        assert!(is_power_of_four(16))
    }

    #[test]
    fn not_5() {
        assert!(!is_power_of_four(5))
    }

    #[test]
    fn is_1() {
        assert!(is_power_of_four(1))
    }
}

pub fn is_power_of_four(n: i32) -> bool {
    if n < 1 {
        return false;
    }
    if n == 1 {
        return true;
    }
    let mut n = n;
    while n != 4 {
        if n % 4 != 0 {
            return false;
        }
        n /= 4;
    }
    true
}
