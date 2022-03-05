use parsable::parsable;
use crate::program::{ProgramContext, Type, ArgumentInfo};
use super::{Identifier, ParsedType, ParsedVarTypeDeclaration, ParsedDefaultValueAssignment, ParsedCommaToken, unwrap_item};

#[parsable]
pub struct ParsedFunctionArgument {
    pub name: Identifier,
    pub ty: Option<ParsedVarTypeDeclaration>,
    pub default_value: Option<ParsedDefaultValueAssignment>,
}

impl ParsedFunctionArgument {
    pub fn process(&self, context: &mut ProgramContext) -> Option<ArgumentInfo> {
        let parsed_type = unwrap_item(&self.ty, &self.name, context)?;
        let ty = parsed_type.process(context)?;

        Some(ArgumentInfo {
            name: self.name.clone(),
            ty,
            is_optional: self.default_value.is_some(),
            default_value: context.vasm(),
        })
    }
}