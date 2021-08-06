use crate::{generation::Wat, items::Identifier};
use super::{ProgramContext, Type, Wasm};

pub fn process_pointer_field_access(field_name: &Identifier, context: &mut ProgramContext) -> Option<Wasm> {
    match field_name {
        _ => None
    }
}