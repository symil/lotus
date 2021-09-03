use crate::{generation::Wat, items::ValueType};

pub struct WasmBlueprint {
    pub ty: ValueType,
    pub blocks: Vec<WasmBlueprintBlock>
}

pub enum WasmBlueprintBlock {
    Wasm(Vec<Wat>),
    GenericMethodCall(GenericMethodCall)
}

pub struct GenericMethodCall {
    pub type_name: String,
    pub method_name: String
}