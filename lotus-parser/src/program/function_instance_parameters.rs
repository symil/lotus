use std::{collections::hash_map::DefaultHasher, hash::{Hash, Hasher}, rc::Rc};
use indexmap::IndexMap;
use crate::{program::{TypeIndex, VariableKind, Wat}, utils::Link};
use super::{FunctionBlueprint, FunctionInstanceContent, FunctionInstanceHeader, GeneratedItemIndex, ItemGenerator, ProgramContext, TypeBlueprint, TypeInstanceContent, TypeInstanceHeader};

#[derive(Debug, Clone)]
pub struct FunctionInstanceParameters {
    pub function_blueprint: Link<FunctionBlueprint>,
    pub this_type: Option<Rc<TypeInstanceHeader>>,
    pub function_parameters: Vec<Rc<TypeInstanceHeader>>
}

impl ItemGenerator<FunctionInstanceHeader, FunctionInstanceContent> for FunctionInstanceParameters {
    fn get_id(&self) -> u64 {
        let mut state = DefaultHasher::new();

        self.function_blueprint.get_addr().hash(&mut state);
        self.this_type.hash(&mut state);
        self.function_parameters.hash(&mut state);

        state.finish()
    }

    fn generate_header(&self, id: u64) -> FunctionInstanceHeader {
        self.function_blueprint.with_ref(|function_unwrapped| {
            let wasm_name = match &self.this_type {
                Some(type_instance) => format!("{}_{}_{}", &type_instance.name, &function_unwrapped.name, id),
                None => format!("{}_{}", &function_unwrapped.name, id),
            };
            let mut wasm_call = match function_unwrapped.is_raw_wasm {
                true => function_unwrapped.body.resolve_without_context(),
                false => vec![
                    Wat::call_from_stack(&wasm_name)
                ],
            };
            let this_type = self.this_type.clone();

            if let Some(this_type) = &self.this_type {
                this_type.type_blueprint.with_ref(|type_unwrapped| {
                    for parameter in type_unwrapped.parameters.values() {
                        let wasm_type = this_type.parameters[parameter.index].type_blueprint.borrow().get_wasm_type().unwrap();

                        for wat in &mut wasm_call {
                            wat.replace(&parameter.wasm_pattern, wasm_type);
                        }
                    }
                });
            }

            self.function_blueprint.with_ref(|function_unwrapped| {
                for parameter in function_unwrapped.parameters.values() {
                    let wasm_type = self.function_parameters[parameter.index].type_blueprint.borrow().get_wasm_type().unwrap();

                    for wat in &mut wasm_call {
                        wat.replace(&parameter.wasm_pattern, wasm_type);
                    }
                }
            });

            FunctionInstanceHeader {
                id,
                this_type,
                wasm_name,
                wasm_call,
            }
        })
    }

    fn generate_content(&self, header: &Rc<FunctionInstanceHeader>, context: &mut ProgramContext) -> FunctionInstanceContent {
        self.function_blueprint.with_ref(|function_unwrapped| {
            let wasm_declaration = match function_unwrapped.is_raw_wasm {
                true => None,
                false => {
                    let type_index = TypeIndex {
                        current_type_instance: self.this_type.clone(),
                        current_function_parameters: self.function_parameters.clone(),
                    };

                    let is_static = function_unwrapped.this_arg.is_none();
                    let mut variables = vec![];
                    let mut wat_args : Vec<(&str, &str)> = vec![];
                    let mut wat_ret = None;
                    let mut wat_locals : Vec<(&str, &str)> = vec![];
                    let mut wat_body = function_unwrapped.body.resolve(&type_index, context);

                    if !function_unwrapped.is_raw_wasm {
                        if let Some(this_arg) = &function_unwrapped.this_arg {
                            variables.push(Rc::clone(this_arg));
                        }

                        if let Some(payload_arg) = &function_unwrapped.payload_arg {
                            variables.push(Rc::clone(payload_arg));
                        }

                        for arg in &function_unwrapped.arguments {
                            variables.push(Rc::clone(arg));
                        }

                        if let Some(return_value) = &function_unwrapped.return_value {
                            if let Some(wasm_type) = return_value.ty.resolve(&type_index, context).wasm_type {
                                wat_ret = Some(wasm_type);
                                wat_locals.push((&return_value.wasm_name, wasm_type));
                                wat_body.push(Wat::get_local(&return_value.wasm_name));
                            }
                        }

                        function_unwrapped.body.collect_variables(&mut variables);

                        for var_info in &variables {
                            if let Some(wasm_type) = var_info.ty.resolve(&type_index, context).wasm_type {
                                let mut array = match var_info.kind {
                                    VariableKind::Global => unreachable!(),
                                    VariableKind::Local => &mut wat_locals,
                                    VariableKind::Argument => &mut wat_args,
                                };

                                array.push((var_info.wasm_name.as_str(), wasm_type))
                            }
                        }
                    }
                    
                    Some(Wat::declare_function(&header.wasm_name, None, wat_args, wat_ret, wat_locals, wat_body))
                }
            };

            FunctionInstanceContent {
                wasm_declaration,
            }
        })
    }
}