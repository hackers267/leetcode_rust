//! 338.比特位计数
//!
//! 给你一个整数 n ，对于 0 <= i <= n 中的每个 i ，计算其二进制表示中 1 的个数 ，返回一个长度为 n + 1 的数组 ans 作为答案。
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode.cn/problems/counting-bits

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn is_2() {
        let n = 2;
        let v = count_bits(n);
        assert_eq!(&v, &[0, 1, 1])
    }

    #[test]
    fn is_5() {
        let n = 5;
        let v = count_bits(5);
        assert_eq!(&v, &[0, 1, 1, 2, 1, 2])
    }
}

pub fn count_bits(n: i32) -> Vec<i32> {
    let range = 0..=n;
    range.into_iter().map(|i| i.count_ones() as i32).collect()
}
