use std::collections::HashMap;

use crate::items::{Identifier, StructQualifier};

use super::{Type, FunctionAnnotation};

#[derive(Clone, Default)]
pub struct StructAnnotation {
    pub name: Identifier,
    pub parent_name: Option<Identifier>,
    pub qualifier: StructQualifier,
    pub type_id: usize,
    pub types: Vec<Identifier>,
    pub self_fields: HashMap<Identifier, FieldDetails>,
    pub fields: HashMap<Identifier, FieldDetails>,
    pub user_methods: HashMap<Identifier, FunctionAnnotation>,
    pub builtin_methods: HashMap<Identifier, FunctionAnnotation>,
    pub hook_event_callbacks: HashMap<Identifier, FunctionAnnotation>,
    pub pre_event_callbacks: HashMap<Identifier, FunctionAnnotation>,
    pub post_event_callbacks: HashMap<Identifier, FunctionAnnotation>,
}

#[derive(Debug, Clone)]
pub struct FieldDetails {
    pub name: Identifier,
    pub ty: Type,
    pub offset: usize,
}