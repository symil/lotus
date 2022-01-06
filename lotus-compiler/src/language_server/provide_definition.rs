use crate::program::ProgramContext;
use super::{LanguageServerCommandParameters, LanguageServerCommandOutput};

pub fn provide_definition(parameters: &LanguageServerCommandParameters, context: &ProgramContext, output: &mut LanguageServerCommandOutput) {
    if let Some((shared_identifier, _)) = context.get_identifier_under_cursor(&parameters.file_path, parameters.cursor_index) {
        if let Some(definition) = &shared_identifier.definition {
            output
                .line("definition")
                .push(&definition.file.path)
                .push(definition.start);
        }
    }
}