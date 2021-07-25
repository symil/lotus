use std::collections::HashMap;

use crate::items::{identifier::Identifier, struct_declaration::{StructQualifier}};

use super::expression_type::{ExpressionType, Mutability, TypeKind};

pub struct StructDefinition {
    pub name: Identifier,
    pub qualifier: StructQualifier,
    pub types: Vec<Identifier>,
    pub fields: HashMap<Identifier, FieldDetails>,
}

impl StructDefinition {
    pub fn add_field(&mut self, name: &Identifier, type_name: &Identifier, type_kind: TypeKind) {
        let primitive_type = match type_name.value.as_str() {
            "num" => FieldType::Numerical,
            "bool" => FieldType::Boolean,
            _ => FieldType::Entity
        };

        let offset = self.fields.values().filter(|field| field.primitive_type == primitive_type).count();

        self.fields.insert(name.clone(), FieldDetails {
            name: name.clone(),
            type_name: type_name.clone(),
            type_kind,
            primitive_type,
            offset
        });
    }
}

#[derive(Debug, Clone)]
pub struct FieldDetails {
    pub name: Identifier,
    pub type_name: Identifier,
    pub type_kind: TypeKind,
    pub primitive_type: FieldType,
    pub offset: usize,
}

impl FieldDetails {
    pub fn get_expr_type(&self) -> ExpressionType {
        ExpressionType {
            type_name: self.type_name.clone(),
            type_kind: self.type_kind,
            mutability: Mutability::Mutable,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FieldType {
    Numerical,
    Boolean,
    Entity
}