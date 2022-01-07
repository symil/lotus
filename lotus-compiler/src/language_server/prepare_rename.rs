use crate::{program::{ProgramContext, EVENT_VAR_NAME, SELF_VAR_NAME, SELF_TYPE_NAME}, command_line::CommandLineOptions};
use super::{LanguageServerCommandParameters, LanguageServerCommandOutput};

pub fn prepare_rename(parameters: &LanguageServerCommandParameters, context: &ProgramContext, output: &mut LanguageServerCommandOutput) {
    let root_directory_path = &parameters.root_directory_path;
    let file_path = &parameters.file_path;
    let cursor_index = parameters.cursor_index;

    if let Some(location) = context.renaming.get_occurence_under_cursor(root_directory_path, file_path, cursor_index) {
        output
            .line("placeholder")
            .push(location.start)
            .push(location.end);
    }
}