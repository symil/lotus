use std::borrow::Borrow;
use parsable::DataLocation;
use crate::language_server::{WorkspaceEdit, TextEdit};
use super::CodeActionKind;

pub struct CodeAction {
    pub title: String,
    pub kind: CodeActionKind,
    pub edit: WorkspaceEdit
}

impl CodeAction {
    pub fn new<S : ToString>(title: S) -> Self {
        Self {
            title: title.to_string(),
            kind: CodeActionKind::Empty,
            edit: WorkspaceEdit::new(),
        }
    }

    pub fn add_edit<L : Borrow<DataLocation>, T : ToString>(&mut self, deleted_location: L, inserted_text: T) {
        self.edit.inserts.push(TextEdit::new(deleted_location.borrow().clone(), inserted_text.to_string()))
    }
}
