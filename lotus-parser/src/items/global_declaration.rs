use std::fs::Metadata;

use parsable::parsable;
use crate::program::{GlobalVarInstance, ProgramContext, VariableInfo, VariableKind};
use super::{VarDeclaration, VisibilityWrapper};

#[parsable]
pub struct GlobalDeclaration {
    pub visibility: VisibilityWrapper,
    #[parsable(suffix=";")]
    pub var_declaration: VarDeclaration,

    #[parsable(ignore)]
    pub file_name: String,
    #[parsable(ignore)]
    pub file_namespace: String
}

impl GlobalDeclaration {
    pub fn process(&self, index: usize, context: &mut ProgramContext) {
        context.set_file_location(&self.file_name, &self.file_namespace);
        context.reset_local_scope();

        if let Some(wasm) = self.var_declaration.process(VariableKind::Global, context) {
            let mut global_annotation = GlobalVarInstance {
                metadata: ItemMetadata {
                    id: index,
                    name: self.var_declaration.var_name.clone(),
                    file_name: context.get_current_file_name(),
                    file_namespace: context.get_current_file_namespace(),
                    visibility: self.visibility.get_token()
                },
                var_info: wasm.variables[0].clone(),
                value: wasm.wat,
            };

            if context.get_global_by_name(&self.var_declaration.var_name).is_some() {
                context.errors.add(&self.var_declaration.var_name, format!("duplicate global declaration: `{}`", &self.var_declaration.var_name));
            }

            context.add_global(global_annotation);
        }
    }
}