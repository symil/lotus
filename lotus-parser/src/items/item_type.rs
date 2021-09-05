use parsable::parsable;
use crate::program::{ProgramContext, Type};
use super::{FunctionType, ValueType};

#[parsable]
pub enum ItemType {
    Value(ValueType),
    Function(FunctionType)
}

impl ItemType {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Type> {
        match self {
            ItemType::Value(value_type) => value_type.process(context),
            ItemType::Function(_) => todo!(),
        }
    }
}