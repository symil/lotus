use std::collections::HashMap;
use parsable::parsable;
use crate::{generation::{Wat, ToWat, ToWatVec}, merge, program::{ProgramContext, Type, VarInfo, VariableScope, Wasm}};
use super::{Expression, Identifier, FullType, VarDeclarationQualifier};

#[parsable]
pub struct VarDeclaration {
    pub qualifier: Option<VarDeclarationQualifier>,
    pub var_type: FullType,
    pub var_name: Identifier,
    #[parsable(prefix="=")]
    pub init_value: Expression
}

impl VarDeclaration {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Wasm> {
        if context.current_scope == VariableScope::Global && self.qualifier.is_none() {
            context.error(self, format!("global variables must be declared with the `const` qualifier"));
        } else if context.current_scope == VariableScope::Local && self.qualifier.is_some() {
            context.error(self, format!("local variables must be declared without the `const` qualifier"));
        }
        
        if context.current_scope != VariableScope::Global && context.var_exists(&self.var_name) {
            context.error(&self.var_name, format!("duplicate variable declaration: `{}` already exists in this scope", &self.var_name));
        }

        let var_type_opt = Type::from_parsed_type(&self.var_type, context);
        let var_wasm_opt = self.init_value.process(context);

        let mut result = None;

        if let Some(var_type) = var_type_opt {
            if context.current_scope != VariableScope::Global {
                context.push_local_var(&self.var_name, &var_type);
            }

            if let Some(var_wasm) = var_wasm_opt {
                if var_type.is_assignable(&var_wasm.ty, context, &mut HashMap::new()) {
                    result = Some(Wasm::untyped(merge![var_wasm.wat, context.current_scope.set_from_stack(self.var_name.as_str())]));
                } else {
                    context.error(&self.init_value, format!("assignment: right-hand side type `{}` does not match left-hand side type `{}`", &var_wasm.ty, &var_type));
                }
            }
        }

        result
    }
}