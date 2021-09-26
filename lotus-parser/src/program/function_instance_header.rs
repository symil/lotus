use std::{hash::Hash, rc::Rc};
use super::{ProgramContext, TypeInstanceHeader, Wat};

#[derive(Debug)]
pub struct FunctionInstanceHeader {
    pub id: u64,
    pub this_type: Option<Rc<TypeInstanceHeader>>,
    pub wasm_name: String,
    pub wasm_call: Vec<Wat>
}

impl FunctionInstanceHeader {

}

impl Hash for FunctionInstanceHeader {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}