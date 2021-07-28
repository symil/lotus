use std::collections::HashMap;

use crate::items::{identifier::Identifier, struct_declaration::{StructQualifier}};

use super::{expression_type::{ExpressionType}, function_annotation::FunctionAnnotation};

pub struct StructAnnotation {
    pub name: Identifier,
    pub qualifier: StructQualifier,
    pub types: Vec<Identifier>,
    pub fields: HashMap<Identifier, FieldDetails>,
    pub methods: HashMap<Identifier, FunctionAnnotation>
}

impl StructAnnotation {
    pub fn new(name: &Identifier, qualifier: &StructQualifier) -> Self {
        Self {
            name: name.clone(),
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
    pub expr_type: ExpressionType,
    pub offset: usize,
}

impl FieldDetails {
    pub fn get_expr_type(&self) -> ExpressionType {
        self.expr_type.clone()
    }
}