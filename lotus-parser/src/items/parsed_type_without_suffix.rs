use parsable::parsable;
use crate::program::{ProgramContext, Type};
use super::{Identifier, ParsedTypeSingle, ParsedTypeTuple};

#[parsable]
pub enum ParsedTypeWithoutSuffix {
    Single(ParsedTypeSingle),
    Tuple(ParsedTypeTuple)
}

impl ParsedTypeWithoutSuffix {
    pub fn as_var_name(&self) -> Option<&Identifier> {
        match self {
            ParsedTypeWithoutSuffix::Single(single) => single.as_var_name(),
            ParsedTypeWithoutSuffix::Tuple(tuple) => tuple.as_var_name(),
        }
    }

    pub fn collected_instancied_type_names(&self, list: &mut Vec<Identifier>) {
        match self {
            ParsedTypeWithoutSuffix::Single(single) => single.collected_instancied_type_names(list),
            ParsedTypeWithoutSuffix::Tuple(tuple) => tuple.collected_instancied_type_names(list),
        }
    }

    pub fn process(&self, check_interfaces: bool, context: &mut ProgramContext) -> Option<Type> {
        match self {
            ParsedTypeWithoutSuffix::Single(single) => single.process(check_interfaces, context),
            ParsedTypeWithoutSuffix::Tuple(tuple) => tuple.process(check_interfaces, context),
        }
    }
}