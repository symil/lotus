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

fn is_alpha_char(c: u8) -> bool {
    c == b'_' || (c >= b'a' && c <= b'z') || (c >= b'A' && c <= b'Z')
}

fn is_digit_char(c: u8) -> bool {
    (c >= b'0' && c <= b'9')
}