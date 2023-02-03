//! # 412. Fizz Buzz
//!
//! 给你一个整数 n ，找出从 1 到 n 各个整数的 Fizz Buzz 表示，并用字符串数组 answer（下标从 1 开始）返回结果，其中：
//!
//!   -  answer\[i\] == "FizzBuzz" 如果 i 同时是 3 和 5 的倍数。
//!   -  answer\[i\] == "Fizz" 如果 i 是 3 的倍数。
//!   -  answer\[i\] == "Buzz" 如果 i 是 5 的倍数。
//!   -  answer\[i\] == i （以字符串形式）如果上述条件全不满足。
//!
//! 来源：力扣（LeetCode）
//!
//! 链接：<https://leetcode.cn/problems/fizz-buzz>

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn n_is_3() {
        let n = 3;
        let result = fizz_buzz(n);
        assert_eq!(result, &["1", "2", "Fizz"])
    }

    #[test]
    fn n_is_5() {
        let result = fizz_buzz(5);
        let actual = &["1", "2", "Fizz", "4", "Buzz"];
        assert_eq!(result, actual)
    }

    #[test]
    fn n_is_15() {
        let result = fizz_buzz(15);
        let actual = &[
            "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz", "13",
            "14", "FizzBuzz",
        ];
        assert_eq!(result, actual)
    }
}

pub fn fizz_buzz(n: i32) -> Vec<String> {
    let range = 0..n;
    range
        .into_iter()
        .map(|i| {
            let i = i + 1;
            if i % 15 == 0 {
                "FizzBuzz".to_string()
            } else if i % 5 == 0 {
                "Buzz".to_string()
            } else if i % 3 == 0 {
                "Fizz".to_string()
            } else {
                i.to_string()
            }
        })
        .collect()
}
