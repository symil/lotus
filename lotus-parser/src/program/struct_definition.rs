use std::collections::HashMap;

use crate::items::{identifier::Identifier, struct_declaration::{StructQualifier, TypeSuffix}};

pub struct StructDefinition {
    pub name: Identifier,
    pub qualifier: StructQualifier,
    pub types: Vec<Identifier>,
    pub fields: HashMap<Identifier, FieldDetails>,
}

impl StructDefinition {
    pub fn add_field(&mut self, name: &Identifier, type_name: &Identifier, kind: FieldKind) {
        let primitive_type = match type_name.value.as_str() {
            "num" => FieldType::Numerical,
            "bool" => FieldType::Boolean,
            _ => FieldType::Entity
        };

        let offset = self.fields.values().filter(|field| field.primitive_type == primitive_type).count();

        self.fields.insert(name.clone(), FieldDetails {
            name: name.clone(),
            type_name: type_name.clone(),
            primitive_type,
            kind,
            offset
        });
    }
}

#[derive(Debug, Clone)]
pub struct FieldDetails {
    pub name: Identifier,
    pub type_name: Identifier,
    pub primitive_type: FieldType,
    pub kind: FieldKind,
    pub offset: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FieldKind {
    Single,
    Array
}

impl FieldKind {
    pub fn from_suffix(suffix: &Option<TypeSuffix>) -> Self {
        match suffix {
            Some(_) => FieldKind::Array,
            None => FieldKind::Single,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FieldType {
    Numerical,
    Boolean,
    Entity
}