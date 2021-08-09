use std::collections::HashMap;
use parsable::parsable;
use crate::{generation::{Wat, ToWat, ToWatVec}, merge, program::{ProgramContext, Type, VarInfo, Wasm}};
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
        if context.inside_const_expr && self.qualifier.is_none() {
            context.error(self, format!("global variables must be declared with the `const` qualifier"));
        } else if !context.inside_const_expr && self.qualifier.is_some() {
            context.error(self, format!("local variables must be declared without the `const` qualifier"));
        }

        let var_exists = context.var_exists(&self.var_name);

        if var_exists {
            context.error(&self.var_name, format!("duplicate variable declaration: `{}` already exists in this scope", &self.var_name));
        }

        let var_type_opt = Type::from_parsed_type(&self.var_type, context);
        let var_wasm_opt = self.init_value.process(context);

        let mut result = None;

        if let Some(var_type) = var_type_opt {
            context.push_var(&self.var_name, VarInfo::new(&self.var_name, &var_type));

            if let Some(var_wasm) = var_wasm_opt {
                if var_type.is_assignable(&var_wasm.ty, context, &mut HashMap::new()) {
                    result = Some(Wasm::untyped(merge![var_wasm.wat, Wat::set_local_from_stack(self.var_name.as_str())]));
                }
            }
        }

        result
    }
}