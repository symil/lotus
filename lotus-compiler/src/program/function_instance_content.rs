use std::rc::Rc;
use crate::{program::{FunctionInstanceWasmType, THIS_VAR_NAME, TypeIndex, VariableInfo, VariableKind}, utils::Link};
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

                let is_static = function_unwrapped.signature.this_type.is_none();
                let mut variables = vec![];
                let mut wat_args = vec![];
                let mut wat_ret = vec![];
                let mut wat_locals = vec![];
                let mut wat_body = function_unwrapped.body.resolve(&type_index, context);
                let mut list : Vec<(VariableInfo, String)> = vec![];

                if !function_unwrapped.is_raw_wasm {
                    if let Some(this_type) = &function_unwrapped.signature.this_type {
                        if let Some(wasm_type) = this_type.resolve(&type_index, context).wasm_type {
                            wat_args.push((THIS_VAR_NAME.to_string(), wasm_type));
                        }
                    }

                    for (arg_name, arg_type) in function_unwrapped.argument_names.iter().zip(function_unwrapped.signature.argument_types.iter()) {
                        if let Some(wasm_type) = arg_type.resolve(&type_index, context).wasm_type {
                            wat_args.push((arg_name.to_unique_string(), wasm_type));
                        }
                    }

                    if let Some(wasm_type) = function_unwrapped.signature.return_type.resolve(&type_index, context).wasm_type {
                        wat_ret.push(wasm_type);
                    }

                    function_unwrapped.body.collect_variables(&mut variables);

                    for var_info in variables {
                        if let Some(wasm_type) = var_info.ty().resolve(&type_index, context).wasm_type {
                            let mut array = match var_info.kind().clone() {
                                VariableKind::Global => unreachable!(),
                                VariableKind::Local => &mut wat_locals,
                                VariableKind::Argument => &mut wat_args,
                            };

                            array.push((var_info.get_wasm_name(), wasm_type))
                        }
                    }
                }

                let wasm_type = FunctionInstanceWasmType {
                    arg_types: wat_args.iter().map(|(name, ty)| *ty).collect(),
                    return_types: wat_ret.clone(),
                };

                wasm_declaration = Some(Wat::declare_function(&header.wasm_name, None, wat_args, wat_ret, wat_locals, wat_body));

                if let Some(index) = header.function_index {
                    context.assign_function_to_index(index, &header);
                }
            }

            FunctionInstanceContent {
                wasm_declaration,
                wasm_type_name
            }
        })
    }
}