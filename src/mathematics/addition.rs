pub fn add_f64(a: f64, b: f64) -> f64
{
    return a + b;
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test cases for add_f64
    #[test]
    fn test_add_f64_floats() {
        let a = 1.0;
        let b = 2.0;
        let expected = 3.0;
        assert_eq!(add_f64(a, b), expected);
    }

    #[test]
    fn test_add_f64_integers() {
        let a = 1;
        let b = 2;
        let expected = 3.0;
        assert_eq!(add_f64(a as f64, b as f64), expected);
    }
}