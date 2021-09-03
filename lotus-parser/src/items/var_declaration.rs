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

        let mut source = vec![];
        let mut ok = false;
        let mut final_var_type = Type::Void;

        if let Some(wasm) = self.init_value.process(context) {
            if !wasm.ty.is_assignable() {
                context.errors.add(&self.init_value, format!("cannot assign type `{}`", &wasm.ty));
            } else {
                match &self.var_type {
                    Some(parsed_type) => match Type::from_parsed_type(parsed_type, context) {
                        Some(var_type) => {
                            final_var_type = var_type.clone();

                            if var_type.is_assignable_to(&wasm.ty, context, &mut HashMap::new()) {
                                final_var_type = var_type;
                                ok = true;
                            } else {
                                context.errors.add(&self.init_value, format!("assignment: type `{}` does not match type `{}`", &wasm.ty, &var_type));
                            }
                        },
                        None => {}
                    },
                    None => {
                        if !wasm.ty.is_ambiguous() {
                            final_var_type = wasm.ty.clone();
                            ok = true;
                        } else {
                            context.errors.add(&self.init_value, format!("insufficient infered type `{}` (consider declaring the variable type explicitly)", &wasm.ty));
                        }
                    }
                };
            }

            source.push(wasm);
        }

        let var_info = context.push_var(&self.var_name, &final_var_type, kind);

        source.push(Wasm::new(Type::Void, var_info.set_from_stack(), vec![var_info]));

        match ok {
            true => Some(Wasm::merge(final_var_type, source)),
            false => None
        }
    }
}