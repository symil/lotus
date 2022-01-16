use std::collections::HashSet;
use parsable::parsable;
use crate::program::{ProgramContext, Type};
use super::{ParsedType, ParsedFunctionArgument, Identifier};

#[parsable]
pub struct ParsedFunctionSignature {
    #[parsable(brackets="()", separator=",")]
    pub arguments: Vec<ParsedFunctionArgument>,
    #[parsable(prefix="->")]
    pub return_type: Option<ParsedType>,
}

impl ParsedFunctionSignature {
    pub fn process(&self, context: &mut ProgramContext) -> (Vec<(Identifier, Type)>, Option<Type>) {
        let mut arg_names = HashSet::new();
        let mut arguments = vec![];
        let mut return_type = None;

        for argument in &self.arguments {
            if let Some((arg_name, arg_type)) = argument.process(context) {
                if !arg_names.insert(arg_name.clone()) {
                    context.errors.generic(&argument, format!("duplicate argument: {}", &arg_name));
                }

                arguments.push((arg_name, arg_type));
            }
        }

        if let Some(ret) = &self.return_type {
            if let Some(ret_type) = ret.process(false, context) {
                return_type = Some(ret_type);
            }
        }

        (arguments, return_type)
    }
}