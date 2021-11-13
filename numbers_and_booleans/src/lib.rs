fn is_even(number: u32) -> bool {
    number % 2 == 0
}

fn hypot(first_side: f64, second_side: f64) -> f64 {
    (f64::powi(first_side, 2) + f64::powi(second_side, 2)).sqrt()
}

#[test]
fn test_is_even() {
    assert!(is_even(4));
}

#[test]
fn test_is_odd() {
    assert_eq!(is_even(7), false);
}

#[test]
fn test_hypot() {
    assert_eq!(hypot(2.5, 3.1), 3.9824615503479754)
}
