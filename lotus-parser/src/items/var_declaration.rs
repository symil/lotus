use std::collections::HashMap;
use parsable::parsable;
use crate::{generation::{Wat, ToWat, ToWatVec}, merge, program::{ProgramContext, Type, VariableInfo, VariableKind, Wasm}};
use super::{Expression, Identifier, FullType, VarDeclarationQualifier};

#[parsable]
pub struct VarDeclaration {
    pub qualifier: VarDeclarationQualifier,
    pub var_name: Identifier,
    #[parsable(prefix=":")]
    pub var_type: Option<FullType>,
    #[parsable(prefix="=")]
    pub init_value: Expression
}

impl VarDeclaration {
    pub fn process(&self, scope: VariableKind, context: &mut ProgramContext) -> Option<Wasm> {
        context.ckeck_var_unicity(&self.var_name);

        let mut result = None;
        let mut inferred_type = None;
        let mut var_info = VariableInfo::default();

        if let Some(wasm) = self.init_value.process(context) {
            match &self.var_type {
                Some(parsed_type) => match Type::from_parsed_type(parsed_type, context) {
                    Some(var_type) => {
                        var_info = context.push_var(&self.var_name, &var_type, scope);

                        if var_type.is_assignable(&wasm.ty, context, &mut HashMap::new()) {
                            inferred_type = Some(var_type);
                        } else {
                            context.error(&self.init_value, format!("assignment: right-hand side type `{}` does not match left-hand side type `{}`", &wasm.ty, &var_type));
                        }
                    },
                    None => {}
                },
                None => {
                    let type_ok = match &wasm.ty {
                        Type::Void => false,
                        Type::System => false,
                        Type::Boolean => true,
                        Type::Integer => true,
                        Type::Float => true,
                        Type::String => true,
                        Type::Null => false,
                        Type::TypeId => true,
                        Type::Struct(_) => true,
                        Type::Pointer(_) => true,
                        Type::Array(_) => true,
                        Type::Function(_, _) => true,
                        Type::Any(_) => false,
                    };

                    if type_ok {
                        var_info = context.push_var(&self.var_name, &wasm.ty, scope);
                        inferred_type = Some(wasm.ty.clone());
                    } else {
                        context.error(&self.init_value, format!("insufficient infered type `{}` (consider declaring the variable type explicitely)", &wasm.ty));
                    }
                }
            };

            result = match inferred_type {
                Some(var_type) => Some(Wasm::new(var_type, merge![wasm.wat, var_info.set_from_stack()], vec![var_info])),
                None => None
            }
        }

        result
    }
}