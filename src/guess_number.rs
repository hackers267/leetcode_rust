//! 374.猜数字大小
//!
//!猜数字游戏的规则如下：
//!
//!  - 每轮游戏，我都会从 1 到 n 随机选择一个数字。 请你猜选出的是哪个数字。
//!  - 如果你猜错了，我会告诉你，你猜测的数字比我选出的数字是大了还是小了。
//!
//! 你可以通过调用一个预先定义好的接口 int guess(int num) 来获取猜测结果，返回值一共有 3 种可能的情况（-1，1 或 0）：
//!
//!  - -1：我选出的数字比你猜的数字小 pick < num
//!  - 1：我选出的数字比你猜的数字大 pick > num
//!  - 0：我选出的数字和你猜的数字一样。恭喜！你猜对了！pick == num
//!
//! 返回我选出的数字。
//!
//! 来源：力扣（LeetCode）
//!
//! 链接：<https://leetcode.cn/problems/guess-number-higher-or-lower>

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn pick_is_6() {
        assert_eq!(guess_number(10, 6), 6)
    }

    #[test]
    fn pick_is_2() {
        assert_eq!(guess_number(2, 2), 2);
    }
}

fn calc_mid(hi: i32, lo: i32) -> i32 {
    lo + (hi - lo) / 2
}

use std::cmp::Ordering;
fn guess(n: i32, pick: i32) -> i32 {
    match n.cmp(&pick) {
        Ordering::Equal => 0,
        Ordering::Greater => -1,
        Ordering::Less => 1,
    }
}

fn guess_number(n: i32, pick: i32) -> i32 {
    let mut hi = n;
    let mut lo = 1;
    let mut mid = calc_mid(hi, lo);
    while lo <= hi {
        mid = calc_mid(hi, lo);
        let v = guess(mid, pick);
        println!("{v},{mid},{pick}");
        if v == 0 {
            break;
        }
        if v == -1 {
            hi = mid - 1;
        }
        if v == 1 {
            lo = mid + 1;
        }
    }
    mid
}
