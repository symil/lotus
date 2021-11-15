use std::{hash::Hash, rc::Rc};
use crate::{items::Visibility, program::FunctionInstanceWasmType};
use super::{FunctionInstanceParameters, ProgramContext, TypeInstanceHeader, Wat};

#[derive(Debug)]
pub struct FunctionInstanceHeader {
    pub id: u64,
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
            let mut wasm_call = match function_unwrapped.is_raw_wasm {
                true => function_unwrapped.body.resolve_without_context(),
                false => vec![
                    Wat::call_from_stack(&wasm_name)
                ],
            };
            let this_type = parameters.this_type.clone();

            if let Some(this_type) = &parameters.this_type {
                this_type.type_blueprint.with_ref(|type_unwrapped| {
                    for parameter in type_unwrapped.parameters.values() {
                        let p = &this_type.parameters[parameter.index];
                        let wasm_type = p.type_blueprint.borrow().get_wasm_type(&p.parameters).unwrap();

                        for wat in &mut wasm_call {
                            wat.replace(&parameter.wasm_pattern, wasm_type);
                        }
                    }
                });
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
                this_type,
                wasm_name,
                wasm_call,
            })
        })
    }
}

impl Hash for FunctionInstanceHeader {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}