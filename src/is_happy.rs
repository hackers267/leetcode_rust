#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn is_happy_19() {
        assert!(is_happy(19));
    }

    #[test]
    fn not_happy_2() {
        assert!(!is_happy(2));
    }

    #[test]
    fn not_happy_6() {
        assert!(!is_happy(6));
    }
}

use std::collections::HashMap;
pub fn is_happy(n: i32) -> bool {
    let mut hash_map = HashMap::new();
    let mut n = n;
    while n != 1 {
        let value = hash_map.get(&n);
        if value.is_some() {
            return false;
        }
        hash_map.insert(n, 1);
        println!("{n}");
        n = calc(n)
    }
    true
}

fn calc(n: i32) -> i32 {
    format!("{n}")
        .split("")
        .filter_map(|x| x.parse::<i32>().ok())
        .fold(0, |acc, cur| acc + cur.pow(2))
}
