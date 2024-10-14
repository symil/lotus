use super::TextEdit;

pub struct WorkspaceEdit {
    pub text_edits: Vec<TextEdit>
}

impl WorkspaceEdit {
    pub fn new() -> Self {
        Self {
            text_edits: vec![],
        }
    }
}