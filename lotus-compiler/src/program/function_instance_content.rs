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
                    for arg_var in &function_unwrapped.argument_variables {
                        variables.push(arg_var.clone());
                    }

                    if let Some(wasm_type) = function_unwrapped.signature.return_type.resolve(&type_index, context).wasm_type {
                        wat_ret.push(wasm_type);
                    }

                    function_unwrapped.body.collect_variables(&mut variables);

                    for var_info in variables {
                        if let Some(wasm_type) = var_info.resolve_wasm_type(&type_index, context) {
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