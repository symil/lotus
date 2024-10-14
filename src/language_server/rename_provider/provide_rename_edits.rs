use std::collections::HashSet;
use crate::{program::{ProgramContext}, command_line::CommandLineOptions, utils::is_valid_identifier, language_server::{LanguageServerCommandParameters, LanguageServerCommandOutput}};

pub fn provide_rename_edits(parameters: &LanguageServerCommandParameters, context: &ProgramContext, output: &mut LanguageServerCommandOutput) {
    if is_valid_identifier(&parameters.new_name) {
        if let Some((shared_name, _)) = context.rename_provider.get_shared_name() {
            for occurence in &shared_name.occurences {
                output
                    .line("replace")
                    .push(&occurence.file.path)
                    .push(occurence.start)
                    .push(occurence.end)
                    .push(&parameters.new_name);
            }
        }
    }
}
