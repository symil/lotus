use std::rc::Rc;
use parsable::ItemLocation;
use crate::{items::{Identifier, ParsedVisibilityToken}, program::{RETAIN_METHOD_NAME, VariableKind}};
use super::{GlobalItem, GlobalVarInstance, ProgramContext, TypeIndex, VariableInfo, Vasm, Visibility};

#[derive(Debug)]
pub struct GlobalVarBlueprint {
    pub var_id: u64,
    pub name: Identifier,
    pub visibility: Visibility,
    pub var_info: VariableInfo,
    pub init_vasm: Vasm,
}

impl GlobalVarBlueprint {
    pub fn generate_instance(&self, context: &mut ProgramContext) -> GlobalVarInstance {
        let type_index = TypeIndex::empty();
        let wasm_name = self.var_info.get_wasm_name().to_string();
        let wasm_type = self.var_info.ty().resolve(&type_index, context).wasm_type.unwrap();

        let mut local_var_list = vec![];
        let mut wasm_locals = vec![];
        self.init_vasm.collect_variables(&mut local_var_list);

        for var_info in local_var_list {
            if var_info.kind().is_local() {
                if let Some(wasm_type) = var_info.ty().resolve(&type_index, context).wasm_type {
                    wasm_locals.push((wasm_type, var_info.get_wasm_name()));
                }
            }
        }

        let init_wat = self.init_vasm.resolve(&type_index, context);
        let retain_wat = context.vasm()
            .call_static_method(&self.var_info.ty(), RETAIN_METHOD_NAME, &[], vec![
                context.vasm().get_var(&self.var_info, None)
            ], context)
            .resolve(&type_index, context);

        GlobalVarInstance {
            wasm_name,
            wasm_type,
            init_wat,
            retain_wat,
            wasm_locals,
        }
    }
}

impl GlobalItem for GlobalVarBlueprint {
    fn get_name(&self) -> &Identifier { &self.name }
    fn get_visibility(&self) -> Visibility { self.visibility }
}