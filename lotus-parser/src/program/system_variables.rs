use crate::{generation::Wat, items::Identifier};

use super::{BuiltinType, ProgramContext, Type, Wasm};

pub fn process_system_variable(name: &Identifier, context: &mut ProgramContext) -> Option<Wasm> {
    match name.as_str() {
        "alloc" => Some(Wasm::typed(
            Type::function(vec![Type::int()], Type::pointer()),
            context.wasm.memory.alloc()
        )),
        "free" => Some(Wasm::typed(
            Type::function(vec![Type::pointer()], Type::Void),
            context.wasm.memory.free()
        )),
        "log_ptr" => Some(Wasm::typed(
            Type::function(vec![Type::pointer()], Type::Void),
            context.wasm.std.log_i32()
        )),
        "memory" => Some(Wasm::typed(
            Type::pointer(),
            Wat::const_i32(0)
        )),
        _ => None
    }
}