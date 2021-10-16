use std::rc::Rc;
use crate::{program::{FunctionInstanceWasmType, TypeIndex, VariableKind}, utils::Link};
use super::{FunctionInstanceHeader, FunctionInstanceParameters, ProgramContext, Wat};

#[derive(Debug)]
pub struct FunctionInstanceContent {
    pub wasm_declaration: Option<Wat>,
    pub wasm_type_name: String
}

impl FunctionInstanceContent {
    pub fn from_parameters(parameters: &FunctionInstanceParameters, header: Rc<FunctionInstanceHeader>, context: &mut ProgramContext) -> Self {
        parameters.function_blueprint.with_ref(|function_unwrapped| {
            let mut wasm_declaration = None;
            let mut wasm_type_name = String::new();

            if !function_unwrapped.is_raw_wasm {
                let type_index = TypeIndex {
                    current_type_instance: parameters.this_type.clone(),
                    current_function_parameters: parameters.function_parameters.clone(),
                };

                let is_static = function_unwrapped.this_arg.is_none();
                let mut variables = vec![];
                let mut wat_args : Vec<(&str, &str)> = vec![];
                let mut wat_ret = vec![];
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
                            wat_ret.push(wasm_type);
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

                let wasm_type = FunctionInstanceWasmType {
                    arg_types: wat_args.iter().map(|(name, ty)| *ty).collect(),
                    return_types: wat_ret.clone(),
                };

                wasm_declaration = Some(Wat::declare_function(&header.wasm_name, None, wat_args, wat_ret, wat_locals, wat_body));

                if function_unwrapped.is_dynamic {
                    wasm_type_name = context.get_function_instance_wasm_type_name(wasm_type);

                    let placeholder = parameters.this_type.as_ref().unwrap().get_placeholder_function_wasm_type_name(&parameters.function_blueprint);

                    context.placeholder_to_wasm_type.insert(placeholder, wasm_type_name.clone());
                }
            }

            FunctionInstanceContent {
                wasm_declaration,
                wasm_type_name
            }
        })
    }
}