use parsable::parsable;
use crate::program::{GlobalAnnotation, ProgramContext, Type, VariableScope, Wasm};
use super::VarDeclaration;

#[parsable]
pub struct GlobalDeclaration {
    #[parsable(suffix=";")]
    pub var_declaration: VarDeclaration
}

impl GlobalDeclaration {
    pub fn process_declaration(&self, index: usize, context: &mut ProgramContext) {
        if let Some(global_type) = Type::from_parsed_type(&self.var_declaration.var_type, context) {
            let mut global_annotation = GlobalAnnotation::default();

            global_annotation.index = index;
            global_annotation.wasm_name = format!("{}", &self.var_declaration.var_name);
            global_annotation.ty = global_type;

            if context.globals.contains_key(&self.var_declaration.var_name) {
                context.error(&self.var_declaration.var_name, format!("duplicate global declaration: `{}`", &self.var_declaration.var_name));
            }
            
            context.globals.insert(&self.var_declaration.var_name, global_annotation);
        }
    }

    pub fn process_assignment(&self, index: usize, context: &mut ProgramContext) {
        context.reset_local_scope(VariableScope::Global);
        
        if let Some(wasm) = self.var_declaration.process(context) {
            if let Some(global_annotation) = context.globals.get_mut_by_id(&self.var_declaration.var_name, index) {
                global_annotation.value = wasm.wat;
            }
        }
    }
}