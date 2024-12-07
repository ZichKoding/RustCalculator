/// Divides one `f64` floating-point number by another and returns the quotient.
///
/// This function is private, so it will not appear in publicly generated documentation
/// unless `cargo doc --document-private-items` is used. Since it's private, the following
/// example is provided as text-only and will not be compiled as a doc test.
///
/// # Example (as text, not tested)
///
/// ```text
/// // This is not an executable doc test, just an illustration.
/// // If the function were public, we might call it like:
/// // let result = rust_calculator::mathematics::division::divide_f64(4.0, 2.0);
/// // assert_eq!(result, 2.0);
/// ```
fn divide_f64(a: f64, b: f64) -> f64 {
    // Return the result of dividing `a` by `b`.
    // Note: Dividing by zero will result in `f64::INFINITY` or `f64::NEG_INFINITY`,
    // depending on the sign of the numerator.
    a / b
}

#[cfg(test)]
mod tests {
    // Bring divide_f64 into the test scope.
    use super::*;

    // Test cases for divide_f64
    #[test]
    fn test_divide_f64_floats() {
        // Test dividing two floating-point values.
        let a = 1.0;
        let b = 2.0;
        let expected = 0.5;
        // Check that the division result is as expected.
        assert_eq!(divide_f64(a, b), expected);
    }
    
    #[test]
    fn test_divide_f64_integers() {
        // Even though the function expects f64, we can pass integers by casting.
        let a = 1;
        let b = 2;
        let expected = 0.5;
        // Convert integers to f64 before division and verify the result.
        assert_eq!(divide_f64(a as f64, b as f64), expected);
    }

    #[test]
    fn test_divide_f64_zero_positive() {
        // Dividing by zero results in positive infinity if the numerator is positive.
        let a = 1.0;
        let b = 0.0;
        let expected = f64::INFINITY;
        // Check that the result is infinite and matches the expected infinity sign.
        assert!(divide_f64(a, b).is_infinite());
        assert_eq!(divide_f64(a, b), expected);
    }

    #[test]
    fn test_divide_f64_zero_negative() {
        // Dividing by zero results in negative infinity if the numerator is negative.
        let a = -1.0;
        let b = 0.0;
        let expected = f64::NEG_INFINITY;
        // Check that the result is infinite and matches the expected negative infinity.
        assert!(divide_f64(a, b).is_infinite());
        assert_eq!(divide_f64(a, b), expected);
    }
}
