#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn title_is_a() {
        let n = 1;
        let title = convert_to_title(n);
        assert_eq!(title, "A")
    }
    #[test]
    fn title_is_ab() {
        let n = 28;
        let title = convert_to_title(n);
        assert_eq!(title, "AB");
    }

    #[test]
    fn title_is_zy() {
        let title = convert_to_title(701);
        assert_eq!(title, "ZY");
    }
    #[test]
    fn title_is_fxshrxw() {
        let title = convert_to_title(2147483647);
        assert_eq!(title, "FXSHRXW")
    }
}

fn get_char(num: i32) -> char {
    (num + 65) as u8 as char
}

pub fn convert_to_title(column_number: i32) -> String {
    let mut div = column_number;
    let mut v = vec![];
    while div > 0 {
        let remind = (div - 1) % 26;
        v.insert(0, remind);
        div = (div - 1) / 26;
    }
    v.iter()
        .map(|v| get_char(*v))
        .fold(String::from(""), |acc, cur| format!("{acc}{cur}"))
}
