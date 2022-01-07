use crate::program::ProgramContext;
use super::{LanguageServerCommandParameters, LanguageServerCommandOutput};

pub fn provide_hover(parameters: &LanguageServerCommandParameters, context: &ProgramContext, output: &mut LanguageServerCommandOutput) {
    if let Some(area) = context.hover_provider.get_area_under_cursor(&parameters.file_path, parameters.cursor_index) {
        if let Some(ty) = area.get_type() {
            let location = area.get_location();

            output
                .line("hover")
                .push(location.start)
                .push(location.end)
                .push(ty.to_string());
        }
    }
}