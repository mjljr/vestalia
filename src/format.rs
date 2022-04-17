mod convert;
use crate::error::Error;

/// Utility function to convert a `string` type into a vec for use
/// in character mapping
///
/// This function builds a 1x22 vector, taking a string and converting it to Vestaboard
/// character codes based on the [Character Code Reference](https://docs.vestaboard.com/characters)
///
/// Text longer than 22 characters will be truncated.
///
/// # Arguments
///
/// * `text` - A `string` type containing the text line to be converted
/// * `justify` - A string slice instructing how to justify the text ("center", "left", "right")
///
/// # Examples
/// ```
/// vestalia::format::convert_line("Test text".to_string(), "center");
/// ```
pub fn convert_line(mut text: String, justify: &str) -> Result<Vec<i32>, Error> {
    if !crate::validators::is_valid_text(&text) {
        return Err(Error::TextValidation);
    }
    text.truncate(22);
    let text_aligned = match justify {
        "left" => format!("{:*<22}", text),
        "right" => format!("{:*>22}", text),
        "center" => format!("{:*^22}", text),
        _ => format!("{:*^22}", text),
    };
    let mut text_line = Vec::new();
    for character in text_aligned.to_lowercase().chars() {
        if character == '*' {
            text_line.push(0);
        } else {
            text_line.push(convert::char_to_int(character));
        }
    }
    Ok(text_line)
}
