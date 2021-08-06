use crate::{generation::Wat, items::Identifier};
use super::{ProgramContext, Wasm};

pub fn process_boolean_field_access(field_name: &Identifier, context: &mut ProgramContext) -> Option<Wasm> {
    match field_name {
        _ => None
    }
}