use parsable::parsable;
use crate::program::{BuiltinType, ProgramContext, Type};
use super::Identifier;

#[parsable]
pub struct ParsedTypeSuffix {
    pub token: ParsedTypeSuffixToken
}

#[parsable]
#[derive(PartialEq, Clone, Copy)]
pub enum ParsedTypeSuffixToken {
    Array = "[]",
    Option = "?"
}

impl ParsedTypeSuffix {
    pub fn collected_instancied_type_names(&self, list: &mut Vec<Identifier>) {
        let builtin_type = match &self.token {
            ParsedTypeSuffixToken::Array => Some(BuiltinType::Array),
            ParsedTypeSuffixToken::Option => None
        };

        if let Some(builtin_type) = builtin_type {
            list.push(Identifier::unlocated(builtin_type.get_name()));
        }
    }

    pub fn process(&self, current_type: Type, context: &mut ProgramContext) -> Type {
        match &self.token {
            ParsedTypeSuffixToken::Array => context.get_builtin_type(BuiltinType::Array, vec![current_type]),
            ParsedTypeSuffixToken::Option => current_type
        }
    }
}