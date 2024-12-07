pub fn add_f64(a: f64, b: f64) -> f64
{
    return a + b;
}

pub fn round_floats(value: f64) -> i64
{
    return value.round() as i64;
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_rounding_floats_down() {
        let values = vec![1.05, 1.35, 1.45, 1.4999];
        let expected = 1;
        for value in values {
            assert_eq!(round_floats(value), expected);
        }
    }

    #[test]
    fn test_round_up_floats() {
        let values = vec![1.5, 1.55, 1.95];
        let expected = 2;
        for value in values {
            assert_eq!(round_floats(value), expected);
        }
    }
}
