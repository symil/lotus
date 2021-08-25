use std::collections::HashMap;
use parsable::parsable;
use crate::{generation::{Wat, ToWat, ToWatVec}, program::{ProgramContext, Type, VariableInfo, VariableKind, Wasm}};
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
    pub fn process(&self, kind: VariableKind, context: &mut ProgramContext) -> Option<Wasm> {
        context.ckeck_var_unicity(&self.var_name);

        let mut wat = vec![];
        let mut ok = false;
        let mut final_var_type = Type::Void;

        if let Some(wasm) = self.init_value.process(context) {
            match &self.var_type {
                Some(parsed_type) => match Type::from_parsed_type(parsed_type, context) {
                    Some(var_type) => {
                        final_var_type = var_type.clone();

                        if var_type.is_assignable(&wasm.ty, context, &mut HashMap::new()) {
                            final_var_type = var_type;
                            ok = true;
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
                        final_var_type = wasm.ty;
                        ok = true;
                    } else {
                        context.error(&self.init_value, format!("insufficient infered type `{}` (consider declaring the variable type explicitely)", &wasm.ty));
                    }
                }
            };

            wat.extend(wasm.wat);
        }

        let var_info = context.push_var(&self.var_name, &final_var_type, kind);

        wat.push(var_info.set_from_stack());

        match ok {
            true => Some(Wasm::new(final_var_type, wat, vec![var_info])),
            false => None
        }
    }
}