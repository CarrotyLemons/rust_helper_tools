#[macro_export]
macro_rules! assert_flexible {
    ($value1:expr, $value2:expr, $percent:expr) => {{
        // Convert percentage to a ratio
        let ratio = $percent / 100.0;
        let result = ($value1 >= $value2 * (1.0 - ratio) && $value1 <= $value2 * (1.0 + ratio));

        assert!(result, "Assertion failed: {} !~= {}", $value1, $value2);
    }};
}
