use std::collections::HashMap;

use crate::items::{identifier::Identifier, struct_declaration::{StructQualifier, ValueType}};

use super::{expression_type::{ExpressionType}, function_definition::FunctionAnnotation};

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

    pub fn add_field(&mut self, name: &Identifier, value_type: &ValueType) {
        let primitive_type = match value_type.name.as_str() {
            "num" => FieldPrimitiveType::Numerical,
            "bool" => FieldPrimitiveType::Boolean,
            _ => FieldPrimitiveType::Entity
        };

        let offset = self.fields.values().filter(|field| field.primitive_type == primitive_type).count();

        self.fields.insert(name.clone(), FieldDetails {
            name: name.clone(),
            type_: ExpressionType::from_value_type(value_type),
            primitive_type,
            offset
        });
    }
}

#[derive(Debug, Clone)]
pub struct FieldDetails {
    pub name: Identifier,
    pub type_: ExpressionType,
    pub primitive_type: FieldPrimitiveType,
    pub offset: usize,
}

impl FieldDetails {
    pub fn get_expr_type(&self) -> ExpressionType {
        self.type_.clone()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FieldPrimitiveType {
    Numerical,
    Boolean,
    Entity
}