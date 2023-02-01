//! 389.找不同
//!
//!给定两个字符串 `s` 和 `t`，它们只包含小写字母。
//!
//! 字符串`t`由字符串 `s` 随机重排，然后在随机位置添加一个字母。
//!
//! 请找出在`t`中被添加的字母。
//!
//! 来源：力扣（LeetCode）
//!
//! 链接：<https://leetcode.cn/problems/find-the-difference>

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn difference_is_e() {
        let s = String::from("abcd");
        let t = String::from("abcde");
        let r = find_the_difference(s, t);
        assert_eq!(r, 'e')
    }

    #[test]
    fn difference_is_y() {
        let s = String::from("");
        let t = String::from("y");
        let r = find_the_difference(s, t);
        assert_eq!(r, 'y');
    }
}

pub fn find_the_difference(s: String, t: String) -> char {
    let sum1: usize = s.chars().map(|v| v as usize).sum();
    let sum2: usize = t.chars().map(|v| v as usize).sum();
    (sum2 - sum1) as u8 as char
}
