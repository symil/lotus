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
    pub fn collect_instancied_type_names(&self, list: &mut Vec<String>) {
        let builtin_type = match &self.token {
            ParsedTypeSuffixToken::Array => Some(BuiltinType::Array),
            ParsedTypeSuffixToken::Option => None
        };

        if let Some(builtin_type) = builtin_type {
            list.push(builtin_type.get_name().to_string());
        }
    }

    pub fn process(&self, current_type: Type, context: &mut ProgramContext) -> Type {
        match &self.token {
            ParsedTypeSuffixToken::Array => context.get_builtin_type(BuiltinType::Array, vec![current_type]),
            ParsedTypeSuffixToken::Option => current_type
        }
    }
}