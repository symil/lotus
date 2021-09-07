use crate::items::FullType;
use super::ProgramContext;

#[derive(Debug, Clone)]
pub enum Type {
    Void,
    Generic(GenericInfo),
    Actual(TypeRef),
    TypeRef(TypeRef)
}

#[derive(Debug, Clone)]
pub struct TypeRef {
    pub type_id: u64, // blueprint id
    pub type_context: Option<u64>, // blueprint id
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
}

impl Default for Type {
    fn default() -> Self {
        Self::Void
    }
}