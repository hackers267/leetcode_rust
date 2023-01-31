//! 205.同构字符串
//!
//! 题目内容：
//!
//!给定两个字符串 s 和 t ，判断它们是否是同构的。
//!
//! 如果 s 中的字符可以按某种映射关系替换得到 t ，那么这两个字符串是同构的。
//!
//! 每个出现的字符都应当映射到另一个字符，同时不改变字符的顺序。不同字符不能映射到同一个字符上，相同字符只能映射到同一个字符上，字符可以映射到自己本身。
//!
//! 来源：力扣（LeetCode）
//! 链接：https://leetcode.cn/problems/isomorphic-strings

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn egg_add_is_isomorphic() {
        let s = String::from("egg");
        let t = String::from("add");
        assert!(is_isomorphic(s, t))
    }

    #[test]
    fn foo_bar_not_isomorphic() {
        let s = String::from("foo");
        let t = String::from("bar");
        assert!(!is_isomorphic(s, t))
    }

    #[test]
    fn abab_abdc_not_isomorphic() {
        let s = String::from("abab");
        let t = String::from("abdc");
        assert!(!is_isomorphic(s, t))
    }

    #[test]
    fn bbbaaaba_aaabbbba_not_isomorphic() {
        let s = String::from("bbbaaaba");
        let t = String::from("aaabbbba");
        assert!(!is_isomorphic(s, t))
    }
}

use std::collections::HashMap;

pub fn is_isomorphic(s: String, t: String) -> bool {
    let mut s2t = HashMap::new();
    let mut t2s = HashMap::new();
    for (a, b) in s.chars().zip(t.chars()) {
        if let Some(c) = s2t.insert(a, b) {
            if c != b {
                return false;
            }
        }
        if let Some(c) = t2s.insert(b, a) {
            if c != a {
                return false;
            }
        }
    }
    true
}
