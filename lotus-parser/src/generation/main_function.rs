use crate::wat;
use super::{ToWat, WasmModule, Wat, ToWatVec};

pub struct MainFunction;

impl MainFunction {
    pub fn new() -> Self {
        Self
    }

    pub fn get_functions(&self, module: &WasmModule) -> Vec<Wat> {
        vec![
            Wat::declare_function("main", Some("_start"), vec![], None, vec![
                Wat::declare_local_i32("addr"),

                module.memory.init(),
                Wat::set_local("addr", module.memory.alloc(Wat::const_i32(1))),
                Wat::log_var("addr"),

                Wat::call("log_i32", vec![ module.memory.alloc(Wat::const_i32(1))]),
                Wat::call("log_i32", vec![ module.memory.alloc(Wat::const_i32(1))]),

                module.memory.free(Wat::get_local("addr")),
                Wat::set_local("addr", module.memory.alloc(Wat::const_i32(1))),
                Wat::log_var("addr"),

                Wat::call("log_i32", vec![ module.memory.alloc(Wat::const_i32(1))]),
            ])
        ]
    }
}