use std::{collections::HashMap, hash::Hash, rc::Rc};
use indexmap::IndexMap;
use crate::utils::Link;
use super::{OBJECT_HEADER_SIZE, TypeBlueprint};

#[derive(Debug)]
pub struct TypeInstanceHeader {
    pub id: u64,
    pub name: String,
    pub type_blueprint: Link<TypeBlueprint>,
    pub parameters: Vec<Rc<TypeInstanceHeader>>,
    pub wasm_type: Option<&'static str>
}

#[derive(Debug)]
pub struct FieldInstance {
    pub offset: usize,
    pub wasm_type: &'static str
}

impl Hash for TypeInstanceHeader {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}