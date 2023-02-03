//! # 88.合并两个有序数组
//!
//!给你两个按 非递减顺序 排列的整数数组 nums1 和 nums2，另有两个整数 m 和 n ，分别表示 nums1 和 nums2 中的元素数目。
//!
//! 请你 合并 nums2 到 nums1 中，使合并后的数组同样按 非递减顺序 排列。
//!
//! 注意：最终，合并后数组不应由函数返回，而是存储在数组 nums1 中。为了应对这种情况，nums1 的初始长度为 m + n，其中前 m 个元素表示应合并的元素，后 n 个元素为 0 ，应忽略。nums2 的长度为 n 。
//!
//! 来源：力扣（LeetCode）
//!
//! 链接：<https://leetcode.cn/problems/merge-sorted-array>

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution1() {
        let mut nums1: Vec<i32> = vec![1, 2, 3, 0, 0, 0];
        let m = 3;
        let mut nums2: Vec<i32> = vec![2, 5, 6];
        let n = 3;
        merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, &[1, 2, 2, 3, 5, 6])
    }

    #[test]
    fn solution2() {
        let mut nums1: Vec<i32> = vec![0];
        let m = 1;
        let mut nums2: Vec<i32> = vec![];
        let n = 0;
        merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, &[0])
    }

    #[test]
    fn solution3() {
        let mut nums1: Vec<i32> = vec![0];
        let m = 0;
        let mut nums2: Vec<i32> = vec![1];
        let n = 1;
        merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, &[1])
    }

    #[test]
    fn solution4() {
        let mut nums1: Vec<i32> = vec![-1, 0, 0, 3, 3, 3, 0, 0, 0];
        let m = 6;
        let mut nums2: Vec<i32> = vec![1, 2, 2];
        let n = 3;
        merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, &[-1, 0, 0, 1, 2, 2, 3, 3, 3]);
    }
}

/// 合同两个有序数组
///
/// # 参数
///
/// * `nums1`: 有序数组1
/// * `m`: 数组1的长度
/// * `nums2`: 有序数组2
/// * `n`: 数组2的长度
///
/// returns: ()
///
/// # Examples
///
/// ```
/// use leetcode_rust::merge::merge;
/// let mut nums1 = vec![1,0];
/// let m = 1;
/// let mut nums2 = vec![2];
/// let n = 1;
/// let result = merge(&mut nums1,m,&mut nums2,n);
/// assert_eq!(nums1,&[1,2])
/// ```
pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut p = 0;
    let mut q = 0;
    let mut sorted = vec![];
    while p < m || q < n {
        let cur = if q == n {
            let cur = p as usize;
            p += 1;
            nums1[cur]
        } else if p == m {
            let cur = q as usize;
            q += 1;
            nums2[cur]
        } else if nums1[p as usize] < nums2[q as usize] {
            let cur = p as usize;
            p += 1;
            nums1[cur]
        } else {
            let cur = q as usize;
            q += 1;
            nums2[cur]
        };
        sorted.push(cur)
    }
    nums1.truncate(0);
    sorted.iter().for_each(|&v| nums1.push(v));
}
