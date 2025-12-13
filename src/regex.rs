pub fn is_shorthand_class(char: char) -> bool {
    matches!(char, 'd' | 'D' | 'w' | 'W' | 's' | 'S')
}

pub fn is_control_char(char: char) -> bool {
    matches!(char, 't' | 'r' | 'n' | '0')
}

pub fn control_to_literal(char: char) -> Option<char> {
    match char {
        't' => Some('\t'),
        'r' => Some('\r'),
        'n' => Some('\n'),
        '0' => Some('\0'),
        _ => None,
    }
}

pub fn is_escaped_literal(char: char) -> bool {
    matches!(
        char,
        '\\' | '|' | '*' | '+' | '?' | '{' | '}' | '[' | ']' | '(' | ')' | '-' | '.' | '^' | '$'
    )
}
