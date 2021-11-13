use parsable::{DataLocation, parsable};
use crate::program::{ProgramContext, Type};
use super::{Macro, ParsedType};

#[parsable]
pub enum ParsedTypeWrapper {
    ParsedType(ParsedType),
    Macro(Macro)
}

impl ParsedTypeWrapper {
    pub fn process(&self, check_interfaces: bool, context: &mut ProgramContext) -> Option<Type> {
        match self {
            ParsedTypeWrapper::ParsedType(parsed_type) => parsed_type.process(check_interfaces, context),
            ParsedTypeWrapper::Macro(mac) => mac.process_as_type(context),
        }
    }

    pub fn get_location(&self) -> &DataLocation {
        match self {
            ParsedTypeWrapper::ParsedType(parsed_type) => &parsed_type.location,
            ParsedTypeWrapper::Macro(mac) => &mac.location,
        }
    }
}