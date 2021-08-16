use std::collections::HashMap;
use crate::items::{Identifier, StructQualifier, VisibilityToken};
use super::{FunctionAnnotation, Id, ItemMetadata, StructInfo, Type, WithMetadata};

#[derive(Debug)]
pub struct StructAnnotation {
    pub metadata: ItemMetadata,
    pub qualifier: StructQualifier,
    pub name: Identifier,
    pub parent_name: Option<Identifier>,
    pub types: Vec<Id>,
    pub self_fields: HashMap<Identifier, FieldDetails>,
    pub fields: HashMap<Identifier, FieldDetails>,
    pub user_methods: HashMap<Identifier, FunctionAnnotation>,
    pub builtin_methods: HashMap<Identifier, FunctionAnnotation>,
    pub hook_event_callbacks: HashMap<Identifier, Vec<FunctionAnnotation>>,
    pub before_event_callbacks: HashMap<Identifier, Vec<FunctionAnnotation>>,
    pub after_event_callbacks: HashMap<Identifier, Vec<FunctionAnnotation>>,
}

#[derive(Debug, Clone)]
pub struct FieldDetails {
    pub name: Identifier,
    pub ty: Type,
    pub offset: usize,
}

impl WithMetadata for StructAnnotation {
    fn get_metadata(&self) -> &ItemMetadata {
        &self.metadata
    }
}

impl StructAnnotation {
    pub fn get_struct_info(&self) -> StructInfo {
        StructInfo::new(self.metadata.id, self.name.to_string())
    }

    pub fn get_id(&self) -> &Id {
        &self.metadata.id
    }
}