use crate::program::ProgramContext;
use super::{LanguageServerCommandParameters, LanguageServerCommandOutput};

pub fn provide_hover(parameters: &LanguageServerCommandParameters, context: &ProgramContext, output: &mut LanguageServerCommandOutput) {
    if let Some((shared_identifier, location)) = parameters.get_shared_identifier_under_cursor(context) {
        if let Some(ty) = &shared_identifier.type_info {
            output
                .line("hover")
                .push(location.start)
                .push(location.end)
                .push(ty.to_string());
        }
    }
}