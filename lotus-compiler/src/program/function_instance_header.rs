use std::{hash::Hash, rc::Rc};
use crate::{items::ParsedVisibilityToken, program::FunctionInstanceWasmType, utils::Link};
use super::{FunctionBlueprint, FunctionInstanceParameters, ProgramContext, TypeInstanceHeader, Wat, Visibility, FunctionBody};

#[derive(Debug)]
pub struct FunctionInstanceHeader {
    pub id: u64,
    pub function_index: Option<usize>,
    pub this_type: Option<Rc<TypeInstanceHeader>>,
    pub wasm_name: String,
    pub wasm_call: Vec<Wat>,
}

impl FunctionInstanceHeader {
    pub fn from_parameters(parameters: &FunctionInstanceParameters, context: &mut ProgramContext) -> Rc<Self> {
        parameters.function_blueprint.with_ref(|function_unwrapped| {
            let id = parameters.get_id();
            let wasm_name = match function_unwrapped.visibility {
                Visibility::System => function_unwrapped.name.to_string(),
                _ => match &parameters.this_type {
                    Some(type_instance) => format!("{}_{}_{}", &type_instance.name, &function_unwrapped.name, id),
                    None => format!("{}_{}", &function_unwrapped.name, id),
                }
            };
            let mut wasm_call = match &function_unwrapped.body {
                FunctionBody::Empty => unreachable!(),
                FunctionBody::Vasm(_) => vec![ Wat::call_from_stack(&wasm_name) ],
                FunctionBody::RawWasm(wat) => wat.clone(),
                FunctionBody::Import(_, _) => vec![ Wat::call_from_stack(&wasm_name) ],
            };
            let this_type = parameters.this_type.clone();
            let mut function_index = None;

            if let Some(this_type) = &parameters.this_type {
                this_type.type_blueprint.with_ref(|type_unwrapped| {
                    for parameter in type_unwrapped.parameters.values() {
                        let p = &this_type.parameters[parameter.index];

                        if let Some(wasm_type) = p.type_blueprint.borrow().get_wasm_type(&p.parameters) {
                            for wat in &mut wasm_call {
                                wat.replace(&parameter.wasm_pattern, wasm_type);
                            }
                        }
                    }
                });
            }

            if function_unwrapped.method_details.is_none() || function_unwrapped.is_event_callback() {
                function_index = Some(context.reserve_next_function_index());
            }

            for parameter in function_unwrapped.parameters.values() {
                let p = &parameters.function_parameters[parameter.index];

                if let Some(wasm_type) = p.type_blueprint.borrow().get_wasm_type(&p.parameters) {
                    for wat in &mut wasm_call {
                        wat.replace(&parameter.wasm_pattern, wasm_type);
                    }
                }
            }

            Rc::new(FunctionInstanceHeader {
                id,
                function_index,
                this_type,
                wasm_name,
                wasm_call,
            })
        })
    }

    pub fn get_placeholder_function_wasm_type_name(&self, function_wrapped: &Link<FunctionBlueprint>) -> String {
        function_wrapped.with_ref(|function_unwrapped| {
            format!("{}_{}", &function_unwrapped.name, self.id)
        })
    }
}

impl Hash for FunctionInstanceHeader {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}