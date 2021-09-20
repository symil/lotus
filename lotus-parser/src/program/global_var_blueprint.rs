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
        let value = self.init_vasm.resolve(&type_index, context);

        GlobalVarInstance {
            wasm_name,
            wasm_type,
            init_value: value,
        }
    }
}

impl GlobalItem for GlobalVarBlueprint {
    fn get_name(&self) -> &Identifier { &self.var_info.name }
    fn get_visibility(&self) -> Visibility { self.visibility }
}