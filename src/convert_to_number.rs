//! # 168. Excel表列名称
//!
//! 给你一个整数 columnNumber ，返回它在 Excel 表中相对应的列名称。
//!
//! 例如：
//!
//! > A -> 1
//! > B -> 2
//! > C -> 3
//! >...
//! > Z -> 26
//! > AA -> 27
//! > AB -> 28
//! >...
//!
//! 来源：力扣（LeetCode）
//!
//! 链接：<https://leetcode.cn/problems/excel-sheet-column-title>
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn number_is_1() {
        let s = String::from("A");
        let n = convert_to_number(s);
        assert_eq!(n, 1);
    }

    #[test]
    fn number_is_28() {
        let n = convert_to_number(String::from("AB"));
        assert_eq!(n, 28)
    }
    #[test]
    fn number_is_701() {
        let n = convert_to_number(String::from("ZY"));
        assert_eq!(n, 701)
    }

    #[test]
    fn number_is_2147483647() {
        let n = convert_to_number(String::from("FXSHRXW"));
        assert_eq!(n, 2147483647)
    }
}

/// 给定一个字符串 columnTitle ，表示 Excel 表格中的列名称。返回 该列名称对应的列序号 。
/// ```
/// use leetcode_rust::convert_to_number::convert_to_number;
/// let n = convert_to_number(String::from("FXSHRXW"));
/// assert_eq!(n, 2147483647)
/// ```
/// 使用[秦九韶算法](https://baike.baidu.com/item/%E7%A7%A6%E4%B9%9D%E9%9F%B6%E7%AE%97%E6%B3%95/449196)实现
pub fn convert_to_number(s: String) -> i32 {
    s.bytes()
        .map(|v| v as i32 - 64)
        .fold(0_i32, |acc, x| acc * 26 + x)
}
