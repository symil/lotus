use crate::{program::{ProgramContext, EVENT_VAR_NAME, SELF_VAR_NAME, SELF_TYPE_NAME}, command_line::CommandLineOptions};
use super::LanguageServerCommandParameters;

pub fn prepare_rename(parameters: &LanguageServerCommandParameters, context: &ProgramContext, lines: &mut Vec<String>) {
    if let (Some(root_directory_path), Some(file_path), Some(cursor_index)) = (&parameters.root_directory_path, &parameters.file_path, parameters.cursor_index) {
        if let Some((shared_identifier, location)) = context.get_identifier_under_cursor(file_path, cursor_index) {
            if !is_special_identifier(location.as_str()) {
                if let Some(definition) = &shared_identifier.definition {
                    if definition.package_root_path.as_str() == root_directory_path {
                        lines.push(format!("placeholder;{};{}", location.start, location.end));
                    }
                }
            }
        }
    }
}

fn is_special_identifier(name: &str) -> bool {
    match name {
        EVENT_VAR_NAME => true,
        SELF_VAR_NAME => true,
        SELF_TYPE_NAME => true,
        _ => false
    }
}