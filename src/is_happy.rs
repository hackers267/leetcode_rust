//! # 202.快乐数
//!
//! 编写一个算法来判断一个数 n 是不是快乐数。
//!
//! 「快乐数」 定义为：
//!
//!  -   对于一个正整数，每一次将该数替换为它每个位置上的数字的平方和。
//!  -   然后重复这个过程直到这个数变为 1，也可能是 无限循环 但始终变不到 1。
//!  -   如果这个过程 结果为 1，那么这个数就是快乐数。
//!
//! 如果 n 是 快乐数 就返回 true ；不是，则返回 false 。
//!
//! 来源：力扣（LeetCode）
//!
//! 链接：<https://leetcode.cn/problems/happy-number>

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn is_happy_19() {
        assert!(is_happy(19));
    }

    #[test]
    fn not_happy_2() {
        assert!(!is_happy(2));
    }

    #[test]
    fn not_happy_6() {
        assert!(!is_happy(6));
    }
}

use std::collections::HashMap;
pub fn is_happy(n: i32) -> bool {
    let mut hash_map = HashMap::new();
    let mut n = n;
    while n != 1 {
        let value = hash_map.get(&n);
        if value.is_some() {
            return false;
        }
        hash_map.insert(n, 1);
        n = calc(n)
    }
    true
}

fn calc(n: i32) -> i32 {
    format!("{n}")
        .split("")
        .filter_map(|x| x.parse::<i32>().ok())
        .fold(0, |acc, cur| acc + cur.pow(2))
}
