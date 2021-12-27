use crate::{program::ProgramContext, command_line::CommandLineOptions};

pub fn provide_rename_edits(context: &mut ProgramContext, options: &CommandLineOptions) -> Vec<String> {
    let mut result = vec![];

    if let (Some(cursor), Some(new_name)) = (options.cursor, &options.new_name) {
        if is_valid_name(new_name) {
            if let Some((shared_identifier, _)) = context.get_identifier_under_cursor(&options.input_path, cursor) {
                for occurence in shared_identifier.get_all_occurences() {
                    result.push(format!("replace;{};{};{};{}", occurence.file_path, occurence.start, occurence.end, new_name));
                }
            }
        }
    }

    result
}

fn is_valid_name(name: &str) -> bool {
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