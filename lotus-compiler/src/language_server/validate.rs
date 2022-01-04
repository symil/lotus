use indexmap::IndexMap;
use parsable::StringReader;
use crate::{program::ProgramContext, command_line::CommandLineOptions};
use super::{LanguageServerCommandParameters, LanguageServerCommandOutput};

pub fn validate(parameters: &LanguageServerCommandParameters, context: &ProgramContext, output: &mut LanguageServerCommandOutput) {
    let mut file_errors = IndexMap::new();

    for source_file in &context.source_file_list {
        file_errors.insert(source_file.file_path.as_str(), vec![]);
    }

    for error in context.errors.get_all() {
        file_errors.get_mut(error.location.file_path.as_str()).unwrap().push(error);
    }

    for (file_path, errors) in file_errors {
        output
            .line("file")
            .push(file_path);
        
        for error in errors {
            if let Some(message) = error.get_message() {
                output
                    .line("error")
                    .push(error.location.start)
                    .push(error.location.end)
                    .push(message);
            }
        }
    }
}