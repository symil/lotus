use std::{collections::{HashMap, hash_map::DefaultHasher}, hash::{Hash, Hasher}, rc::Rc};
use indexmap::IndexMap;
use crate::{items::StackType, program::OBJECT_HEADER_SIZE, utils::Link};
use super::{FieldInstance, ItemGenerator, ProgramContext, TypeBlueprint, TypeIndex, TypeInstanceContent, TypeInstanceHeader, type_blueprint};

#[derive(Debug, Clone)]
pub struct TypeInstanceParameters {
    pub type_blueprint: Link<TypeBlueprint>,
    pub type_parameters: Vec<Rc<TypeInstanceHeader>>,
}

impl ItemGenerator<TypeInstanceHeader, TypeInstanceContent> for TypeInstanceParameters {
    fn get_id(&self) -> u64 {
        let mut state = DefaultHasher::new();

        self.type_blueprint.get_addr().hash(&mut state);
        self.type_parameters.hash(&mut state);

        state.finish()
    }

    fn generate_header(&self, id: u64) -> TypeInstanceHeader {
        self.type_blueprint.with_ref(|type_unwrapped| {
            let type_blueprint = self.type_blueprint.clone();
            let parameters = self.type_parameters.clone();
            let wasm_type = type_unwrapped.get_wasm_type();
            let mut name = type_unwrapped.name.to_string();

            for parameter in &self.type_parameters {
                name.push_str(&format!("_{}", &parameter.name));
            }

            TypeInstanceHeader {
                id,
                name,
                type_blueprint,
                parameters,
                wasm_type,
            }
        })
    }

    fn generate_content(&self, header: &Rc<TypeInstanceHeader>, context: &mut ProgramContext) -> TypeInstanceContent {
        self.type_blueprint.with_ref(|type_unwrapped| {
            // let type_index = TypeIndex {
            //     current_type_instance: Some(header.clone()),
            //     current_function_parameters: vec![],
            // };
            // let mut associated_types = HashMap::new();

            // for (name, associated_type) in type_unwrapped.associated_types.iter() {
            //     associated_types.insert(name.clone(), associated_type.resolve(&type_index, context));
            // }

            TypeInstanceContent {
                // associated_types,
            }
        })
    }
}