use parsable::parsable;
use crate::program::{GlobalAnnotation, ProgramContext, Type, VariableScope, Wasm};
use super::VarDeclaration;

#[parsable]
pub struct GlobalDeclaration {
    pub var_declaration: VarDeclaration
}

impl GlobalDeclaration {
    pub fn process_declaration(&self, context: &mut ProgramContext) {
        if let Some(global_type) = Type::from_parsed_type(&self.var_declaration.var_type, context) {
            let global_annotation = GlobalAnnotation::default();

            global_annotation.wasm_name = format!("global_{}", &self.var_declaration.var_name);
            global_annotation.ty = global_type;

            if context.globals.contains_key(&self.var_declaration.var_name) {
                context.error(&self.var_declaration.var_name, format!("duplicate global declaration: `{}`", &self.var_declaration.var_name));
            } else {
                context.globals.insert(&self.var_declaration.var_name, global_annotation);
            }
        }
    }

    pub fn process_assignment(&self, context: &mut ProgramContext) -> Option<Wasm> {
        context.reset_local_scope(VariableScope::Global);
        
        self.var_declaration.process(context)
    }
}