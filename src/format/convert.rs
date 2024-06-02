/// Maps a character to the Vestaboard
/// [Character Code Reference](https://docs.vestaboard.com/characters)
///
/// # Arguments
///
/// * `character` - A `char` type containing the character to map to a character code
pub fn char_to_int(character: char) -> i32 {
    match character {
        ' ' => 0,
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        '1' => 27,
        '2' => 28,
        '3' => 29,
        '4' => 30,
        '5' => 31,
        '6' => 32,
        '7' => 33,
        '8' => 34,
        '9' => 35,
        '0' => 36,
        '!' => 37,
        '@' => 38,
        '#' => 39,
        '$' => 40,
        '(' => 41,
        ')' => 42,
        // No 45 mapping
        '-' => 44,
        // No 45 mapping
        '+' => 46,
        '&' => 47,
        '=' => 48,
        ';' => 49,
        ':' => 50,
        '\'' => 52,
        '\"' => 53,
        '%' => 54,
        ',' => 55,
        '.' => 56,
        // No 57, 58 mapping
        '/' => 59,
        '?' => 60,
        // No 61 mapping
        '°' => 62,
        _ => 0,
    }
}