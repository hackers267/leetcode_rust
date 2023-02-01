//! 2325.解密消息
//!
//! 给你字符串 key 和 message ，分别表示一个加密密钥和一段加密消息。解密 message 的步骤如下：
//!
//!  1.使用 key 中 26 个英文小写字母第一次出现的顺序作为替换表中的字母 顺序 。
//!  2.将替换表与普通英文字母表对齐，形成对照表。
//!  3.照对照表 替换 message 中的每个字母。
//!  4.格 ' ' 保持不变。
//!
//!  - 例如，key = "happy boy"（实际的加密密钥会包含字母表中每个字母 至少一次），据此，可以得到部分对照表（'h' -> 'a'、'a' -> 'b'、'p' -> 'c'、'y' -> 'd'、'b' -> 'e'、'o' -> 'f'）。
//!
//! 返回解密后的消息。
//!
//!
//!
// 来源：力扣（LeetCode）
// 链接：https://leetcode.cn/problems/decode-the-message

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn message_is_vkbs() {
        let key = String::from("the quick brown fox jumps over the lazy dog");
        let message = String::from("vkbs bs t suepuv");
        let output = decode_message(key, message);
        assert_eq!(output, String::from("this is a secret"))
    }

    #[test]
    fn message_is_long_message() {
        let key = String::from("eljuxhpwnyrdgtqkviszcfmabo");
        let message = String::from("zwx hnfx lqantp mnoeius ycgk vcnjrdb");
        let out = decode_message(key, message);
        let actual = String::from("the five boxing wizards jump quickly");
        assert_eq!(out, actual);
    }
}

use std::collections::HashMap;
pub fn decode_message(key: String, message: String) -> String {
    let v = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];
    let dict = key
        .chars()
        .filter(|&c| c != ' ')
        .fold((0, HashMap::new()), |acc, cur| {
            let (i, mut map) = acc;
            let value = map.get(&cur);
            if value.is_some() {
                (i, map)
            } else {
                map.insert(cur, v[i]);
                (i + 1, map)
            }
        })
        .1;
    let result = message
        .chars()
        .map(|c| dict.get(&c).unwrap_or(&' '))
        .map(|&c| String::from(c))
        .collect::<String>();
    result
}
