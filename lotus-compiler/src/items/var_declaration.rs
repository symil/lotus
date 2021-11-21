use std::{collections::HashMap, rc::Rc};
use parsable::parsable;
use crate::{program::{ProgramContext, TUPLE_FIRST_ASSOCIATED_TYPE_NAME, TUPLE_FIRST_METHOD_NAME, TUPLE_SECOND_ASSOCIATED_TYPE_NAME, TUPLE_SECOND_METHOD_NAME, Type, VI, VariableInfo, VariableKind, Vasm}};
use super::{Expression, Identifier, ParsedType, ParsedTypeWrapper, VarDeclarationQualifier};

#[parsable]
pub struct VarDeclaration {
    pub qualifier: VarDeclarationQualifier,
    pub var_names: VariableNames,
    #[parsable(prefix=":")]
    pub var_type: Option<ParsedTypeWrapper>,
    #[parsable(prefix="=")]
    pub init_value: Expression,
}

#[parsable]
pub enum VariableNames {
    Single(Identifier),
    #[parsable(brackets="()", separator=",")]
    Multiple(Vec<Identifier>)
}

impl VarDeclaration {
    pub fn get_first_var_name(&self) -> &Identifier {
        match &self.var_names {
            VariableNames::Single(name) => name,
            VariableNames::Multiple(names) => names.first().unwrap(),
        }
    }

    pub fn process(&self, context: &mut ProgramContext) -> Option<(Vec<VariableInfo>, Vasm)> {
        let kind = match context.get_current_function() {
            Some(_) => VariableKind::Local,
            None => VariableKind::Global,
        };

        match &self.var_names {
            VariableNames::Single(name) => {
                context.check_var_unicity(name);
            },
            VariableNames::Multiple(names) => {
                for name in names {
                    context.check_var_unicity(name);
                }
            },
        };

        let mut source = vec![];
        let mut ok = false;
        let mut final_var_type = Type::Undefined;
        let mut declared_variables = vec![];

        match &self.var_type {
            Some(parsed_type) => match parsed_type.process(true, context) {
                Some(var_type) => {
                    final_var_type = var_type.clone();

                    if let Some(vasm) = self.init_value.process(Some(&var_type), context) {
                        if vasm.ty.is_assignable_to(&var_type) {
                            final_var_type = var_type;
                            ok = true;
                        } else {
                            context.errors.add(&self.init_value, format!("expected `{}`, got `{}`", &var_type, &vasm.ty));
                        }

                        source.push(vasm);
                    }
                },
                None => {}
            },
            None => {
                if let Some(vasm) = self.init_value.process(None, context) {
                    if !vasm.ty.is_ambiguous() {
                        final_var_type = vasm.ty.clone();
                        ok = true;
                    } else {
                        context.errors.add(&self.init_value, format!("insufficient infered type `{}` (consider declaring the variable type explicitly)", &vasm.ty));
                    }

                    source.push(vasm);
                }
            }
        };

        match &self.var_names {
            VariableNames::Single(name) => {
                let var_info = VariableInfo::from(name.clone(), final_var_type.clone(), kind);

                source.push(Vasm::new(Type::Undefined, vec![var_info.clone()], vec![VI::set_var_from_stack(&var_info)]));
                declared_variables.push(var_info);
            },
            VariableNames::Multiple(names) => {
                if names.len() != 2 {
                    context.errors.add(&self.init_value, format!("tuples can only be declared as pairs"));
                } else {
                    match (final_var_type.get_associated_type(TUPLE_FIRST_ASSOCIATED_TYPE_NAME), final_var_type.get_associated_type(TUPLE_SECOND_ASSOCIATED_TYPE_NAME)) {
                        (Some(first_type), Some(second_type)) => {
                            let tmp_var_info = VariableInfo::from(Identifier::unique("tmp", self), final_var_type.clone(), VariableKind::Local);
                            let var_1 = VariableInfo::from(names[0].clone(), first_type, VariableKind::Local);
                            let var_2 = VariableInfo::from(names[1].clone(), second_type, VariableKind::Local);

                            declared_variables.extend(vec![var_1.clone(), var_2.clone()]);
                            source.push(Vasm::new(Type::Undefined, vec![tmp_var_info.clone(), var_1.clone(), var_2.clone()], vec![
                                VI::set_var_from_stack(&tmp_var_info),
                                VI::get_var(&tmp_var_info),
                                VI::call_regular_method(&final_var_type, TUPLE_FIRST_METHOD_NAME, &[], vec![], context),
                                VI::set_var_from_stack(&var_1),
                                VI::get_var(&tmp_var_info),
                                VI::call_regular_method(&final_var_type, TUPLE_SECOND_METHOD_NAME, &[], vec![], context),
                                VI::set_var_from_stack(&var_2),
                            ]));
                        },
                        _ => {
                            if !final_var_type.is_undefined() {
                                context.errors.add(&self.init_value, format!("cannot destructure type `{}` into 2 values", &final_var_type));
                            }
                        }
                    }
                }
            },
        };

        for var_info in &declared_variables {
            context.push_var(&var_info);
        }

        match ok {
            true => Some((declared_variables, Vasm::merge_with_type(Type::Void, source))),
            false => None
        }
    }
}