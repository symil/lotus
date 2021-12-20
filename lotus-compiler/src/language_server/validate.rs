use indexmap::IndexMap;
use crate::{program::ProgramContext, command_line::CommandLineOptions};
use super::LanguegeServerLogItem;

pub fn validate_program(context: &mut ProgramContext, options: &CommandLineOptions) -> Vec<LanguegeServerLogItem> {
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
        result.push(LanguegeServerLogItem::File(file_path.to_string()));
        
        for error in errors {
            result.push(LanguegeServerLogItem::Error(error));
        }
    }

    result
}