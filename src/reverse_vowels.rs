//! # 345.反转字符串中的元音字母
//!
//! 给你一个字符串 s ，仅反转字符串中的所有元音字母，并返回结果字符串。
//!
//! 元音字母包括 'a'、'e'、'i'、'o'、'u'，且可能以大小写两种形式出现不止一次。
//!
//! 来源：力扣（LeetCode）
//!
//! 链接：<https://leetcode.cn/problems/reverse-vowels-of-a-string>

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hello() {
        let s = "hello".to_string();
        let r = reverse_vowels(s);
        assert_eq!(r, "holle");
    }

    #[test]
    fn leetcode() {
        let s = "leetcode".to_string();
        let r = reverse_vowels(s);
        assert_eq!(r, "leotcede")
    }

    #[test]
    fn aeiou() {
        let s = "aeiou".to_string();
        let r = reverse_vowels(s);
        assert_eq!(r, "uoiea");
    }

    #[test]
    fn empty() {
        let s = " ".to_string();
        let r = reverse_vowels(s);
        assert_eq!(r, " ");
    }
}

/// 判断一个字符是不是元音字符
///
/// # 参数
///
/// * `c`: 字符
///
/// returns: bool 如果是元音字符返回`true`，否则返回`false`
///
/// # 示例
///
/// ```
///  use leetcode_rust::reverse_vowels::is_vowels;
/// let b = is_vowels('a');
/// assert!(b);
/// ```
pub fn is_vowels(c: char) -> bool {
    let v = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    v.contains(&c)
}

/// 反转字符串中的元音字母
///
/// # 参数
///
/// * `s`: 要反转的字符串
///
/// returns: String 反转后的字符串
///
/// # Examples
///
/// ```
/// use leetcode_rust::reverse_vowels::reverse_vowels;
/// let s = "hello".to_string();
/// let r = reverse_vowels(s);
/// assert_eq!(r,"holle".to_string())
/// ```
pub fn reverse_vowels(s: String) -> String {
    let mut p = 0;
    let mut q = s.len() - 1;
    let mut s: Vec<char> = s.chars().collect();
    while p < q {
        if !is_vowels(s[p]) {
            p += 1;
        }
        if !is_vowels(s[q]) {
            q -= 1;
        }
        if is_vowels(s[p]) && is_vowels(s[q]) {
            s.swap(p, q);
            p += 1;
            q -= 1;
        }
    }
    s.iter().collect()
}
