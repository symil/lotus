use crate::program::CursorInfo;
use super::CodeAction;

pub struct CodeActionsProvider {
    pub cursor: Option<CursorInfo>,
    pub available_actions_under_cursor: Vec<CodeAction>
}