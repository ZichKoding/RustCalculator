/// Multiplies two `f64` floating-point numbers and returns their product.
///
/// This function is private, so it will not appear in publicly generated documentation
/// unless `cargo doc --document-private-items` is used. Since it's private, the following
/// example is provided as text-only and will not be compiled as a doc test.
///
/// # Example (as text, not tested)
///
/// ```text
/// let result = rust_calculator::mathematics::multiplication::multiply_f64(2.0, 3.0);
/// assert_eq!(result, 6.0);
/// ```
fn multiply_f64(a: f64, b: f64) -> f64 {
    // Return the product of `a` and `b` as a floating-point number.
    a * b
}

#[cfg(test)]
mod tests {
    // Import the multiply_f64 function into the test module's scope
    use super::*;

    // Test cases for multiply_f64
    #[test]
    fn test_multiply_f64_floats() {
        // Test multiplying two floating-point values directly.
        let a = 1.0;
        let b = 2.0;
        let expected = 2.0;
        // Check that the result matches the expected value.
        assert_eq!(multiply_f64(a, b), expected);
    }

    #[test]
    fn test_multiply_f64_integers() {
        // Although multiply_f64 expects f64 values, we can pass integers by casting.
        let a = 1;
        let b = 2;
        let expected = 2.0;
        // Convert integers to f64 before multiplication and verify the result.
        assert_eq!(multiply_f64(a as f64, b as f64), expected);
    }
}
