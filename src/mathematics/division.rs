fn divide_f64(a: f64, b: f64) -> f64
{
    return a / b;
}


#[cfg(test)]
mod tests {
    use super::*;

    // Test cases for divide_f64
    #[test]
    fn test_divide_f64_floats() {
        let a = 1.0;
        let b = 2.0;
        let expected = 0.5;
        assert_eq!(divide_f64(a, b), expected);
    }
    
    #[test]
    fn test_divide_f64_integers() {
        let a = 1;
        let b = 2;
        let expected = 0.5;
        assert_eq!(divide_f64(a as f64, b as f64), expected);
    }

    #[test]
    fn test_divide_f64_zero_positive() {
        let a = 1.0;
        let b = 0.0;
        let expected = f64::INFINITY;
        assert!(divide_f64(a, b).is_infinite());
        assert_eq!(divide_f64(a, b), expected);
    }

    #[test]
    fn test_divide_f64_zero_negative() {
        let a = -1.0;
        let b = 0.0;
        let expected = f64::NEG_INFINITY;
        assert!(divide_f64(a, b).is_infinite());
        assert_eq!(divide_f64(a, b), expected);
    }
}