use std::collections::HashSet;
use parsable::parsable;
use crate::program::{ProgramContext, Type, ArgumentInfo};
use super::{ParsedType, ParsedFunctionArgument, Identifier};

#[parsable]
pub struct ParsedFunctionSignature {
    #[parsable(brackets="()", separator=",")]
    pub arguments: Vec<ParsedFunctionArgument>,
    #[parsable(prefix="->")]
    pub return_type: Option<ParsedType>,
}

impl ParsedFunctionSignature {
    pub fn process(&self, context: &mut ProgramContext) -> (Vec<ArgumentInfo>, Option<Type>) {
        let mut arg_names = HashSet::new();
        let mut arguments = vec![];
        let mut return_type = None;

        for argument in &self.arguments {
            if let Some(arg_info) = argument.process(context) {
                if !arg_names.insert(arg_info.name.clone()) {
                    context.errors.generic(&argument, format!("duplicate argument: {}", &arg_info.name));
                }

                arguments.push(arg_info);
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