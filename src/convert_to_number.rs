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
pub fn convert_to_number(s: String) -> i32 {
    s.chars()
        .map(|c| (c as i32) - 64)
        .rev()
        .enumerate()
        .fold(0, |acc, (i, v)| acc + v * 26_i32.pow(i as u32))
}
