use std::{collections::HashMap, rc::Rc};
use parsable::parsable;
use crate::{program::{ProgramContext, TUPLE_FIRST_ASSOCIATED_TYPE_NAME, TUPLE_FIRST_METHOD_NAME, TUPLE_SECOND_ASSOCIATED_TYPE_NAME, TUPLE_SECOND_METHOD_NAME, Type, VI, VariableInfo, VariableKind, Vasm}, vasm};
use super::{Expression, Identifier, ParsedType, VarDeclarationQualifier};

#[parsable]
pub struct VarDeclaration {
    pub qualifier: VarDeclarationQualifier,
    pub var_names: VariableNames,
    #[parsable(prefix=":")]
    pub var_type: Option<ParsedType>,
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
        let init_value = match &self.var_type {
            Some(parsed_type) => match parsed_type.process(true, context) {
                Some(var_type) => match self.init_value.process(Some(&var_type), context) {
                    Some(vasm) => match vasm.ty.is_assignable_to(&var_type) {
                        true => Some((var_type, vasm)),
                        false => context.errors.add_generic_and_none(&self.init_value, format!("expected `{}`, got `{}`", &var_type, &vasm.ty))
                    },
                    None => None
                },
                None => None
            },
            None => match self.init_value.process(None, context) {
                Some(vasm) => match vasm.ty.is_ambiguous() {
                    true => context.errors.add_generic_and_none(&self.init_value, format!("insufficient infered type `{}` (consider declaring the variable type explicitly)", &vasm.ty)),
                    false => Some((vasm.ty.clone(), vasm))
                },
                None => None
            }
        };

        let current_function_level = Some(context.get_function_level());
        let (final_var_type, vasm) = match init_value {
            Some((ty, vasm)) => (ty, vasm),
            None => (Type::Undefined, vasm![]),
        };

        match &self.var_names {
            VariableNames::Single(name) => {
                let var_info = context.declare_local_variable(name.clone(), final_var_type.clone());

                Some((
                    vec![var_info.clone()],
                    Vasm::new(Type::Void, vec![var_info.clone()], vec![
                        VI::init_var(&var_info),
                        VI::set_var(&var_info, current_function_level, vasm)
                    ])
                ))
            },
            VariableNames::Multiple(names) => {
                if names.len() != 2 {
                    context.errors.add_generic_and_none(&self.init_value, format!("tuples can only be declared as pairs"))
                } else {
                    match (final_var_type.get_associated_type(TUPLE_FIRST_ASSOCIATED_TYPE_NAME), final_var_type.get_associated_type(TUPLE_SECOND_ASSOCIATED_TYPE_NAME)) {
                        (Some(first_type), Some(second_type)) => {
                            let tmp_var_info = VariableInfo::tmp("tmp", final_var_type.clone());
                            let var_1 = context.declare_local_variable(names[0].clone(), first_type);
                            let var_2 = context.declare_local_variable(names[1].clone(), second_type);

                            Some((
                                vec![var_1.clone(), var_2.clone()],
                                Vasm::new(Type::Void, vec![tmp_var_info.clone(), var_1.clone(), var_2.clone()], vasm![
                                    vasm,
                                    VI::set_tmp_var(&tmp_var_info),
                                    VI::init_var(&var_1),
                                    VI::init_var(&var_2),
                                    VI::set_var(&var_1, current_function_level, vec![
                                        VI::get_tmp_var(&tmp_var_info),
                                        VI::call_regular_method(&final_var_type, TUPLE_FIRST_METHOD_NAME, &[], vec![], context)
                                    ]),
                                    VI::set_var(&var_2, current_function_level, vec![
                                        VI::get_tmp_var(&tmp_var_info),
                                        VI::call_regular_method(&final_var_type, TUPLE_SECOND_METHOD_NAME, &[], vec![], context)
                                    ])
                                ])
                            ))
                        },
                        _ => {
                            if !final_var_type.is_undefined() {
                                context.errors.add_generic(&self.init_value, format!("cannot destructure type `{}` into 2 values", &final_var_type));
                            }

                            None
                        }
                    }
                }
            },
        }
    }
}