use std::{hash::Hash, rc::Rc};
use indexmap::IndexMap;

use crate::utils::Link;
use super::TypeBlueprint;

#[derive(Debug)]
pub struct TypeInstanceHeader {
    pub id: u64,
    pub name: String,
    pub type_blueprint: Link<TypeBlueprint>,
    pub parameters: Vec<Rc<TypeInstanceHeader>>,
    pub fields: IndexMap<String, FieldInstance>,
    pub wasm_type: Option<&'static str>
}

#[derive(Debug)]
pub struct FieldInstance {
    pub ty: Rc<TypeInstanceHeader>,
    pub offset: usize
}

impl Hash for TypeInstanceHeader {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}