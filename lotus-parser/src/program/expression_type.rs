use crate::items::{identifier::Identifier, struct_declaration::{Type, TypeSuffix}};

#[derive(Clone, Debug)]
pub struct ExpressionType {
    pub type_name: Identifier,
    pub type_kind: TypeKind,
    pub mutability: Mutability
}

impl ExpressionType {
    pub fn void() -> Self {
        Self {
            type_name: Identifier::default(),
            type_kind: TypeKind::Single,
            mutability: Mutability::Mutable
        }
    }

    pub fn from_type(type_: &Type) -> Self {
        Self {
            type_name: type_.name.clone(),
            type_kind: TypeKind::from_suffix(&type_.suffix),
            mutability: Mutability::Mutable
        }
    }

    pub fn is_index(&self) -> bool {
        self.type_name.is("num") && self.type_kind == TypeKind::Single
    }

    pub fn is_void(&self) -> bool {
        self.type_name.is("")
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