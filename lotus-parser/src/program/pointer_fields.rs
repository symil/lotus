use crate::{generation::Wat, items::{Expression, Identifier}};
use super::{ProgramContext, TypeOld, IrFragment};

pub fn process_pointer_field_access(pointed_type: &TypeOld, field_name: &Identifier, context: &mut ProgramContext) -> Option<IrFragment> {
    match field_name {
        _ => None
    }
}

pub fn process_pointer_method_call(pointed_type: &TypeOld, method_name: &Identifier, context: &mut ProgramContext) -> Option<IrFragment> {
    match method_name {
        _ => None
    }
}