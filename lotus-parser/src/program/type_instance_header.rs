use std::{borrow::Borrow, collections::HashMap, hash::Hash, rc::Rc};
use indexmap::IndexMap;
use crate::utils::Link;
use super::{FunctionBlueprint, ItemGenerator, OBJECT_HEADER_SIZE, ProgramContext, TypeBlueprint, TypeInstanceParameters};

#[derive(Debug)]
pub struct TypeInstanceHeader {
    pub id: u64,
    pub name: String,
    pub type_blueprint: Link<TypeBlueprint>,
    pub parameters: Vec<Rc<TypeInstanceHeader>>,
    pub wasm_type: Option<&'static str>,
    pub dynamic_method_table_offset: usize,
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

impl TypeInstanceHeader {
    pub fn from_parameters(instance_parameters: &TypeInstanceParameters, context: &mut ProgramContext) -> Rc<Self> {
        instance_parameters.type_blueprint.with_ref(|type_unwrapped| {
            let id = instance_parameters.get_id();
            let type_blueprint = instance_parameters.type_blueprint.clone();
            let parameters = instance_parameters.type_parameters.clone();
            let wasm_type = type_unwrapped.get_wasm_type(&instance_parameters.type_parameters);
            let dynamic_method_count = type_unwrapped.dynamic_methods.len();
            let dynamic_method_table_offset = context.dynamic_method_table.len();
            let mut name = type_unwrapped.name.to_string();

            for parameter in &instance_parameters.type_parameters {
                name.push_str(&format!("_{}", &parameter.name));
            }

            for i in 0..dynamic_method_count.max(1) {
                context.dynamic_method_table.push(None);
            }

            Rc::new(TypeInstanceHeader {
                id,
                name,
                type_blueprint,
                parameters,
                wasm_type,
                dynamic_method_table_offset
            })
        })
    }

    pub fn get_placeholder_function_wasm_type_name(&self, function_wrapped: &Link<FunctionBlueprint>) -> String {
        function_wrapped.with_ref(|function_unwrapped| {
            format!("{}_{}_{}_{}", &function_unwrapped.owner_type.as_ref().unwrap().borrow().name, &function_unwrapped.name, function_unwrapped.dynamic_index, self.id)
        })
    }
}