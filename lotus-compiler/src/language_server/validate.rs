use indexmap::IndexMap;
use crate::{program::ProgramContext, command_line::CommandLineOptions};

pub fn validate(context: &mut ProgramContext, options: &CommandLineOptions) -> Vec<String> {
    let mut result = vec![];
    let mut file_errors = IndexMap::new();

    for source_file in &context.source_file_list {
        let source_file_path = source_file.path.to_str().unwrap();

        file_errors.insert(source_file_path, vec![]);
    }

    for error in context.errors.consume() {
        file_errors.get_mut(&error.location.file_path).unwrap().push(error);
    }

    for (file_path, errors) in file_errors {
        result.push(format!("file;{}", file_path.to_string()));
        
        for error in errors {
            if let Some(message) = error.get_message() {
                result.push(format!("error;{};{};{}", error.location.start, error.location.end, message));
            }
        }
    }

    result
}