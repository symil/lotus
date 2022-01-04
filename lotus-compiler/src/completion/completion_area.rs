use parsable::DataLocation;
use crate::program::Type;

use super::{CompletionItem, CompletionItemKind};

#[derive(Debug)]
pub struct CompletionArea {
    pub location: DataLocation,
    pub details: CompletionDetails
}

#[derive(Debug)]
pub enum CompletionDetails {
    Field(Type)
}

impl CompletionArea {
    pub fn contains_cursor(&self, cursor_index: usize) -> bool {
        self.location.start <= cursor_index && self.location.end >= cursor_index
    }

    pub fn provide_completion_items(&self) -> Vec<CompletionItem> {
        match &self.details {
            CompletionDetails::Field(parent_type) => {
                parent_type.get_all_fields().into_iter().map(|field_info| CompletionItem{
                    name: field_info.name.to_string(),
                    kind: CompletionItemKind::Field
                }).collect()
            },
        }
    }
}