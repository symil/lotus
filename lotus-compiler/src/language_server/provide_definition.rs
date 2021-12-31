use crate::program::ProgramContext;
use super::LanguageServerCommandParameters;

pub fn provide_definition(parameters: &LanguageServerCommandParameters, context: &ProgramContext, lines: &mut Vec<String>) {
    if let (Some(file_path), Some(cursor_index)) = (&parameters.file_path, parameters.cursor_index) {
        if let Some((shared_identifier, _)) = context.get_identifier_under_cursor(file_path, cursor_index) {
            let definition = &shared_identifier.definition;

            lines.push(format!("definition;{};{}", definition.file_path, definition.start));
        }
    }
}