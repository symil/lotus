use std::fmt::Display;

use crate::items::FullType;
use super::ProgramContext;

#[derive(Debug, Clone)]
pub enum Type {
    Void,
    Any,
    Generic(GenericInfo),
    Actual(TypeRef),
    TypeRef(TypeRef)
}

#[derive(Debug, Clone)]
pub struct TypeRef {
    pub name: String, // only used for display
    pub type_id: u64, // blueprint id
    pub generic_values: Vec<Type>
}

#[derive(Debug, Clone)]
pub struct GenericInfo {
    pub name: String,
    pub type_context: u64
}

impl Type {
    pub fn generic(name: String, type_context: u64) -> Type {
        Type::Generic(GenericInfo {
            name,
            type_context,
        })
    }

    pub fn is_void(&self) -> bool {
        match self {
            Type::Void => true,
            _ => false
        }
    }

    pub fn is_any(&self) -> bool {
        match self {
            Type::Any => true,
            _ => false
        }
    }

    pub fn is_generic(&self) -> bool {
        match self {
            Type::Generic(_) => true,
            _ => false
        }
    }

    pub fn is_actual(&self) -> bool {
        match self {
            Type::Actual(_) => true,
            _ => false
        }
    }

    pub fn get_wasm_type(&self, context: &ProgramContext) -> Option<String> {
        match self {
            Type::Void => None,
            Type::Generic(info) => Some(format!("?{}", &info.name)),
            Type::Actual(typeref) => context.types.get_by_id(typeref.type_id).unwrap().get_wasm_type().and_then(|s| Some(s.to_string())),
            _ => unreachable!()
        }
    }

    pub fn is_assignable(&self) -> bool {
        match self {
            Type::Void => true,
            Type::Any => true,
            Type::Generic(_) => true,
            Type::Actual(_) => true,
            Type::TypeRef(_) => false,
        }
    }

    pub fn is_assignable_to(&self, target: &Type, context: &ProgramContext) -> bool {
        match self {
            Type::Void => false,
            Type::Any => true,
            Type::Generic(self_generic_info) => {
                match target {
                    Type::Generic(target_generic_info) => self_generic_info == target_generic_info,
                    _ => false
                }
            },
            Type::Actual(self_typeref) => {
                match target {
                    Type::Actual(target_typeref) => {
                        let self_type = context.types.get_by_id(self_typeref.type_id).unwrap();

                        self_type.inheritance_chain.contains(target_typeref)
                    },
                    _ => false
                }
            },
            Type::TypeRef(_) => false,
        }
    }
}

impl PartialEq for TypeRef {
    fn eq(&self, other: &Self) -> bool {
        self.type_id == other.type_id && self.generic_values == other.generic_values
    }
}

impl PartialEq for GenericInfo {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.type_context == other.type_context
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Generic(l0), Self::Generic(r0)) => l0 == r0,
            (Self::Actual(l0), Self::Actual(r0)) => l0 == r0,
            (Self::TypeRef(l0), Self::TypeRef(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl Default for Type {
    fn default() -> Self {
        Self::Void
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Void => write!(f, "<void>"),
            Type::Any => write!(f, "<any>"),
            Type::Generic(info) => write!(f, "{}", &info.name),
            Type::Actual(typeref) => write!(f, "{}", &typeref.name),
            Type::TypeRef(typeref) => write!(f, "<type {}>", &typeref.name),
        }
    }
}