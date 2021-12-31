use parsable::DataLocation;

use crate::program::{ProgramContext, SharedIdentifier};

pub struct LanguageServerCommandParameters {
    pub root_directory_path: Option<String>,
    pub file_path: Option<String>,
    pub cursor_index: Option<usize>,
    pub new_name: Option<String>
}

impl LanguageServerCommandParameters {
    pub fn get_shared_identifier_under_cursor<'a>(&self, context: &'a ProgramContext) -> Option<(&'a SharedIdentifier, &'a DataLocation)> {
        if let (Some(file_path), Some(cursor_index)) = (&self.file_path, self.cursor_index) {
            return context.get_identifier_under_cursor(file_path, cursor_index);
        }

        None
    }
}