use parsable::parsable;
use colored::*;
use crate::{items::{process_field_access, process_function_call}, program::{AccessType, ProgramContext, Type, VI, VariableKind, Vasm}};
use super::{ArgumentList, FieldOrMethodAccess, FullType, Identifier, VarPrefix, VarPrefixWrapper};

#[parsable]
pub enum RootVarRef {
    Prefixed(VarPrefixWrapper, Option<Identifier>),
    Unprefixed(FullType, Option<ArgumentList>)
}

pub enum ValueOrType {
    Value(Vasm),
    Type(Type)
}

impl RootVarRef {
    pub fn has_side_effects(&self) -> bool {
        match self {
            RootVarRef::Prefixed(_, _) => false,
            RootVarRef::Unprefixed(_, args) => args.is_some(),
        }
    }

    pub fn process(&self, access_type: AccessType, context: &mut ProgramContext) -> Option<ValueOrType> {
        match self {
            RootVarRef::Prefixed(prefix, field_name_opt) => match prefix.process(context) {
                Some(prefix_vasm) => match field_name_opt {
                    Some(field_name) => match process_field_access(&prefix_vasm.ty, field_name, access_type, context) {
                        Some(field_vasm) => Some(ValueOrType::Value(Vasm::merge(vec![prefix_vasm, field_vasm]))),
                        None => None
                    },
                    None => Some(ValueOrType::Value(prefix_vasm))
                },
                None => None
            },
            RootVarRef::Unprefixed(full_type, args_opt) => match args_opt {
                Some(args) => match full_type.as_single_name() {
                    Some(name) => {
                        if let Some(function_blueprint) = context.functions.get_by_identifier(name) {
                            process_function_call(None, function_blueprint, &[], args, access_type, context).and_then(|vasm| {
                                Some(ValueOrType::Value(vasm))
                            })
                        } else {
                            context.errors.add(name, format!("undefined function `{}`", name));
                            None
                        }
                    },
                    None => {
                        if let Some(ty) = full_type.process(context) {
                            context.errors.add(full_type, format!("type `{}` is not a function", &ty));
                        }

                        None
                    },
                },
                None => match full_type.as_single_name() {
                    Some(name) => match context.get_var_info(name) {
                        Some(var_info) => match access_type {
                            AccessType::Get => Some(ValueOrType::Value(Vasm::new(var_info.ty.clone(), vec![], vec![VI::get(&var_info)]))),
                            AccessType::Set(_) => Some(ValueOrType::Value(Vasm::new(var_info.ty.clone(), vec![], vec![VI::set_from_stack(&var_info)]))),
                        },
                        None => {
                            context.errors.set_enabled(false);
                            let type_opt = full_type.process(context);
                            context.errors.set_enabled(true);

                            match type_opt {
                                Some(ty) => Some(ValueOrType::Type(ty)),
                                None => {
                                    context.errors.add(name, format!("undefined variable `{}`", name.as_str().bold()));
                                    None
                                },
                            }
                        },
                    },
                    None => match full_type.process(context) {
                        Some(ty) => Some(ValueOrType::Type(ty)),
                        None => None
                    }
                },
            },
        }
    }
}