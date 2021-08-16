use crate::{generation::Wat, items::VisibilityToken};
use super::{ItemMetadata, Type, WithMetadata};

#[derive(Debug)]
pub struct GlobalAnnotation {
    pub metadata: ItemMetadata,
    pub wasm_name: String,
    pub ty: Type,
    pub value: Vec<Wat>
}

impl WithMetadata for GlobalAnnotation {
    fn get_metadata(&self) -> &ItemMetadata {
        &self.metadata
    }
}