use parsable::parsable;

use crate::program::{BuiltinType, ProgramContext, Type};

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
    pub fn process(&self, current_type: Type, context: &mut ProgramContext) -> Type {
        match &self.value {
            TypeSuffix::Array => context.get_builtin_type(BuiltinType::Array, vec![current_type]),
            TypeSuffix::Option => context.get_builtin_type(BuiltinType::Option, vec![current_type]),
        }
    }
}