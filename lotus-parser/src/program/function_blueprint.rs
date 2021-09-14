use std::rc::Rc;

use indexmap::{IndexMap, IndexSet};
use parsable::DataLocation;
use crate::{items::{EventCallbackQualifier, Identifier, Visibility}, program::{VariableKind, Wat}, utils::Link};
use super::{FunctionInstance, GlobalItem, InterfaceBlueprint, ParameterType, ProgramContext, ResolvedType, Type, TypeBlueprint, TypeContext, VariableInfo, Vasm, VirtualInstruction};

#[derive(Debug)]
pub struct FunctionBlueprint {
    pub function_id: u64,
    pub name: Identifier,
    pub visibility: Visibility,
    pub parameters: IndexMap<String, Link<ParameterType>>,
    pub event_callback_qualifier: Option<EventCallbackQualifier>,
    pub owner_type: Option<Link<TypeBlueprint>>,
    pub owner_interface: Option<Link<InterfaceBlueprint>>,
    pub conditions: Vec<(Identifier, Identifier)>,
    pub this_arg: Option<Rc<VariableInfo>>,
    pub payload_arg: Option<Rc<VariableInfo>>,
    pub arguments: Vec<Rc<VariableInfo>>,
    pub return_value: Option<Rc<VariableInfo>>,
    pub is_raw_wasm: bool,
    pub is_dynamic: bool,
    pub dynamic_index: i32,
    pub body: Vasm
}

impl FunctionBlueprint {
    pub fn is_static(&self) -> bool {
        self.this_arg.is_none()
    }

    pub fn generate_instance(&self, type_context: &TypeContext) -> FunctionInstance {
        let is_static = self.this_arg.is_none();
        let mut variables = vec![];
        let mut wat_args : Vec<(&str, &str)> = vec![];
        let mut wat_ret = None;
        let mut wat_locals : Vec<(&str, &str)> = vec![];
        let mut wat_body = self.body.resolve();

        if !self.is_raw_wasm {
            if let Some(this_arg) = &self.this_arg {
                variables.push(Rc::clone(this_arg));
            }

            if let Some(payload_arg) = &self.payload_arg {
                variables.push(Rc::clone(payload_arg));
            }

            for arg in &self.arguments {
                variables.push(Rc::clone(arg));
            }

            if let Some(return_value) = &self.return_value {
                if let Some(wasm_type) = return_value.ty.resolve().get_wasm_type() {
                    wat_ret = Some(wasm_type);
                    wat_locals.push((&return_value.wasm_name, wasm_type));
                    wat_body.push(Wat::get_local(&return_value.wasm_name));
                }
            }

            self.body.collect_variables(&mut variables);

            for var_info in &variables {
                if let Some(wasm_type) = var_info.ty.resolve().get_wasm_type() {
                    let mut array = match var_info.kind {
                        VariableKind::Global => unreachable!(),
                        VariableKind::Local => &mut wat_locals,
                        VariableKind::Argument => &mut wat_args,
                    };

                    array.push((var_info.wasm_name.as_str(), wasm_type))
                }
            }
        }

        let prefix = match &self.this_arg {
            Some(var_info) => format!("{}_", &var_info.ty.resolve().type_blueprint.borrow().name),
            None => String::new(),
        };
        let wasm_name = format!("{}{}_{}", prefix, &self.name, type_context.get_name());
        let (wasm_declaration, wasm_call) = match self.is_raw_wasm {
            true => ((
                Some(Wat::declare_function(&wasm_name, None, wat_args, wat_ret, wat_locals, wat_body)),
                vec![Wat::call_from_stack(&wasm_name)]
            )),
            false => ((
                None,
                wat_body,
            )),
        };

        FunctionInstance {
            wasm_name,
            wasm_declaration,
            wasm_call,
        }
    }
}

impl GlobalItem for FunctionBlueprint {
    fn get_name(&self) -> &Identifier { &self.name }
    fn get_visibility(&self) -> Visibility { self.visibility }
}