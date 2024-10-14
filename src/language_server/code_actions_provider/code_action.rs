use std::borrow::Borrow;
use parsable::ItemLocation;
use crate::language_server::{WorkspaceEdit, TextEdit};
use super::CodeActionKind;

pub struct CodeAction {
    pub title: String,
    pub kind: CodeActionKind,
    pub workspace_edit: WorkspaceEdit
}

impl CodeAction {
    pub fn new<S : ToString>(title: S, kind: CodeActionKind) -> Self {
        Self {
            title: title.to_string(),
            kind,
            workspace_edit: WorkspaceEdit::new(),
        }
    }

    pub fn add_text_edit(&mut self, edit: TextEdit) {
        self.workspace_edit.text_edits.push(edit);
    }
}
