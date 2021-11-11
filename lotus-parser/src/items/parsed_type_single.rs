use parsable::parsable;
use crate::program::{ProgramContext, Type};
use super::{FunctionType, Identifier, ParsedValueType};

#[parsable]
pub enum ParsedTypeSingle {
    Value(ParsedValueType),
    // Function(FunctionType)
}

impl ParsedTypeSingle {
    pub fn as_var_name(&self) -> Option<&Identifier> {
        match self {
            ParsedTypeSingle::Value(value_type) => value_type.as_single_name(),
            // ItemType::Function(_) => None,
        }
    }

    pub fn collected_instancied_type_names(&self, list: &mut Vec<Identifier>) {
        match self {
            ParsedTypeSingle::Value(value_type) => value_type.collected_instancied_type_names(list),
            // ItemType::Function(_) => {},
        }
    }

    pub fn process(&self, check_interfaces: bool, context: &mut ProgramContext) -> Option<Type> {
        match self {
            ParsedTypeSingle::Value(value_type) => value_type.process(check_interfaces, context),
            // ItemType::Function(_) => todo!(),
        }
    }
}