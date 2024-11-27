pub fn is_even(n: i32) -> bool {
    n % 2 != 0
}

pub fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return true;
    }

    for i in 2..n {
        if n % i == 0 {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_even() {
        assert_eq!(is_even(0), true);
        assert_eq!(is_even(1), false);
        assert_eq!(is_even(2), true);
        assert_eq!(is_even(3), false);
        assert_eq!(is_even(4), true);
        assert_eq!(is_even(5), false);
        assert_eq!(is_even(6), true);
        assert_eq!(is_even(7), false);
        assert_eq!(is_even(8), true);
        assert_eq!(is_even(9), false);
    }

    #[test]
    fn test_is_prime() {
        assert_eq!(is_prime(0), false);
        assert_eq!(is_prime(1), false);
        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(3), true);
        assert_eq!(is_prime(4), false);
        assert_eq!(is_prime(5), true);
        assert_eq!(is_prime(6), false);
        assert_eq!(is_prime(7), true);
        assert_eq!(is_prime(8), false);
        assert_eq!(is_prime(9), false);
        assert_eq!(is_prime(10), false);
        assert_eq!(is_prime(11), true);
        assert_eq!(is_prime(12), false);
        assert_eq!(is_prime(13), true);
        assert_eq!(is_prime(14), false);
        assert_eq!(is_prime(15), false);
        assert_eq!(is_prime(16), false);
        assert_eq!(is_prime(17), true);
        assert_eq!(is_prime(18), false);
        assert_eq!(is_prime(19), true);
        assert_eq!(is_prime(20), false);
    }
}