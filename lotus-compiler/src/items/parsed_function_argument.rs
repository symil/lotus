use parsable::parsable;
use crate::program::{ProgramContext, Type};
use super::{Identifier, ParsedType};

#[parsable]
pub struct ParsedFunctionArgument {
    pub name: Identifier,
    #[parsable(prefix=":")]
    pub ty: Option<ParsedType>,
}

impl ParsedFunctionArgument {
    pub fn process(&self, context: &mut ProgramContext) -> Option<(Identifier, Type)> {
        match &self.ty {
            Some(parsed_type) => match parsed_type.process(false, context) {
                Some(ty) => Some((self.name.clone(), ty)),
                None => None
            },
            None => {
                context.errors.expected_identifier(self);
                None
            }
        }
    }
}