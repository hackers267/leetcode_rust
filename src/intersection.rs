//! # 349.两个数组的交集
//!
//! 给定两个数组 nums1 和 nums2 ，返回 它们的交集 。输出结果中的每个元素一定是 唯一 的。我们可以 不考虑输出结果的顺序 。
//
//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersection_is_2() {
        let nums1 = vec![1, 2, 2, 1];
        let nums2 = vec![2, 2];
        assert_eq!(intersection(nums1, nums2), vec![2])
    }

    #[test]
    fn intersection_is_4_9() {
        let nums1 = vec![4, 9, 5];
        let nums2 = vec![9, 4, 9, 8, 4];
        assert_eq!(intersection(nums1, nums2), vec![9, 4]);
    }
}

use std::collections::HashMap;
pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let map = nums1.iter().fold(HashMap::new(), |mut acc, cur| {
        acc.insert(cur, 1);
        acc
    });
    nums2
        .iter()
        .filter(|&v| map.contains_key(v))
        .copied()
        .fold(vec![], |mut acc, v| {
            if acc.contains(&v) {
                acc
            } else {
                acc.push(v);
                acc
            }
        })
}
