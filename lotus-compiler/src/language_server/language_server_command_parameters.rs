pub struct LanguageServerCommandParameters {
    pub root_directory_path: Option<String>,
    pub file_path: Option<String>,
    pub cursor_index: Option<usize>,
    pub new_name: Option<String>
}