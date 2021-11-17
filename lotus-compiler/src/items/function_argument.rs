use parsable::parsable;
use crate::program::{ProgramContext, Type};
use super::{Identifier, ParsedType};

#[parsable]
pub struct FunctionArgument {
    #[parsable(suffix=":")]
    pub name: Option<Identifier>,
    pub ty: ParsedType,
}

impl FunctionArgument {
    pub fn process(&self, context: &mut ProgramContext) -> Option<(Identifier, Type)> {
        let arg_name = self.name.clone().unwrap_or_else(|| Identifier::unique("arg", self));

        match self.ty.process(false, context) {
            Some(arg_type) => Some((arg_name, arg_type)),
            None => None
        }
    }
}