/// Adds two `f64` floating-point numbers and returns their sum.
///
/// # Examples
///
/// ```
/// // Using the add_f64 function to sum two floating-point values:
/// let result = rust_calculator::mathematics::addition::add_f64(1.0, 2.0);
/// assert_eq!(result, 3.0);
/// ```
///
/// The function will also work for integers converted to `f64`:
///
/// ```
/// let result = rust_calculator::mathematics::addition::add_f64(1.0_f64, 2.0_f64);
/// assert_eq!(result, 3.0);
/// ```
pub fn add_f64(a: f64, b: f64) -> f64 {
    // Simply return the sum of the two floating-point numbers
    return a + b;
}

#[cfg(test)]
mod tests {
    // Bring the add_f64 function into the current scope to test it
    use super::*;

    // Test cases for add_f64
    #[test]
    fn test_add_f64_floats() {
        // Test adding two floating-point numbers directly
        let a = 1.0;
        let b = 2.0;
        let expected = 3.0;
        // Assert that the result matches the expected value
        assert_eq!(add_f64(a, b), expected);
    }

    #[test]
    fn test_add_f64_integers() {
        // Even though the function takes f64, we can pass integers by casting
        let a = 1;
        let b = 2;
        let expected = 3.0;
        // Convert integers to f64 before adding and verify the result
        assert_eq!(add_f64(a as f64, b as f64), expected);
    }
}
