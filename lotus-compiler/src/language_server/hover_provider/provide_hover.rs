use crate::{program::ProgramContext, language_server::{LanguageServerCommandParameters, LanguageServerCommandOutput}};

pub fn provide_hover(parameters: &LanguageServerCommandParameters, context: &ProgramContext, output: &mut LanguageServerCommandOutput) {
    if let Some(hover) = context.hover_provider.get_hover() {
        if let Some(ty) = &hover.ty {
            let location = &hover.location;

            output
                .line("hover")
                .push(location.start)
                .push(location.end)
                .push(ty.to_string());
        }
    }
}