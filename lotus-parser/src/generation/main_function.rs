use crate::{generation::MEM_INIT_FUNC_NAME, wat};
use super::{ToWat, WasmModule, Wat, ToWatVec};

pub struct MainFunction;

impl MainFunction {
    pub fn new() -> Self {
        Self
    }

    pub fn get_functions(&self, module: &WasmModule) -> Vec<Wat> {
        vec![
            Wat::declare_function("main", Some("_start"), vec![], None, vec![], vec![
                Wat::declare_local_i32("addr"),
                Wat::call(MEM_INIT_FUNC_NAME, vec![]),
            ])
        ]
    }
}