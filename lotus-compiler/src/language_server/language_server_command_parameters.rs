use parsable::DataLocation;
use crate::program::{ProgramContext, SharedIdentifier};

pub struct LanguageServerCommandParameters {
    pub root_directory_path: String,
    pub file_path: String,
    pub cursor_index: usize,
    pub payload: Option<String>,
}