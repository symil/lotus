use std::collections::HashMap;

use crate::items::{Identifier, StructQualifier};

use super::{Type, FunctionAnnotation};

#[derive(Clone)]
pub struct StructAnnotation {
    pub name: Identifier,
    pub type_id: usize,
    pub qualifier: StructQualifier,
    pub types: Vec<Identifier>,
    pub fields: HashMap<Identifier, FieldDetails>,
    pub methods: HashMap<Identifier, FunctionAnnotation>
}

impl StructAnnotation {
    pub fn new(name: &Identifier, qualifier: &StructQualifier) -> Self {
        Self {
            name: name.clone(),
            type_id: todo!(),
            qualifier: qualifier.clone(),
            types: vec![],
            fields: HashMap::new(),
            methods: HashMap::new()
        }
    }
}

#[derive(Debug, Clone)]
pub struct FieldDetails {
    pub name: Identifier,
    pub expr_type: Type,
    pub offset: usize,
}

impl FieldDetails {
    pub fn get_expr_type(&self) -> Type {
        self.expr_type.clone()
    }
}