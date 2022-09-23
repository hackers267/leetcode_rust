#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn is_palindrome_easy_true_test() {
        let x = 12321;
        let result = is_palindrome(x);
        assert!(result);
    }
    #[test]
    fn is_palindrome_easy_false_test() {
        let x = 12345;
        let result = is_palindrome(x);
        assert!(!result);
    }
    #[test]
    fn is_palindrome_with_zero() {
        let result = is_palindrome(0);
        assert!(result);
    }
    #[test]
    fn is_palindrome_with_five() {
        let result = is_palindrome(5);
        assert!(result);
    }
    #[test]
    fn is_palindrome_with_hundred() {
        let result = is_palindrome(100);
        assert!(!result);
    }
    #[test]
    fn is_palindrome_negative() {
        let result = is_palindrome(-1);
        assert!(!result)
    }
}
pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    let result = get_reverse(x);
    result == x
}

fn get_reverse(m:i32)->i32{
    m.to_string()
        .split("")
        .filter_map(|x| {
            match x.is_empty() {
                false=>Some(x.to_string()),
                true=>None
            }
        })
        .enumerate()
        .fold(0i32,|acc,(index,value)|{
            let i:i32 = value.parse().unwrap();
            acc+ i * 10_i32.pow(index as u32)
        })
}