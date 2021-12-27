use crate::{program::ProgramContext, command_line::CommandLineOptions};

pub fn prepare_rename(context: &mut ProgramContext, options: &CommandLineOptions) -> Vec<String> {
    let mut result = vec![];

    if let Some(cursor) = options.cursor {
        if let Some((shared_identifier, location)) = context.get_identifier_under_cursor(&options.input_path, cursor) {
            if shared_identifier.definition.package_root_path == options.get_root_directory_path().as_str() {
                result.push(format!("placeholder;{};{}", location.start, location.end));
            }
        }
    }

    result
}