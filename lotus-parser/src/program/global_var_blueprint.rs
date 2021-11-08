use std::rc::Rc;
use parsable::DataLocation;
use crate::items::{Identifier, Visibility};
use super::{GlobalItem, GlobalVarInstance, ProgramContext, TypeIndex, VariableInfo, Vasm};

#[derive(Debug)]
pub struct GlobalVarBlueprint {
    pub var_id: u64,
    pub visibility: Visibility,
    pub var_info: Rc<VariableInfo>,
    pub init_vasm: Vasm
}

impl GlobalVarBlueprint {
    pub fn generate_instance(&self, context: &mut ProgramContext) -> GlobalVarInstance {
        let type_index = TypeIndex::empty();
        let wasm_name = self.var_info.get_wasm_name().to_string();
        let wasm_type = self.var_info.ty.resolve(&type_index, context).wasm_type.unwrap();

        let mut local_var_list = vec![];
        let mut wasm_locals = vec![];
        self.init_vasm.collect_variables(&mut local_var_list);

        let type_index = TypeIndex {
            current_type_instance: None,
            current_function_parameters: vec![],
        };

        for var_info in local_var_list {
            if let Some(wasm_type) = var_info.ty.resolve(&type_index, context).wasm_type {
                wasm_locals.push((wasm_type, var_info.wasm_name.clone()));
            }
        }

        let init_value = self.init_vasm.resolve(&type_index, context);


        GlobalVarInstance {
            wasm_name,
            wasm_type,
            init_value,
            wasm_locals,
        }
    }
}

impl GlobalItem for GlobalVarBlueprint {
    fn get_name(&self) -> &Identifier { &self.var_info.name }
    fn get_visibility(&self) -> Visibility { self.visibility }
}