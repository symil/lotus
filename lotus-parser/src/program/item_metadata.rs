use crate::items::{Identifier, VisibilityToken};
use super::Id;

#[derive(Debug)]
pub struct ItemMetadata {
    pub id: Id,
    pub name: Identifier,
    pub file_name: String,
    pub namespace: String,
    pub visibility: VisibilityToken,
}

impl ItemMetadata {
    pub fn to_unique_name(&self) -> String {
        format!("{}_{}", self.name, self.id)
    }
}