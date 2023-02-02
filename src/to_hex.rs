//! 405.数字转换为十六进制数
//!
//! 给定一个整数，编写一个算法将这个数转换为十六进制数。对于负整数，我们通常使用 补码运算 方法。
//!
//! 注意:
//!
//!    1. 十六进制中所有字母(a-f)都必须是小写。
//!    2. 十六进制字符串中不能包含多余的前导零。如果要转化的数为0，那么以单个字符'0'来表示；对于其他情况，十六进制字符串中的第一个字符将不会是0字符。
//!    3. 给定的数确保在32位有符号整数范围内。
//!    4. 不能使用任何由库提供的将数字直接转换或格式化为十六进制的方法。
//!
//! 来源：力扣（LeetCode）
//!
//! 链接：<https://leetcode.cn/problems/convert-a-number-to-hexadecimal>

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn to_1a() {
        assert_eq!(to_hex(26), String::from("1a"))
    }

    #[test]
    fn to_4d2() {
        assert_eq!(to_hex(1234), String::from("4d2"))
    }

    #[test]
    fn to_ffffffff() {
        assert_eq!(to_hex(-1), String::from("ffffffff"));
    }
}

pub fn to_hex(num: i32) -> String {
    if num == 0 {
        return String::from("0");
    }
    let vec = [
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "a", "b", "c", "d", "e", "f",
    ];
    let range = 0..=7;
    let mut str = String::from("");
    let mut great_zero = false;
    for i in range {
        let v = num >> (4 * (7 - i)) & 0xf;
        if v > 0 {
            great_zero = true;
        }
        if great_zero {
            let s = vec[v as usize];
            str = format!("{str}{s}")
        }
    }
    str
}
