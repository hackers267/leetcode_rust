//! # 268.丢失的数字
//! 给定一个包含 [0, n] 中 n 个数的数组 nums ，找出 [0, n] 这个范围内没有出现在数组中的那个数。
//!
//! 来源：力扣（LeetCode）
//!
//! 链接：<https://leetcode.cn/problems/missing_number>

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn number_is_2() {
        let v = vec![3, 0, 1];
        assert_eq!(missing_number(v), 2)
    }

    #[test]
    fn number_is_8() {
        let v = vec![9, 6, 4, 2, 3, 5, 7, 0, 1];
        assert_eq!(missing_number(v), 8)
    }

    #[test]
    fn number_is_1() {
        let v = vec![0];
        assert_eq!(missing_number(v), 1)
    }

    #[test]
    fn number_is_0() {
        let v = vec![1, 2, 3];
        assert_eq!(missing_number(v), 0)
    }
}

pub fn missing_number(nums: Vec<i32>) -> i32 {
    let sum: i32 = nums.iter().sum();
    let len = nums.len();
    let total = len * (len + 1) / 2;
    total as i32 - sum
}
