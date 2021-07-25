use crate::items::{identifier::Identifier, struct_declaration::TypeSuffix};

#[derive(Clone, Debug)]
pub struct ExpressionType {
    pub type_name: Identifier,
    pub type_kind: TypeKind,
    pub mutability: Mutability
}

impl ExpressionType {
    pub fn is_index(&self) -> bool {
        self.type_name.is("num") && self.type_kind == TypeKind::Single
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TypeKind {
    Single,
    Array
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Mutability {
    Constant,
    Mutable
}

impl TypeKind {
    pub fn from_suffix(suffix: &Option<TypeSuffix>) -> Self {
        match suffix {
            Some(_) => TypeKind::Array,
            None => TypeKind::Single,
        }
    }
}