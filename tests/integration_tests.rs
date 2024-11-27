use lab5::string_manipulator::{concatenate, reverse, to_int};
use lab5::number_checker::{is_prime, is_even};
use lab5::arithmetic::{add, div};

#[test]
fn test_arithmetic_and_number_checker() {
    let sum = add(1, 2);
    assert_eq!(sum, 3);
    assert_eq!(is_prime(sum), true);
    assert_eq!(is_even(sum), false);

    let quotient = div(3, 2);
    assert_eq!(quotient, Ok(1.5));
    assert_eq!(is_prime((quotient.unwrap() as i32) * 2), true);
}

#[test]
fn test_string_manipulator_and_number_checker() {
    let s1 = "Hello";
    let s2 = "5World";
    let concatenated = concatenate(s1, s2);
    assert_eq!(concatenated, "Hello5World");
    assert_eq!(to_int(&concatenated), Err("Invalid number".to_string()));

    let s3 = "123";
    let reversed = reverse(s3);
    assert_eq!(reversed, "321");
    assert_eq!(to_int(&reversed), Ok(321));
}