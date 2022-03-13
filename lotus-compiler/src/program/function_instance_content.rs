use std::rc::Rc;
use crate::{program::{CLOSURE_VARIABLES_VAR_NAME, FunctionInstanceWasmType, SELF_VAR_NAME, TypeIndex, VariableInfo, VariableKind}, utils::Link};
use super::{FunctionInstanceHeader, FunctionInstanceParameters, ProgramContext, Wat, Vasm, FunctionBody};

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

            if let FunctionBody::Vasm(body_vasm) = &function_unwrapped.body {
                let type_index = TypeIndex {
                    current_type_instance: parameters.this_type.clone(),
                    current_function_parameters: parameters.function_parameters.clone(),
                };

                let is_static = function_unwrapped.signature.this_type.is_none();
                let mut variables = vec![];
                let mut wat_args = vec![];
                let mut wat_ret = vec![];
                let mut wat_locals = vec![];
                let mut wat_body = vec![];
                let mut list : Vec<(VariableInfo, String)> = vec![];

                for arg_var in &function_unwrapped.argument_variables {
                    variables.push(arg_var.clone());
                    wat_body.extend(context.vasm().init_var(arg_var).resolve(&type_index, context));
                }

                // println!("{}", &function_unwrapped.name);
                // function_unwrapped.signature.return_type.print();
                if let Some(wasm_type) = function_unwrapped.signature.return_type.resolve(&type_index, context).wasm_type {
                    wat_ret.push(wasm_type);
                }

                body_vasm.collect_variables(&mut variables);

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

                if function_unwrapped.is_closure() {
                    wat_args.push((CLOSURE_VARIABLES_VAR_NAME.to_string(), "i32"));
                }

                wat_body.extend(body_vasm.resolve(&type_index, context));

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