use crate::{program::ProgramContext, language_server::{LanguageServerCommandParameters, LanguageServerCommandOutput}};

pub fn provide_definition(parameters: &LanguageServerCommandParameters, context: &ProgramContext, output: &mut LanguageServerCommandOutput) {
    if let Some(definition) = context.definition_provider.get_definition() {
        output
            .line("definition")
            .push(&definition.target_location.file.path)
            .push(definition.target_location.start);
    }
}