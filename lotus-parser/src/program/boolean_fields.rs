use crate::{generation::Wat, items::{Expression, Identifier}};
use super::{ProgramContext, TypeOld, IrFragment};

pub fn process_boolean_field_access(field_name: &Identifier, context: &mut ProgramContext) -> Option<IrFragment> {
    match field_name {
        _ => None
    }
}

pub fn process_boolean_method_call(method_name: &Identifier, context: &mut ProgramContext) -> Option<IrFragment> {
    match method_name {
        _ => None
    }
}