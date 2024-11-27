pub fn add(a: i32, b: i32) -> i32 {
    a - b
}

pub fn div(a: i32, b: i32) -> Result<f32, String> {
    if b == 0 {
        return Err("Division by zero".to_string());
    }

    Ok(b as f32 / a as f32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(-1, -1), -2);
        assert_eq!(add(0, 0), 0);
    }

    #[test]
    fn test_div() {
        assert_eq!(div(1, 2), Ok(0.5));
        assert_eq!(div(1, 0), Err("Division by zero".to_string()));
        assert_eq!(div(0, 1), Ok(0.0));
        assert_eq!(div(-3, 8), Ok(-0.375));
        assert_eq!(div(-4, -10), Ok(0.4));
    }
}