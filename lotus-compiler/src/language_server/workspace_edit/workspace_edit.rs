use super::TextEdit;

pub struct WorkspaceEdit {
    pub inserts: Vec<TextEdit>
}

impl WorkspaceEdit {
    pub fn new() -> Self {
        Self {
            inserts: vec![],
        }
    }
}