use parsable::parsable;
use crate::program::{BuiltinType, ProgramContext, Type};
use super::Identifier;

#[parsable]
pub struct  TypeSuffixWrapper {
    pub value: TypeSuffix
}

#[parsable]
#[derive(PartialEq, Clone, Copy)]
pub enum TypeSuffix {
    Array = "[]",
    Option = "?"
}

impl TypeSuffixWrapper {
    pub fn collected_instancied_type_names(&self, list: &mut Vec<Identifier>) {
        let builtin_type = match &self.value {
            TypeSuffix::Array => Some(BuiltinType::Array),
            TypeSuffix::Option => None
        };

        if let Some(builtin_type) = builtin_type {
            list.push(Identifier::unlocated(builtin_type.get_name()));
        }
    }

    pub fn process(&self, current_type: Type, context: &mut ProgramContext) -> Type {
        match &self.value {
            TypeSuffix::Array => context.get_builtin_type(BuiltinType::Array, vec![current_type]),
            TypeSuffix::Option => current_type
        }
    }
}