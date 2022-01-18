use std::{collections::{HashMap, hash_map::DefaultHasher}, hash::{Hash, Hasher}, rc::Rc};
use indexmap::IndexMap;
use crate::{program::OBJECT_HEADER_SIZE, utils::Link};
use super::{FieldInstance, ItemGenerator, ProgramContext, TypeBlueprint, TypeIndex, TypeInstanceContent, TypeInstanceHeader, type_blueprint};

#[derive(Debug, Clone)]
pub struct TypeInstanceParameters {
    pub type_blueprint: Link<TypeBlueprint>,
    pub type_parameters: Vec<Rc<TypeInstanceHeader>>,
}

impl  TypeInstanceParameters {
    pub fn get_id(&self) -> u64 {
        let mut state = DefaultHasher::new();

        self.type_blueprint.get_addr().hash(&mut state);
        self.type_parameters.hash(&mut state);

        state.finish()
    }
}