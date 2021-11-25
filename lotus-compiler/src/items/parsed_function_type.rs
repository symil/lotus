use parsable::parsable;
use crate::program::{ProgramContext, Signature, Type};
use super::ParsedType;

#[parsable]
pub struct ParsedFunctionType {
    #[parsable(regex="fn")]
    pub fn_token: String,
    #[parsable(brackets="()", separator=",")]
    pub arguments: Vec<ParsedType>,
    #[parsable(brackets="()")]
    pub return_type: Option<Box<ParsedType>>
}

impl ParsedFunctionType {
    pub fn process(&self, check_interfaces: bool, context: &mut ProgramContext) -> Option<Type> {
        let mut ok = true;
        let mut argument_types = vec![];
        let mut return_type = context.void_type();

        for parsed_type in &self.arguments {
            if let Some(ty) = parsed_type.process(check_interfaces, context) {
                argument_types.push(ty);
            } else {
                ok = false;
            }
        }

        if let Some(parsed_type) = &self.return_type {
            if let Some(ty) = parsed_type.process(check_interfaces, context) {
                return_type = ty;
            } else {
                ok = false;
            }
        }

        match ok {
            true => Some(Type::Function(Box::new(Signature {
                this_type: None,
                argument_types,
                return_type,
            }))),
            false => None,
        }
    }
}