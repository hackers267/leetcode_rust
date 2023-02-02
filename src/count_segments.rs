//! # 434.字符串的单词数
//!
//! 统计字符串中的单词个数，这里的单词指的是连续的不是空格的字符。
//!
//! 请注意，你可以假定字符串里不包括任何不可打印的字符。
//!
//! 示例:
//!
//! 输入: "Hello, my name is John"
//! 输出: 5
//! 解释: 这里的单词是指连续的不是空格的字符，所以 "Hello," 算作 1 个单词。
//!
//! 来源：力扣（LeetCode）
//!
//! 链接：<https://leetcode.cn/problems/number-of-segments-in-a-string>

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn count_is_5() {
        let s = "Hello, my name is John".to_string();
        assert_eq!(count_segments(s), 5)
    }

    #[test]
    fn count_is_2() {
        let s = "cargo rust".to_string();
        assert_eq!(count_segments(s), 2);
    }

    #[test]
    fn count_is_0() {
        let s = "".to_string();
        assert_eq!(count_segments(s), 0);
    }
}

pub fn count_segments(s: String) -> i32 {
    s.split(' ').filter(|&s| !s.is_empty()).count() as i32
}
