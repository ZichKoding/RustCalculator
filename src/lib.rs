mod addition;
mod subtraction;
mod multiplication;
mod division;

pub fn round_floats(value: f64) -> i64
{
    return value.round() as i64;
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test cases for round_floats
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
