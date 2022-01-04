use crate::program::ProgramContext;
use super::{LanguageServerCommandParameters, LanguageServerCommandOutput};

pub fn provide_completion_items(parameters: &LanguageServerCommandParameters, context: &ProgramContext, output: &mut LanguageServerCommandOutput) {
    if let Some(cursor_index) = parameters.cursor_index {
        if let Some(completion_area) = context.get_autocomplete_area(&parameters.file_path, cursor_index) {
            for item in completion_area.provide_completion_items() {
                output
                    .line("item")
                    .push(item.label)
                    .push_opt(item.kind.map(|kind| kind.to_str()))
                    .push_opt(item.description.as_ref())
                    .push_opt(item.detail.as_ref())
                    .push_opt(item.documentation.as_ref());
            }
        }
    }
}