mod mathematics;

pub fn round_floats(value: f64) -> i64
{
    return value.round() as i64;
}

pub fn add_return_int(a: f64, b: f64) -> i64
{
    let result = mathematics::addition::add_f64(a, b);
    return round_floats(result);
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

    // Test cases for add_return_int
    #[test]
    fn test_add_return_int_floats() {
        let a = 1.0;
        let b = 2.0;
        let expected = 3;
        assert_eq!(add_return_int(a, b), expected);
    }

    #[test]
    fn test_add_return_int_nonzero_floats() {
        let a = 1.05;
        let b = 2.45;
        let d = 3.1;
        let c = 1.399;
        let expected = 4;
        assert_eq!(add_return_int(a as f64, b as f64), expected);
        assert_eq!(add_return_int(d as f64, c as f64), expected);
    }
}
