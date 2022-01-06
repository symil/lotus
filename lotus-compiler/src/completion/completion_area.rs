use parsable::DataLocation;
use crate::program::{Type, FieldKind};
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
        let mut items = vec![];

        match &self.details {
            CompletionDetails::Field(parent_type) => {
                for field_info in parent_type.get_all_fields() {
                    items.push(
                        CompletionItem::new(field_info.name.as_str())
                            .kind(CompletionItemKind::Field)
                            .description(field_info.ty.to_string())
                    );
                }

                for method_info in parent_type.get_all_methods(FieldKind::Regular) {
                    if !method_info.borrow().name.as_str().starts_with("__") {
                        items.push(
                            CompletionItem::new(method_info.borrow().name.as_str())
                                .kind(CompletionItemKind::Method)
                                .description(method_info.borrow().get_self_type().to_string())
                        );
                    }
                }
            },
        }

        items
    }
}