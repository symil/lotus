use std::collections::HashMap;

use crate::items::{Identifier, StructQualifier};

use super::{Type, FunctionAnnotation};

#[derive(Default)]
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