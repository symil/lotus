use crate::{generation::Wat, items::VisibilityToken};
use super::{ItemMetadata, Type, VariableInfo, WithMetadata};

#[derive(Debug)]
pub struct GlobalAnnotation {
    pub metadata: ItemMetadata,
    pub var_info: VariableInfo,
    pub value: Vec<Wat>
}

impl WithMetadata for GlobalAnnotation {
    fn get_metadata(&self) -> &ItemMetadata {
        &self.metadata
    }
}