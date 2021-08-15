use parsable::parsable;
use crate::program::{GlobalAnnotation, ProgramContext, Type, VariableKind, Wasm};
use super::VarDeclaration;

#[parsable]
pub struct GlobalDeclaration {
    #[parsable(suffix=";")]
    pub var_declaration: VarDeclaration
}

impl GlobalDeclaration {
    pub fn process(&self, index: usize, context: &mut ProgramContext) {
        context.reset_local_scope();

        if let Some(wasm) = self.var_declaration.process(VariableKind::Global, context) {
            let mut global_annotation = GlobalAnnotation::default();

            global_annotation.index = index;
            global_annotation.wasm_name = wasm.declared_variables[0].wasm_name.clone();
            global_annotation.ty = wasm.ty;
            global_annotation.value = wasm.wat;

            if context.globals.contains_key(&self.var_declaration.var_name) {
                context.error(&self.var_declaration.var_name, format!("duplicate global declaration: `{}`", &self.var_declaration.var_name));
            }

            context.globals.insert(&self.var_declaration.var_name, global_annotation);
        }
    }
}