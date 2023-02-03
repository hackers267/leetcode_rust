//! # 9.回文数
//!
//! 给你一个整数 x ，如果 x 是一个回文整数，返回 true ；否则，返回 false 。
//!
//! 回文数是指正序（从左向右）和倒序（从右向左）读都是一样的整数。
//!
//! - 例如，121 是回文，而 123 不是。
//
//! 来源：力扣（LeetCode）
//!
//! 链接：<https://leetcode.cn/problems/palindrome-number>

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn is_palindrome_easy_true_test() {
        let x = 12321;
        let result = is_palindrome(x);
        assert!(result);
    }
    #[test]
    fn is_palindrome_easy_false_test() {
        let x = 12345;
        let result = is_palindrome(x);
        assert!(!result);
    }
    #[test]
    fn is_palindrome_with_zero() {
        let result = is_palindrome(0);
        assert!(result);
    }
    #[test]
    fn is_palindrome_with_five() {
        let result = is_palindrome(5);
        assert!(result);
    }
    #[test]
    fn is_palindrome_with_hundred() {
        let result = is_palindrome(100);
        assert!(!result);
    }
    #[test]
    fn is_palindrome_negative() {
        let result = is_palindrome(-1);
        assert!(!result)
    }
}
pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    let result = get_reverse(x);
    result == x
}

fn get_reverse(m: i32) -> i32 {
    m.to_string()
        .split("")
        .filter_map(|x| match x.is_empty() {
            false => Some(x.to_string()),
            true => None,
        })
        .enumerate()
        .fold(0i32, |acc, (index, value)| {
            let i: i32 = value.parse().unwrap();
            acc + i * 10_i32.pow(index as u32)
        })
}
