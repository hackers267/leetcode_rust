//! 2319.判断矩阵是否是一个X矩阵
//!
//! 题目描述：
//!
//! 如果一个正方形矩阵满足下述 全部 条件，则称之为一个 X 矩阵 ：
//!
//! 1.矩阵对角线上的所有元素都 不是 0
//! 2.矩阵中所有其他元素都是 0
//!
//! 给你一个大小为 n x n 的二维整数数组 grid ，表示一个正方形矩阵。如果 grid 是一个 X 矩阵 ，返回 true ；否则，返回 false 。
//!
//! 来源：力扣（LeetCode）
//! 链接：https://leetcode.cn/problems/check-if-matrix-is-x-matrix
//!
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn check_x_matrix_is_valid() {
        let grid: Grid = vec![
            vec![2, 0, 0, 1],
            vec![0, 3, 1, 0],
            vec![0, 5, 2, 0],
            vec![4, 0, 0, 2],
        ];
        assert!(check_x_matrix(grid));
    }

    #[test]
    fn check_x_matrix_is_invalid() {
        let grid: Grid = vec![vec![5, 7, 0], vec![0, 3, 1], vec![0, 5, 0]];
        assert!(!check_x_matrix(grid))
    }
}
type Grid = Vec<Vec<i32>>;
pub fn check_x_matrix(grid: Grid) -> bool {
    let len = grid.len();
    for (i, line) in grid.iter().enumerate() {
        for (j, &value) in line.iter().enumerate() {
            if is_diagonal(i, j, len) && value == 0 {
                return false;
            }
            if !is_diagonal(i, j, len) && value != 0 {
                return false;
            }
        }
    }
    true
}

fn is_diagonal(i: usize, j: usize, len: usize) -> bool {
    i == j || i == len - j - 1
}
