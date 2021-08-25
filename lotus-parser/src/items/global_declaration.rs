use std::fs::Metadata;

use parsable::parsable;
use crate::program::{GlobalAnnotation, ItemMetadata, ProgramContext, Type, VariableInfo, VariableKind, Wasm};
use super::{VarDeclaration, Visibility};

#[parsable]
pub struct GlobalDeclaration {
    pub visibility: Visibility,
    #[parsable(suffix=";")]
    pub var_declaration: VarDeclaration,

    #[parsable(ignore)]
    pub file_name: String,
    #[parsable(ignore)]
    pub namespace_name: String
}

impl GlobalDeclaration {
    pub fn process(&self, index: usize, context: &mut ProgramContext) {
        context.set_file_location(&self.file_name, &self.namespace_name);
        context.reset_local_scope();

        if let Some(wasm) = self.var_declaration.process(VariableKind::Global, context) {
            let mut global_annotation = GlobalAnnotation {
                metadata: ItemMetadata {
                    id: index,
                    name: self.var_declaration.var_name.clone(),
                    file_name: context.get_current_file_name(),
                    namespace_name: context.get_current_namespace_name(),
                    visibility: self.visibility.get_token()
                },
                var_info: wasm.variables[0].clone(),
                value: wasm.wat,
            };

            if context.get_global_by_name(&self.var_declaration.var_name).is_some() {
                context.error(&self.var_declaration.var_name, format!("duplicate global declaration: `{}`", &self.var_declaration.var_name));
            }

            context.add_global(global_annotation);
        }
    }
}