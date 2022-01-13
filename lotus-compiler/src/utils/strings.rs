pub fn is_valid_identifier(name: &str) -> bool {
    let bytes = name.as_bytes();

    if name.is_empty() {
        return false;
    }

    if !is_alpha_char(bytes[0]) {
        return false;
    }

    for c in &bytes[1..] {
        if !is_alpha_char(*c) && !is_digit_char(*c) {
            return false;
        }
    }

    true
}

pub fn contains_valid_identifier_character(string: &str) -> bool {
    for c in string.as_bytes() {
        if is_alpha_char(*c) || is_digit_char(*c) {
            return true;
        }
    }

    return false;
}

fn is_alpha_char(c: u8) -> bool {
    c == b'_' || (c >= b'a' && c <= b'z') || (c >= b'A' && c <= b'Z')
}

fn is_digit_char(c: u8) -> bool {
    (c >= b'0' && c <= b'9')
}

pub fn is_blank_string(string: &str) -> bool {
    for c in string.as_bytes() {
        if !is_blank_character(*c) {
            return false;
        }
    }

    true
}

fn is_blank_character(c: u8) -> bool {
    c == b' ' || c == b'\n' || c == b'\r' || c == b'\t'
}