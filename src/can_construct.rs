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

fn get_idx(c: u8) -> usize {
    (c - b'a') as usize
}

pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    if magazine.len() < ransom_note.len() {
        return false;
    }
    let mut v = [0; 26];
    magazine.bytes().map(get_idx).for_each(|idx| {
        v[idx] += 1;
    });
    for idx in ransom_note.bytes().map(get_idx) {
        let value = v[idx];
        if value == 0 {
            return false;
        } else {
            v[idx] -= 1;
        }
    }
    true
}
