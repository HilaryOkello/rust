pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) / (9.0 / 5.0)
}
pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f_to_c() {
        let temp_f = 20.0;
        let expected_c = -6.666666666666666;
        let result_c = fahrenheit_to_celsius(temp_f);
        assert!((result_c - expected_c).abs() < 1e-10, "Expected {}, got {}", expected_c, result_c);
    }
}
