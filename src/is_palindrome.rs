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
    let mut y: Vec<i32> = div(x);
    fn div(y: i32) -> Vec<i32> {
        let mut vec: Vec<i32> = vec![];
        let i = y % 10;
        vec.push(i);
        let m = y / 10;
        if m != 0 {
            let v = div(m);
            vec.extend_from_slice(&v);
        }
        vec
    }
    y.reverse();
    let result = y.iter().enumerate().fold(0, |acc, (index, value)| {
        acc + value * 10i32.pow(index as u32)
    });
    result == x
}
