use crate::{program::ProgramContext, language_server::{LanguageServerCommandParameters, LanguageServerCommandOutput}};

pub fn provide_definition(parameters: &LanguageServerCommandParameters, context: &ProgramContext, output: &mut LanguageServerCommandOutput) {
    if let Some(area) = context.hover_provider.get_area_under_cursor(&parameters.file_path, parameters.cursor_index) {
        if let Some(definition) = area.get_definition() {
            output
                .line("definition")
                .push(&definition.file.path)
                .push(definition.start);
        }
    }
}