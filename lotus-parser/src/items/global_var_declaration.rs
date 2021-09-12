use std::fs::Metadata;

use parsable::parsable;
use crate::program::{GlobalVarBlueprint, GlobalVarInstance, ProgramContext, VariableInfo, VariableKind};
use super::{VarDeclaration, Visibility, VisibilityWrapper};

#[parsable]
pub struct GlobalVarDeclaration {
    pub visibility: VisibilityWrapper,
    #[parsable(suffix=";")]
    pub var_declaration: VarDeclaration,
}

impl GlobalVarDeclaration {
    pub fn process(&self, context: &mut ProgramContext) {
        context.reset_local_scope();

        if let Some((var_info, init_vasm)) = self.var_declaration.process(VariableKind::Global, context) {
            let global_var_blueprint = GlobalVarBlueprint {
                var_id: self.location.get_hash(),
                visibility: self.visibility.value.unwrap_or(Visibility::Private),
                var_info,
                init_vasm,
            };

            if context.global_vars.get_by_name(&self.var_declaration.var_name).is_some() {
                context.errors.add(&self.var_declaration.var_name, format!("duplicate global variable declaration: `{}`", &self.var_declaration.var_name));
            }

            context.global_vars.insert(global_var_blueprint);
        }
    }
}