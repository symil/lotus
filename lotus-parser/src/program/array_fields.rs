use crate::{generation::Wat, items::Identifier};

use super::{ProgramContext, Type, Wasm};

pub fn process_array_method_call(item_type: &Type, method_name: &Identifier, context: &mut ProgramContext) -> Option<Wasm> {
    match method_name.as_str() {
        "len" => Some(Wasm::typed(
            Type::function(vec![], Type::int()),
            context.wasm.std.array_length()
        )),
        "get" => Some(Wasm::typed(
            Type::function(vec![Type::int()], item_type.clone()),
            context.wasm.std.array_get()
        )),
        _ => None
    }
}

pub fn process_array_field_access(item_type: &Type, field_name: &Identifier, context: &mut ProgramContext) -> Option<Wasm> {
    match field_name.as_str() {
        _ => None
    }
}