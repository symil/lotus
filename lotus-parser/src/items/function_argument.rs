use parsable::parsable;
use crate::program::{ProgramContext, Type};

use super::{Identifier, FullType};

#[parsable]
pub struct FunctionArgument {
    #[parsable(suffix=":")]
    pub name: Option<Identifier>,
    pub ty: FullType,
}

impl FunctionArgument {
    pub fn process(&self, context: &mut ProgramContext) -> Option<(String, Type)> {
        let arg_name = match &self.name {
            Some(name) => name.to_string(),
            None => Identifier::unique("arg", self).to_string()
        };

        match self.ty.process(context) {
            Some(arg_type) => Some((arg_name, arg_type)),
            None => None
        }
    }
}