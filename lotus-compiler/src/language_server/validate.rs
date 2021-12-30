use indexmap::IndexMap;
use parsable::StringReader;
use crate::{program::ProgramContext, command_line::CommandLineOptions};
use super::LanguageServerCommandParameters;

pub fn validate(parameters: &LanguageServerCommandParameters, context: &ProgramContext, lines: &mut Vec<String>) {
    let mut file_errors = IndexMap::new();

    for source_file in &context.source_file_list {
        let source_file_path = source_file.path.to_str().unwrap();

        file_errors.insert(source_file_path, vec![]);
    }

    for error in context.errors.get_all() {
        file_errors.get_mut(&error.location.file_path).unwrap().push(error);
    }

    for (file_path, errors) in file_errors {
        lines.push(format!("file;{}", file_path.to_string()));
        
        for error in errors {
            if let Some(message) = error.get_message() {
                lines.push(format!("error;{};{};{}", error.location.start, error.location.end, message));
            }
        }
    }
}