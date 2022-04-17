use regex::Regex;

/// Validates that valid strings are being passed to the Vestaboard API.
///
/// Uses regex to determine if valid characters are being sent to the Vestaboard. There is limited
/// character support in the [Character Code Reference](https://docs.vestaboard.com/characters),
/// so this function helps ensure valid strings are being sent. If the message fails regex,
/// false is returned. If it passes, true is returned.
///
/// # Arguments
///
/// * `text` - A string slice type that holds the text message
pub fn is_valid_text(text: &str) -> bool {
    let re = Regex::new(r#"^(?:[A-Za-z\d!@#$()\-+&=;:'""%,./?°\s]|(?:\{\d{1,2}\}))*$"#).unwrap();
    re.is_match(text)
}

/// Validates that the character `Vec` size is correct (6x22).
///
/// The Vestaboard API expects a 6x22 array, so this function validates that input.
/// If the message fails the tests, false is returned. If it passes, true is returned.
///
/// # Arguments
///
/// * `characters` - A `Vec<Vec<i32>>` type that holds the character codes
pub fn is_valid_vec(characters: &Vec<Vec<i32>>) -> bool {
    if characters.len() != 6 {
        return false;
    }
    for row in characters.iter() {
        if row.len() != 22 {
            return false;
        }
    }
    true
}
