//! 263.丑数
//!
//! 题目描述：
//!
//! 丑数 就是只包含质因数 2、3 和 5 的正整数。
//!
//! 给你一个整数 n ，请你判断 n 是否为 丑数 。如果是，返回 true ；否则，返回 false 。
//! 丑数 就是只包含质因数 2、3 和 5 的正整数。
//! 来源：力扣（LeetCode）
//! 链接：https://leetcode.cn/problems/ugly-number

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn is_ugly_6() {
        let n = 6;
        assert!(is_ugly(n))
    }

    #[test]
    fn is_ugly_1() {
        assert!(is_ugly(1))
    }

    #[test]
    fn not_ugly_14() {
        assert!(!is_ugly(14))
    }

    #[test]
    fn not_ugly_99() {
        assert!(!is_ugly(99))
    }
}
pub fn is_ugly(n: i32) -> bool {
    if n < 1 {
        return false;
    }
    let mut n = n;
    while n % 2 == 0 {
        n /= 2;
    }
    while n % 3 == 0 {
        n /= 3;
    }
    while n % 5 == 0 {
        n /= 5;
    }
    n == 1
}
