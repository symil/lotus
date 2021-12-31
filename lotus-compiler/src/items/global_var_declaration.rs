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
        if let Some((var_list, init_vasm)) = self.var_declaration.process(context) {
            if var_list.len() != 1 {
                context.errors.generic(self, format!("cannot declare global variabels as tuples"));
            }

            let global_var_blueprint = GlobalVarBlueprint {
                var_id: self.location.get_hash(),
                name: var_list.first().unwrap().name().clone(),
                visibility: self.visibility.value.unwrap_or(Visibility::Private),
                var_info: var_list.first().unwrap().clone(),
                init_vasm,
            };

            let var_name = self.var_declaration.get_first_var_name();

            if context.global_vars.get_by_identifier(var_name).is_some() {
                context.errors.generic(var_name, format!("duplicate global variable declaration: `{}`", var_name));
            }

            context.global_vars.insert(global_var_blueprint, None);
        }
    }
}