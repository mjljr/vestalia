#[cfg(test)]
mod tests {
    static PASSING_STRING: &str = "My text";
    static FAILING_STRING: &str = "My text****";
    #[test]
    fn test_convert_line_valid_text() {
        let test_text = PASSING_STRING.to_string();
        let result = vestalia::format::convert_line(test_text, "center").unwrap();
        let expected = vec![
            0, 0, 0, 0, 0, 0, 0, 13, 25, 0, 20, 5, 24, 20, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        assert_eq!(expected, result);
    }
    #[test]
    fn test_convert_line_invalid_text() {
        let test_text = FAILING_STRING.to_string();
        let result = vestalia::format::convert_line(test_text, "center");
        assert!(result.is_err());
    }
}
