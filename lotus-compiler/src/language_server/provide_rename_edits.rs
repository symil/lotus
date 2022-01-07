use std::collections::HashSet;

use crate::{program::{ProgramContext}, command_line::CommandLineOptions, utils::is_valid_identifier};
use super::{LanguageServerCommandParameters, LanguageServerCommandOutput, is_special_identifier};

pub fn provide_rename_edits(parameters: &LanguageServerCommandParameters, context: &ProgramContext, output: &mut LanguageServerCommandOutput) {
    let new_name = parameters.payload.as_ref().map(|s| s.as_str()).unwrap_or("");

    if is_valid_identifier(new_name) {
        if let Some((shared_identifier, _)) = context.get_identifier_under_cursor(&parameters.file_path, parameters.cursor_index) {
            for occurence in shared_identifier.get_all_occurences() {
                if !is_special_identifier(occurence.as_str()) {
                    output
                        .line("replace")
                        .push(&occurence.file.path)
                        .push(occurence.start)
                        .push(occurence.end)
                        .push(new_name);
                } else {
                    dbg!(occurence);
                }
            }
        }
    }
}
