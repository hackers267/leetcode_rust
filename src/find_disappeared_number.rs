//! # 448.找到所有数组中消失的数字
//!
//! 给你一个含 n 个整数的数组 nums ，其中 nums\[i\] 在区间 \[1, n\] 内。请你找出所有在 \[1, n\] 范围内但没有出现在 nums 中的数字，并以数组的形式返回结果。
//!
//! 示例 1：
//!
//! 输入：nums = \[4,3,2,7,8,2,3,1\]
//!
//! 输出：\[5,6\]
//!
//! 来源：力扣（LeetCode）
//!
//! 链接：<https://leetcode.cn/problems/find-all-numbers-disappeared-in-an-array>

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution1() {
        let nums = vec![4, 3, 2, 7, 8, 2, 3, 1];
        let result = find_disappeared_numbers(nums);
        assert_eq!(result, &[5, 6]);
    }

    #[test]
    fn solution2() {
        let nums = vec![1, 1];
        let result = find_disappeared_numbers(nums);
        assert_eq!(result, &[2])
    }
}

/// 找到所有数组中消失的数字
///
/// # 参数
///
/// * `nums`: 数组
///
/// returns: Vec<i32, Global> 消失的数字组成的数组
///
/// # Examples
///
/// ```
/// use leetcode_rust::find_disappeared_number::find_disappeared_numbers;
/// let nums = vec![1,1];
/// let result = find_disappeared_numbers(nums);
/// assert_eq!(result,&[2]);
/// ```
pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
    let mut range: Vec<i32> = (1..=nums.len()).into_iter().map(|i| i as i32).collect();
    for i in nums {
        let idx = (i - 1) as usize;
        range[idx] = 0;
    }
    range.iter().filter(|&&v| v != 0).copied().collect()
}
