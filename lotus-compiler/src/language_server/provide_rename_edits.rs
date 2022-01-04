use crate::{program::{ProgramContext}, command_line::CommandLineOptions};
use super::{LanguageServerCommandParameters, LanguageServerCommandOutput};

pub fn provide_rename_edits(parameters: &LanguageServerCommandParameters, context: &ProgramContext, output: &mut LanguageServerCommandOutput) {
    if let (Some(cursor_index), Some(new_name)) = (parameters.cursor_index, &parameters.payload) {
        if is_valid_name(new_name) {
            if let Some((shared_identifier, _)) = context.get_identifier_under_cursor(&parameters.file_path, cursor_index) {
                for occurence in shared_identifier.get_all_occurences() {
                    output
                        .line("replace")
                        .push(occurence.file_path)
                        .push(occurence.start)
                        .push(occurence.end)
                        .push(new_name);
                }
            }
        }
    }
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