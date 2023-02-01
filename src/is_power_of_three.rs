//! 326. 3的幂
//!
//!给定一个整数，写一个函数来判断它是否是 3 的幂次方。如果是，返回 true ；否则，返回 false 。
//!
//! 整数 n 是 3 的幂次方需满足：存在整数 x 使得 n == 3x
//!
//! 来源：力扣（LeetCode）
//! 链接：https://leetcode.cn/problems/power-of-three

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn is_27() {
        assert!(is_pow_of_three(27));
    }

    #[test]
    fn is_9() {
        assert!(is_pow_of_three(9));
    }

    #[test]
    fn not_45() {
        assert!(!is_pow_of_three(45));
    }

    #[test]
    fn not_0() {
        assert!(!is_pow_of_three(0));
    }
}

pub fn is_pow_of_three(n: i32) -> bool {
    if n < 1 {
        return false;
    }
    if n == 2 {
        return true;
    }
    let mut n = n;
    while n != 3 {
        if n % 3 != 0 {
            return false;
        }
        n /= 3;
    }
    true
}
