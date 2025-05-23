use parsable::parsable;
use crate::program::{GlobalVarBlueprint, GlobalVarInstance, ProgramContext, VariableInfo, VariableKind, Visibility};
use super::{ParsedVarDeclaration, ParsedVisibilityToken, ParsedVisibility, ParsedSemicolonToken, unwrap_item};

#[parsable]
pub struct ParsedGlobalVarDeclaration {
    pub visibility: Option<ParsedVisibility>,
    pub var_declaration: ParsedVarDeclaration,
    pub semicolon: Option<ParsedSemicolonToken>,
}

impl ParsedGlobalVarDeclaration {
    pub fn process(&self, context: &mut ProgramContext) {
        if let Some((var_list, init_vasm)) = self.var_declaration.process(context) {
            if var_list.len() != 1 {
                context.errors.generic(self, format!("cannot declare global variabels as tuples"));
            }

            let var_info = var_list.first().unwrap().clone();
            let visibility = ParsedVisibility::process_or(&self.visibility, Visibility::Private);
            let global_var_blueprint = GlobalVarBlueprint {
                var_id: self.location.get_hash(),
                name: var_list.first().unwrap().name().clone(),
                visibility,
                var_info: var_info.clone(),
                init_vasm,
            };

            let var_name = var_info.name().clone();

            if visibility.is_system() {
                var_info.with_mut(|mut var_content| var_content.wasm_name = var_name.to_string());
            }

            if context.global_vars.get_by_identifier(&var_name).is_some() {
                context.errors.generic(&var_name, format!("duplicate global variable declaration: `{}`", var_name));
            }

            context.global_vars.insert(global_var_blueprint, None);

            unwrap_item(&self.semicolon, self, context);
        }
    }
}