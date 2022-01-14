use crate::{program::ProgramContext, utils::is_blank_string, language_server::{LanguageServerCommandParameters, LanguageServerCommandOutput}};

pub fn provide_completion_items(parameters: &LanguageServerCommandParameters, context: &ProgramContext, output: &mut LanguageServerCommandOutput) {
    let completion_items = context.completion_provider.as_ref().unwrap().get_completion_items();
    let range : Option<String> = None;

    for item in completion_items {
        output
            .line("item")
            .push(item.label)
            .push(item.position.map(|position| position as u32).unwrap_or(0))
            .push_opt(item.kind.map(|kind| kind.to_str()))
            .push_opt(range.as_ref())
            .push_opt(item.description.as_ref())
            .push_opt(item.detail.as_ref())
            .push_opt(item.documentation.as_ref())
            .push_opt(item.insert_text.as_ref())
            .push_opt(item.filter_text.as_ref())
            .push_opt(item.command);
    }
}