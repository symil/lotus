use crate::{program::ProgramContext, command_line::CommandLineOptions};
use super::LanguageServerCommandParameters;

pub fn prepare_rename(parameters: &LanguageServerCommandParameters, context: &ProgramContext, lines: &mut Vec<String>) {
    if let (Some(root_directory_path), Some(file_path), Some(cursor_index)) = (&parameters.root_directory_path, &parameters.file_path, parameters.cursor_index) {
        if let Some((shared_identifier, location)) = context.get_identifier_under_cursor(file_path, cursor_index) {
            if shared_identifier.definition.package_root_path == root_directory_path {
                lines.push(format!("placeholder;{};{}", location.start, location.end));
            }
        }
    }
}