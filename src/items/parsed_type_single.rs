use parsable::parsable;
use crate::program::{ProgramContext, Type};
use super::{ParsedFunctionType, Identifier, ParsedValueType};

#[parsable]
pub enum ParsedTypeSingle {
    Function(ParsedFunctionType),
    Value(ParsedValueType),
}

impl ParsedTypeSingle {
    pub fn as_single_identifier(&self) -> Option<&Identifier> {
        match self {
            ParsedTypeSingle::Value(value_type) => value_type.as_single_name(),
            ParsedTypeSingle::Function(_) => None,
        }
    }

    pub fn collect_instancied_type_names(&self, list: &mut Vec<String>, context: &mut ProgramContext) {
        match self {
            ParsedTypeSingle::Value(value_type) => value_type.collect_instancied_type_names(list),
            ParsedTypeSingle::Function(_) => {},
        }
    }

    pub fn process(&self, check_interfaces: bool, type_hint: Option<&Type>, context: &mut ProgramContext) -> Option<Type> {
        match self {
            ParsedTypeSingle::Value(value_type) => value_type.process(check_interfaces, type_hint, context),
            ParsedTypeSingle::Function(function_type) => function_type.process(check_interfaces, context),
        }
    }
}

impl Default for ParsedTypeSingle {
    fn default() -> Self {
        Self::Value(ParsedValueType::default())
    }
}