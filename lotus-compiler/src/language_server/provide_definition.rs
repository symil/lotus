use crate::program::ProgramContext;
use super::{LanguageServerCommandParameters, LanguageServerCommandOutput};

pub fn provide_definition(parameters: &LanguageServerCommandParameters, context: &ProgramContext, output: &mut LanguageServerCommandOutput) {
    if let Some((shared_identifier, _)) = parameters.get_shared_identifier_under_cursor(context) {
        if let Some(definition) = &shared_identifier.definition {
            output
                .line("definition")
                .push(&definition.file_path)
                .push(definition.start);
        }
    }
}