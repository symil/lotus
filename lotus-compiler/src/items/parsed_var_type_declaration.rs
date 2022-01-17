use parsable::parsable;
use crate::program::{ProgramContext, Type};
use super::{ParsedColon, ParsedType, unwrap_item};

#[parsable]
pub struct ParsedVarTypeDeclaration {
    pub colon: ParsedColon,
    pub ty: Option<ParsedType>
}

impl ParsedVarTypeDeclaration {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Type> {
        let ty = unwrap_item(&self.ty, self, context)?;

        ty.process(context.get_current_function().is_some(), context)
    }
}