//! 383.赎金信
//!
//! 给你两个字符串：ransomNote 和 magazine ，判断 ransomNote 能不能由 magazine 里面的字符构成。
//!
//! 如果可以，返回 true ；否则返回 false 。
//!
//! magazine 中的每个字符只能在 ransomNote 中使用一次。
//!
//! 来源：力扣（LeetCode）
//!
//! 链接：<https://leetcode.cn/problems/ransom-note>

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn a_b_false() {
        test_false("a", "b")
    }

    #[test]
    fn aa_ab_false() {
        test_false("aa", "ab")
    }

    #[test]
    fn aa_aab_true() {
        test_true("aa", "aab")
    }

    fn test_false(r: &str, m: &str) {
        let r = String::from(r);
        let m = String::from(m);
        assert!(!can_construct(r, m))
    }

    fn test_true(r: &str, m: &str) {
        let r = String::from(r);
        let m = String::from(m);
        assert!(can_construct(r, m))
    }
}

use std::collections::HashMap;
pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    let mut dict = magazine.chars().fold(HashMap::new(), |mut acc, c| {
        let entry = acc.entry(c);
        let v = entry.or_insert(0);
        *v += 1;
        acc
    });
    for c in ransom_note.chars() {
        let v = dict.get(&c);
        if v.is_some() {
            let v = *v.unwrap();
            if v == 1 {
                dict.remove(&c);
            } else {
                dict.insert(c, v - 1);
            }
        } else {
            return false;
        }
    }
    true
}
