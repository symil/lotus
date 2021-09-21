use std::{collections::HashMap, rc::Rc};
use parsable::parsable;
use crate::{program::{ProgramContext, Type, VI, VariableInfo, VariableKind, Vasm}};
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
    pub fn process(&self, kind: VariableKind, context: &mut ProgramContext) -> Option<(Rc<VariableInfo>, Vasm)> {
        context.ckeck_var_unicity(&self.var_name);

        let mut source = vec![];
        let mut ok = false;
        let mut final_var_type = Type::Undefined;

        if let Some(vasm) = self.init_value.process(context) {
            if !vasm.ty.is_assignable() {
                context.errors.add(&self.init_value, format!("cannot assign type `{}`", &vasm.ty));
            } else {
                match &self.var_type {
                    Some(parsed_type) => match parsed_type.process(context) {
                        Some(var_type) => {
                            final_var_type = var_type.clone();

                            if var_type.is_assignable_to(&vasm.ty) {
                                final_var_type = var_type;
                                ok = true;
                            } else {
                                context.errors.add(&self.init_value, format!("assignment: type `{}` does not match type `{}`", &vasm.ty, &var_type));
                            }
                        },
                        None => {}
                    },
                    None => {
                        if !vasm.ty.is_ambiguous() {
                            final_var_type = vasm.ty.clone();
                            ok = true;
                        } else {
                            context.errors.add(&self.init_value, format!("insufficient infered type `{}` (consider declaring the variable type explicitly)", &vasm.ty));
                        }
                    }
                };
            }

            source.push(vasm);
        }

        let var_info = VariableInfo::new(self.var_name.clone(), final_var_type.clone(), kind);

        context.push_var(&var_info);

        source.push(Vasm::new(Type::Undefined, vec![Rc::clone(&var_info)], vec![VI::set_from_stack(&var_info)]));

        match ok {
            true => Some((var_info, Vasm::merge(source))),
            false => None
        }
    }
}