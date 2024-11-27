pub fn concatenate(a: &str, b: &str) -> String {
    format!("{}{}", a, b)
}

pub fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

pub fn to_int(s: &str) -> Result<i32, String> {
    match s.parse::<i32>() {
        Ok(n) => Ok(n),
        Err(_) => Err("Invalid number".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concatenate() {
        assert_eq!(concatenate("Hello", "World"), "HelloWorld");
        assert_eq!(concatenate("Hello", ""), "Hello");
        assert_eq!(concatenate("", "World"), "World");
        assert_eq!(concatenate("", ""), "");
    }

    #[test]
    fn test_reverse() {
        assert_eq!(reverse("Hello"), "olleH");
        assert_eq!(reverse("World"), "dlroW");
        assert_eq!(reverse("Hello, World!"), "!dlroW ,olleH");
        assert_eq!(reverse(""), "");
    }

    #[test]
    fn test_to_int() {
        assert_eq!(to_int("123"), Ok(123));
        assert_eq!(to_int("-123"), Ok(-123));
        assert_eq!(to_int("0"), Ok(0));
        assert_eq!(to_int("abc"), Err("Invalid number".to_string()));
        assert_eq!(to_int("123abc"), Err("Invalid number".to_string()));
        assert_eq!(to_int("abc123"), Err("Invalid number".to_string()));
        assert_eq!(to_int(""), Err("Invalid number".to_string()));
    }
}