//! # 461.汉明距离
//!
//! 两个整数之间的 汉明距离 指的是这两个数字对应二进制位不同的位置的数目。
//!
//! 给你两个整数 x 和 y，计算并返回它们之间的汉明距离。
//!
//! 示例 1：
//! 输入：x = 1, y = 4
//! 输出：2
//!
//! 解释：
//!  1   (0 0 0 1)
//!  4   (0 1 0 0)
//!       ↑   ↑
//!  上面的箭头指出了对应二进制位不同的位置。
//!
//!  示例 2：
//!
//!  输入：x = 3, y = 1
//!  输出：1
//!
//! 来源：力扣（LeetCode）
//!
//! 链接：<https://leetcode.cn/problems/hamming-distance>

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn distance_is_2_between_1_and_4() {
        assert_eq!(hamming_distance(1, 4), 2);
    }

    #[test]
    fn distance_is_1_between_1_and_3() {
        assert_eq!(hamming_distance(3, 1), 1)
    }

    #[test]
    fn distance_is_7_between_2_and_999() {
        assert_eq!(hamming_distance(2, 999), 7);
    }
}

pub fn hamming_distance(x: i32, y: i32) -> i32 {
    let v = x ^ y;
    v.count_ones() as i32
}
