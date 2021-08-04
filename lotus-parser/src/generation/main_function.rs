use crate::wat;
use super::{ToWat, WasmModule, Wat, ToWatVec};

pub struct MainFunction;

impl MainFunction {
    pub fn new() -> Self {
        Self
    }

    pub fn get_header(&self, module: &WasmModule) -> Vec<Wat> {
        vec![
            Wat::declare_function("main", Some("_start"), vec![], None, vec![
                module.memory.init(),
                wat!["call", "$log_i32", module.memory.alloc() ],
                wat!["call", "$log_i32", module.memory.alloc() ],
                wat!["call", "$log_i32", module.memory.alloc() ],
            ])
        ]
    }
}