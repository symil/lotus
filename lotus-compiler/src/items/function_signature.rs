use std::collections::HashSet;
use parsable::parsable;
use crate::program::{ProgramContext, Type};
use super::{ParsedType, FunctionArgument, Identifier};

#[parsable]
pub struct FunctionSignature {
    #[parsable(brackets="()", separator=",")]
    pub arguments: Vec<FunctionArgument>,
    #[parsable(prefix="->")]
    pub return_type: Option<ParsedType>,
}

impl FunctionSignature {
    pub fn process(&self, context: &mut ProgramContext) -> (Vec<(Identifier, Type)>, Option<Type>) {
        let mut arg_names = HashSet::new();
        let mut arguments = vec![];
        let mut return_type = None;

        for argument in &self.arguments {
            if let Some((arg_name, arg_type)) = argument.process(context) {
                if !arg_names.insert(arg_name.clone()) {
                    dbg!(arg_name.as_str());
                    println!("{}", &arg_type);
                    context.errors.add(&argument, format!("duplicate argument: {}", &arg_name));
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