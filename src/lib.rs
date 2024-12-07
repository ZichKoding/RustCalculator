// Declare a module named `mathematics`.
// This will refer to a `mathematics` directory or file for related code,
// such as arithmetic operations, to keep the code organized.
pub mod mathematics;

/// Rounds a floating-point number to the nearest integer and returns it as an i64.
///
/// # Examples
///
/// ```
/// let rounded = rust_calculator::round_floats(1.4999);
/// assert_eq!(rounded, 1);
///
/// let rounded_up = rust_calculator::round_floats(1.5);
/// assert_eq!(rounded_up, 2);
/// ```
pub fn round_floats(value: f64) -> i64 {
    // Use the built-in `round()` method to round the floating point value.
    // Then convert the result to i64.
    return value.round() as i64;
}

/// Adds two floating-point numbers using a function defined in the `mathematics::addition` module
/// and then rounds the result to the nearest integer.
///
/// # Examples
///
/// ```
/// // Assuming mathematics::addition::add_f64(1.05, 2.45) returns approximately 3.5,
/// // rounding 3.5 would give us 4.
/// let result = rust_calculator::add_return_int(1.05, 2.45);
/// assert_eq!(result, 4);
/// ```
pub fn add_return_int(a: f64, b: f64) -> i64 {
    // Call the addition function from the mathematics module.
    let result = mathematics::addition::add_f64(a, b);
    // Round the resulting float and return as an integer.
    return round_floats(result);
}

/// Creates a vector of key-value pairs from two corresponding vectors of keys and values.
/// If there are more keys than values, missing values are replaced with empty strings.
/// If there are more values than keys, the extra values are ignored.
///
/// # Parameters
///
/// - `keys`: A vector of string slices representing the keys.
/// - `values`: A vector of string slices representing the values.
///
/// # Returns
///
/// A vector of tuples, where each tuple is `(key, value)`.
///
/// # Examples
///
/// ```
/// let keys = vec!["name", "age", "city"];
/// let values = vec!["John Doe", "30"];
/// let pairs = rust_calculator::create_key_value_pairs(keys, values);
/// // The last key "city" does not have a corresponding value, so it gets an empty string.
/// assert_eq!(pairs, vec![("name", "John Doe"), ("age", "30"), ("city", "")]);
/// ```
pub fn create_key_value_pairs<'a>(keys: Vec<&'a str>, values: Vec<&'a str>) -> Vec<(&'a str, &'a str)> {
    // Initialize a new vector to store the resulting key-value pairs.
    let mut object = Vec::new();
    // Iterate over each index of the keys vector.
    for i in 0..keys.len() {
        // Check if the current index exceeds the number of values available.
        if i >= values.len() {
            // No corresponding value: pair the key with an empty string.
            object.push((keys[i], ""));
            continue;
        } else {
            // Corresponding value exists: pair key with the value at the same index.
            object.push((keys[i], values[i]));
        }
    }
    object
}

#[cfg(test)]
mod tests {
    // Import all items from the parent module so we can test them directly.
    use super::*;

    // Test cases for round_floats
    #[test]
    fn test_rounding_floats_down() {
        // Values that should round down to 1
        let values = vec![1.05, 1.35, 1.45, 1.4999];
        let expected = 1;
        for value in values {
            assert_eq!(round_floats(value), expected);
        }
    }

    #[test]
    fn test_round_up_floats() {
        // Values that should round up to 2
        let values = vec![1.5, 1.55, 1.95];
        let expected = 2;
        for value in values {
            assert_eq!(round_floats(value), expected);
        }
    }

    // Test cases for add_return_int
    #[test]
    fn test_add_return_int_floats() {
        // 1.0 + 2.0 = 3.0 -> rounds to 3
        let a = 1.0;
        let b = 2.0;
        let expected = 3;
        assert_eq!(add_return_int(a, b), expected);
    }

    #[test]
    fn test_add_return_int_nonzero_floats() {
        // 1.05 + 2.45 ≈ 3.5 -> rounds to 4
        let a = 1.05;
        let b = 2.45;
        let expected = 4;
        assert_eq!(add_return_int(a, b), expected);

        // 3.1 + 1.399 ≈ 4.499 -> rounds to 4
        let d = 3.1;
        let c = 1.399;
        assert_eq!(add_return_int(d, c), expected);
    }

    // Test cases for create_key_value_pairs
    #[test]
    fn test_create_key_value_pairs() {
        let keys = vec!["name", "age", "city"];
        let values = vec!["John Doe", "30", "New York"];
        let expected = vec![("name", "John Doe"), ("age", "30"), ("city", "New York")];
        assert_eq!(create_key_value_pairs(keys, values), expected);
    }

    #[test]
    fn test_create_key_value_pairs_empty() {
        // Both keys and values are empty, resulting in an empty vector.
        let keys = vec![];
        let values = vec![];
        let expected: Vec<(&str, &str)> = vec![];
        assert_eq!(create_key_value_pairs(keys, values), expected);
    }

    #[test]
    fn test_create_key_value_pairs_mismatch_less_values() {
        // There are fewer values than keys, so the extra key gets paired with an empty string.
        let keys = vec!["name", "age", "city"];
        let values = vec!["John Doe", "30"];
        let expected = vec![("name", "John Doe"), ("age", "30"), ("city", "")];
        assert_eq!(create_key_value_pairs(keys, values), expected);
    }

    #[test]
    fn test_create_key_value_pairs_mismatch_more_values() {
        // There are more values than keys, so the extra value "New York" is ignored.
        let keys = vec!["name", "age"];
        let values = vec!["John Doe", "30", "New York"];
        let expected = vec![("name", "John Doe"), ("age", "30")];
        assert_eq!(create_key_value_pairs(keys, values), expected);
    }
}
