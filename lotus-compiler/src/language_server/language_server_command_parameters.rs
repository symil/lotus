use parsable::DataLocation;
use crate::program::{ProgramContext, SharedIdentifier};

pub struct LanguageServerCommandParameters {
    pub root_directory_path: String,
    pub file_path: String,
    pub cursor_index: Option<usize>,
    pub payload: Option<String>,
}

impl LanguageServerCommandParameters {
    pub fn get_shared_identifier_under_cursor<'a>(&self, context: &'a ProgramContext) -> Option<(&'a SharedIdentifier, &'a DataLocation)> {
        if let Some(cursor_index) = self.cursor_index {
            return context.get_identifier_under_cursor(&self.file_path, cursor_index);
        }

        None
    }
}