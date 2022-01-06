use crate::program::ProgramContext;
use super::{LanguageServerCommandParameters, LanguageServerCommandOutput};

pub fn provide_hover(parameters: &LanguageServerCommandParameters, context: &ProgramContext, output: &mut LanguageServerCommandOutput) {
    if let Some((shared_identifier, location)) = context.get_identifier_under_cursor(&parameters.file_path, parameters.cursor_index) {
        if let Some(ty) = &shared_identifier.type_info {
            output
                .line("hover")
                .push(location.start)
                .push(location.end)
                .push(ty.to_string());
        }
    }
}