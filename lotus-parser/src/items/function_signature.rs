use std::collections::HashSet;

use parsable::parsable;
use crate::program::{ProgramContext, Type};
use super::{FullType, FunctionArgument, Identifier};

#[parsable]
pub struct FunctionSignature {
    #[parsable(brackets="()", separator=",")]
    pub arguments: Vec<FunctionArgument>,
    #[parsable(prefix=":")]
    pub return_type: Option<FullType>,
}

impl FunctionSignature {
    pub fn process(&self, context: &mut ProgramContext) -> (Vec<(Identifier, Type)>, Type) {
        let mut arg_names = HashSet::new();
        let mut arguments = vec![];
        let mut return_type = Type::Void;

        for argument in &self.arguments {
            let arg_name = argument.name.clone();

            if !arg_names.insert(arg_name.clone()) {
                context.error(&arg_name, format!("duplicate argument: {}", &arg_name));
            }

            if let Some(arg_type) = Type::from_parsed_type(&argument.ty, context) {
                arguments.push((arg_name, arg_type));
            } else {
                arguments.push((arg_name, Type::Void));
            }
        }

        if let Some(ret) = &self.return_type {
            if let Some(ret_type) = Type::from_parsed_type(ret, context) {
                return_type = ret_type;
            }
        }

        (arguments, return_type)
    }
}