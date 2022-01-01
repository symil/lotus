use crate::program::ProgramContext;
use super::LanguageServerCommandParameters;

pub fn provide_definition(parameters: &LanguageServerCommandParameters, context: &ProgramContext, lines: &mut Vec<String>) {
    if let Some((shared_identifier, _)) = parameters.get_shared_identifier_under_cursor(context) {
        if let Some(definition) = &shared_identifier.definition {
            lines.push(format!("definition;{};{}", definition.file_path, definition.start));
        }
    }
}