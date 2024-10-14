use crate::{program::{ProgramContext, EVENT_VAR_NAME, SELF_VAR_NAME, SELF_TYPE_NAME}, command_line::CommandLineOptions, language_server::{LanguageServerCommandOutput, LanguageServerCommandParameters}};

pub fn prepare_rename(parameters: &LanguageServerCommandParameters, context: &ProgramContext, output: &mut LanguageServerCommandOutput) {
    if let Some((_, occurence)) = context.rename_provider.get_shared_name() {
        output
            .line("placeholder")
            .push(occurence.start)
            .push(occurence.end);
    }
}