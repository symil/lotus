use crate::{language_server::{LanguageServerCommandOutput, LanguageServerCommandParameters}, program::ProgramContext};

pub fn provide_code_actions(parameters: &LanguageServerCommandParameters, context: &ProgramContext, output: &mut LanguageServerCommandOutput) {
    for code_action in context.code_actions_provider.get_code_actions() {
        output.line("action")
            .push(&code_action.title)
            .push(code_action.kind);

        for text_edit in &code_action.workspace_edit.text_edits {
            output.line("replace")
                .push(&text_edit.edit_location.file.path)
                .push(text_edit.edit_location.start)
                .push(text_edit.edit_location.end)
                .push(&text_edit.replacement_text);
        }
    }
}