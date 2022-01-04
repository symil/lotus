use crate::{program::{ProgramContext, EVENT_VAR_NAME, SELF_VAR_NAME, SELF_TYPE_NAME}, command_line::CommandLineOptions};
use super::{LanguageServerCommandParameters, LanguageServerCommandOutput};

pub fn prepare_rename(parameters: &LanguageServerCommandParameters, context: &ProgramContext, output: &mut LanguageServerCommandOutput) {
    if let Some(cursor_index) = parameters.cursor_index {
        if let Some((shared_identifier, location)) = context.get_identifier_under_cursor(&parameters.file_path, cursor_index) {
            if !is_special_identifier(location.as_str()) {
                if let Some(definition) = &shared_identifier.definition {
                    if definition.package_root_path.as_str() == &parameters.root_directory_path {
                        output
                            .line("placeholder")
                            .push(location.start)
                            .push(location.end);
                    }
                }
            }
        }
    }
}

fn is_special_identifier(name: &str) -> bool {
    match name {
        EVENT_VAR_NAME => true,
        SELF_VAR_NAME => true,
        SELF_TYPE_NAME => true,
        _ => false
    }
}