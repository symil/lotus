use std::collections::HashSet;
use crate::{program::{ProgramContext}, command_line::CommandLineOptions, utils::is_valid_identifier, language_server::{LanguageServerCommandParameters, LanguageServerCommandOutput}};

pub fn provide_rename_edits(parameters: &LanguageServerCommandParameters, context: &ProgramContext, output: &mut LanguageServerCommandOutput) {
    let root_directory_path = &parameters.root_directory_path;
    let file_path = &parameters.file_path;
    let cursor_index = parameters.cursor_index;
    let new_name = &parameters.new_name;

    if let Some(occurences) = context.rename_provider.get_all_occurences(root_directory_path, file_path, cursor_index) {
        for occurence in occurences {
            output
                .line("replace")
                .push(&occurence.file.path)
                .push(occurence.start)
                .push(occurence.end)
                .push(new_name);
        }
    }
}
