use std::{collections::hash_map::DefaultHasher, hash::{Hash, Hasher}, rc::Rc};
use indexmap::IndexMap;
use crate::{items::Visibility, program::{TypeIndex, VariableKind, Wat}, utils::Link};
use super::{FunctionBlueprint, FunctionInstanceContent, FunctionInstanceHeader, GeneratedItemIndex, ItemGenerator, ProgramContext, TypeBlueprint, TypeInstanceContent, TypeInstanceHeader};

#[derive(Debug, Clone)]
pub struct FunctionInstanceParameters {
    pub function_blueprint: Link<FunctionBlueprint>,
    pub this_type: Option<Rc<TypeInstanceHeader>>,
    pub function_parameters: Vec<Rc<TypeInstanceHeader>>
}

impl FunctionInstanceParameters {
    pub fn get_id(&self) -> u64 {
        let mut state = DefaultHasher::new();

        self.function_blueprint.get_addr().hash(&mut state);
        self.this_type.hash(&mut state);
        self.function_parameters.hash(&mut state);

        state.finish()
    }
}