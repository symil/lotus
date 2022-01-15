use indexmap::IndexMap;
use parsable::DataLocation;

use crate::{program::{CursorLocation, Cursor}, language_server::TextEdit};
use super::{CodeAction, CodeActionKind};

pub struct CodeActionsProvider {
    pub cursor: Cursor,
    pub available_actions_under_cursor: IndexMap<String, CodeAction>
}

impl CodeActionsProvider {
    pub fn new(cursor: &Cursor) -> Self {
        Self {
            cursor: cursor.clone(),
            available_actions_under_cursor: IndexMap::new(),
        }
    }

    pub fn add_replace_action<F : Fn() -> Option<String>>(&mut self, location: &DataLocation, title: &'static str, location_to_replace: Option<&DataLocation>, make_replacement: F) {
        if !self.cursor.is_on_location(location) {
            return;
        }

        if !self.available_actions_under_cursor.contains_key(title) {
            let mut code_action = CodeAction::new(title, CodeActionKind::QuickFix);
            let edit_location = location_to_replace.unwrap_or_else(|| self.cursor.get_location().unwrap()).get_end();

            if let Some(mut replacement_text) = make_replacement() {
                let indent = get_indentation(&location.file.content, edit_location.start);
                let separator = format!("\n{}", indent);

                replacement_text = replacement_text.split("\n")
                    .collect::<Vec<&str>>()
                    .join(&separator);

                code_action.add_text_edit(TextEdit {
                    edit_location,
                    replacement_text,
                });

                self.available_actions_under_cursor.insert(title.to_string(), code_action);
            }
        }
    }

    pub fn get_code_actions(&self) -> Vec<&CodeAction> {
        self.available_actions_under_cursor.values().collect()
    }
}

fn get_indentation(content: &str, index: usize) -> &str {
    let mut start_index = index;

    while start_index > 0 && is_space_or_tab(content.as_bytes()[start_index - 1]) {
        start_index -= 1;
    }

    &content[start_index..index]
}

fn is_space_or_tab(c: u8) -> bool {
    c == b' ' || c == b'\t'
}