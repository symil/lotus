use parsable::DataLocation;

use crate::program::CursorLocation;
use super::CodeAction;

pub struct CodeActionsProvider {
    pub cursor: Option<CursorLocation>,
    pub available_actions_under_cursor: Vec<CodeAction>
}

impl CodeActionsProvider {
    pub fn new(cursor: &Option<CursorLocation>) -> Self {
        Self {
            cursor: cursor.clone(),
            available_actions_under_cursor: vec![],
        }
    }

    pub fn add_insert(&mut self, location: Option<&DataLocation>, text: String) {

    }
}