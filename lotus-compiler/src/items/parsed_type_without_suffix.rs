use parsable::parsable;
use crate::program::{ProgramContext, Type};
use super::{Identifier, ParsedTypeSingle, ParsedTypeTuple};

#[parsable]
pub enum ParsedTypeWithoutSuffix {
    Single(ParsedTypeSingle),
    Tuple(ParsedTypeTuple)
}

impl ParsedTypeWithoutSuffix {
    pub fn as_single_identifier(&self) -> Option<&Identifier> {
        match self {
            ParsedTypeWithoutSuffix::Single(single) => single.as_single_identifier(),
            ParsedTypeWithoutSuffix::Tuple(tuple) => tuple.as_single_identifier(),
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

impl Default for ParsedTypeWithoutSuffix {
    fn default() -> Self {
        Self::Single(ParsedTypeSingle::default())
    }
}