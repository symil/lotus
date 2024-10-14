use std::hash::Hash;

#[derive(Debug)]
pub struct FunctionInstanceWasmType {
    pub arg_types: Vec<&'static str>,
    pub return_types: Vec<&'static str>
}

impl Hash for FunctionInstanceWasmType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.arg_types.hash(state);
        ".".hash(state);
        self.return_types.hash(state);
    }
}

impl PartialEq for FunctionInstanceWasmType {
    fn eq(&self, other: &Self) -> bool {
        self.arg_types == other.arg_types && self.return_types == other.return_types
    }
}

impl Eq for FunctionInstanceWasmType {

}