#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn result_is_two() {
        let s = "l|*e*et|c**o|*de|".to_string();
        let result = count_asterisks(s);
        assert_eq!(result, 2);
    }

    #[test]
    fn result_is_zero() {
        let s = "iamprogrammer".to_string();
        assert_eq!(count_asterisks(s), 0);
    }

    #[test]
    fn result_is_five() {
        let s = "yo|uar|e**|b|e***au|tifu|l".to_string();
        assert_eq!(count_asterisks(s), 5);
    }
}

pub fn count_asterisks(s: String) -> i32 {
    s.chars()
        .fold((0, true), |(count, add), c| {
            (
                count + if add && c == '*' { 1 } else { 0 },
                if c == '|' { !add } else { add },
            )
        })
        .0
}
