use std::{borrow::Borrow, collections::HashMap, hash::Hash, rc::Rc};
use indexmap::IndexMap;
use parsable::ItemLocation;
use crate::utils::Link;
use super::{ActualTypeContent, FieldKind, FunctionBlueprint, ItemGenerator, OBJECT_HEADER_SIZE, ProgramContext, Type, TypeBlueprint, TypeInstanceParameters, TypeIndex};

#[derive(Debug)]
pub struct TypeInstanceHeader {
    pub id: u64,
    pub name: String,
    pub ty: Type,
    pub type_blueprint: Link<TypeBlueprint>,
    pub parameters: HashMap<ItemLocation, Rc<TypeInstanceHeader>>,
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
            let dynamic_method_count = type_unwrapped.dynamic_methods.len();
            let dynamic_method_table_offset = context.reserve_next_function_index();
            let location = &ItemLocation::default();
            let mut parameters = HashMap::new();
            let mut type_parameters = vec![];
            let mut name = type_unwrapped.name.to_string();

            for parameter in &instance_parameters.type_parameters {
                name.push_str(&format!("_{}", &parameter.name));
                type_parameters.push(parameter.ty.clone());
            }

            let ty = Type::actual(&type_blueprint, type_parameters, location);

            if instance_parameters.type_parameters.len() != type_blueprint.borrow().parameters.len() {
                panic!("type `{}`: expected {} parameters, got {}", type_blueprint.borrow().name.as_str(), type_blueprint.borrow().parameters.len(), instance_parameters.type_parameters.len());
            }

            for _ in 1..dynamic_method_count {
                context.reserve_next_function_index();
            }

            let mut current_type_opt = Some(ty.clone());
            let empty_type_index = TypeIndex::empty();

            while let Some(current_type) = current_type_opt {
                let info = current_type.as_actual().unwrap();

                info.type_blueprint.with_ref(|type_unwrapped| {
                    for (param_info, ty) in type_unwrapped.parameters.values().zip(info.parameters.iter()) {
                        let key = param_info.key.clone();
                        // TODO: protect against cycles? (which should be invalid)
                        let type_instance = ty.resolve(&empty_type_index, context);

                        parameters.insert(key, type_instance);
                    }
                });

                current_type_opt = current_type.get_parent();
            }

            let wasm_type = type_unwrapped.get_wasm_type(&parameters);

            Rc::new(TypeInstanceHeader {
                id,
                name,
                ty,
                type_blueprint,
                parameters,
                wasm_type,
                dynamic_method_table_offset
            })
        })
    }

    pub fn get_method(&self, kind: FieldKind, name: &str) -> Option<Link<FunctionBlueprint>> {
        self.type_blueprint.with_ref(|type_unwrapped| {
            let index_map = match kind {
                FieldKind::Static => &type_unwrapped.static_methods,
                FieldKind::Regular => &type_unwrapped.regular_methods,
            };

            index_map.get(name).and_then(|func_ref| Some(func_ref.function.clone()))
        })
    }

    pub fn get_type_id(&self) -> usize {
        self.dynamic_method_table_offset
    }

    pub fn has_wasm_type(&self) -> bool {
        self.wasm_type.is_some()
    }
}