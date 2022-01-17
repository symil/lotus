use parsable::parsable;
use crate::program::{ProgramContext, Type};
use super::{Identifier, ParsedTypeSingle, ParsedTypeTuple, ParsedMacroType};

#[parsable]
pub enum ParsedTypeWithoutSuffix {
    Macro(ParsedMacroType),
    Single(ParsedTypeSingle),
    Tuple(ParsedTypeTuple)
}

impl ParsedTypeWithoutSuffix {
    pub fn as_single_identifier(&self) -> Option<&Identifier> {
        match self {
            ParsedTypeWithoutSuffix::Macro(_) => None,
            ParsedTypeWithoutSuffix::Single(single) => single.as_single_identifier(),
            ParsedTypeWithoutSuffix::Tuple(tuple) => tuple.as_single_identifier(),
        }
    }

    pub fn collecte_instancied_type_names(&self, list: &mut Vec<String>, context: &mut ProgramContext) {
        match self {
            ParsedTypeWithoutSuffix::Macro(mac) => list.push(mac.process_as_name(context).unwrap().to_string()),
            ParsedTypeWithoutSuffix::Single(single) => single.collecte_instancied_type_names(list, context),
            ParsedTypeWithoutSuffix::Tuple(tuple) => tuple.collecte_instancied_type_names(list, context),
        }
    }

    pub fn process(&self, check_interfaces: bool, context: &mut ProgramContext) -> Option<Type> {
        match self {
            ParsedTypeWithoutSuffix::Macro(mac) => mac.process(context),
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