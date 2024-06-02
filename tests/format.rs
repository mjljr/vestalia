#[cfg(test)]
mod tests {
    static PASSING_STRING_LEFT: &str = "abcdefghijklmnop";
    static PASSING_STRING_CENTER: &str = "qrstuvwxyz1234567890";
    static PASSING_STRING_RIGHT: &str = "!@#$()-+&=;:'\"%,./?°";
    static PASSING_STRING_DEFAULT: &str = "My text";
    static FAILING_STRING: &str = "My text****";
    #[test]
    fn test_convert_line_valid_text_left() {
        let test_text = PASSING_STRING_LEFT.to_string();
        let result = vestalia::format::convert_line(test_text, "left").unwrap();
        let expected = vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 0, 0, 0, 0, 0, 0,
        ];
        assert_eq!(expected, result);
    }
    #[test]
    fn test_convert_line_valid_text_center() {
        let test_text = PASSING_STRING_CENTER.to_string();
        let result = vestalia::format::convert_line(test_text, "center").unwrap();
        let expected = vec![
            0, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 0,
        ];
        assert_eq!(expected, result);
    }
    #[test]
    fn test_convert_line_valid_text_right() {
        let test_text = PASSING_STRING_RIGHT.to_string();
        let result = vestalia::format::convert_line(test_text, "right").unwrap();
        let expected = vec![
            0, 0, 37, 38, 39, 40, 41, 42, 44, 46, 47, 48, 49, 50, 52, 53, 54, 55, 56, 59, 60, 62,
        ];
        assert_eq!(expected, result);
    }
    #[test]
    fn test_convert_line_valid_text_default() {
        let test_text = PASSING_STRING_DEFAULT.to_string();
        let result = vestalia::format::convert_line(test_text, "default").unwrap();
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
