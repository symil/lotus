use std::collections::HashSet;
use parsable::parsable;
use crate::program::{ProgramContext, TypeOld};
use super::{FullType, FunctionArgument, Identifier};

#[parsable]
pub struct FunctionSignature {
    #[parsable(brackets="()", separator=",")]
    pub arguments: Vec<FunctionArgument>,
    #[parsable(prefix="->")]
    pub return_type: Option<FullType>,
}

impl FunctionSignature {
    pub fn process(&self, context: &mut ProgramContext) -> (Vec<(Identifier, TypeOld)>, TypeOld) {
        let mut arg_names = HashSet::new();
        let mut arguments = vec![];
        let mut return_type = TypeOld::Void;

        for argument in &self.arguments {
            let arg_name = argument.name.clone();

            if !arg_names.insert(arg_name.clone()) {
                context.errors.add(&arg_name, format!("duplicate argument: {}", &arg_name));
            }

            if let Some(arg_type) = TypeOld::from_parsed_type(&argument.ty, context) {
                arguments.push((arg_name, arg_type));
            } else {
                arguments.push((arg_name, TypeOld::Void));
            }
        }

        if let Some(ret) = &self.return_type {
            if let Some(ret_type) = TypeOld::from_parsed_type(ret, context) {
                return_type = ret_type;
            }
        }

        (arguments, return_type)
    }
}