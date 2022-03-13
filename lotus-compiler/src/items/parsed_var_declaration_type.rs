use parsable::{parsable};
use crate::program::{ProgramContext, Type};
use super::{ParsedColonToken, ParsedType};

#[parsable(cascade = true)]
pub struct ParsedVarDeclarationType {
    pub colon: Option<ParsedColonToken>,
    pub var_type: Option<ParsedType>,
}

impl ParsedVarDeclarationType {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Type> {
        if let Some(colon) = &self.colon {
            if let Some(parsed_type) = &self.var_type {
                parsed_type.process(true, None, context)
            } else {
                context.errors.expected_type(colon);
                None
            }
        } else {
            None
        }
    }
}