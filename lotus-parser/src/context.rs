use std::collections::HashMap;

use crate::items::{function_declaration::FunctionDeclaration, struct_declaration::StructDeclaration};

pub struct Context {
    pub types: HashMap<String, StructDeclaration>,
    pub functions: HashMap<String, FunctionDeclaration>
}

impl Context {
}