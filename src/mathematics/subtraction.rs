/// Subtracts the second `f64` floating-point number from the first and returns the result.
///
/// # Examples
///
/// ```
/// // Using the subtract_f64 function to subtract two floating-point values:
/// let result = rust_calculator::mathematics::subtraction::subtract_f64(1.0, 2.0);
/// assert_eq!(result, -1.0);
/// ```
///
/// The function will also work for integers converted to `f64`:
///
/// ```
/// let result = rust_calculator::mathematics::subtraction::subtract_f64(1.0_f64, 2.0_f64);
/// assert_eq!(result, -1.0);
/// ```
pub fn subtract_f64(a: f64, b: f64) -> f64 {
    // Return the difference between `a` and `b` as a floating-point number.
    return a - b;
}

#[cfg(test)]
mod tests {
    // Bring the subtract_f64 function into the current scope for testing.
    use super::*;

    // Test cases for subtract_f64
    #[test]
    fn test_subtract_f64_floats() {
        // Test subtracting two floating-point values directly.
        let a = 1.0;
        let b = 2.0;
        let expected = -1.0;
        // Check that the result matches the expected value.
        assert_eq!(subtract_f64(a, b), expected);
    }

    #[test]
    fn test_subtract_f64_integers() {
        // Even though the function takes f64, we can pass integers by casting.
        let a = 1;
        let b = 2;
        let expected = -1.0;
        // Convert integers to f64 before subtracting and verify the result.
        assert_eq!(subtract_f64(a as f64, b as f64), expected);
    }
}
