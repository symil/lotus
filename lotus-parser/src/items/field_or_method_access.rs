use std::{cell::Ref, collections::HashMap};
use parsable::parsable;
use colored::*;
use crate::{program::{AccessType, FunctionBlueprint, GET_AS_PTR_METHOD_NAME, FieldKind, ProgramContext, SET_AS_PTR_METHOD_NAME, Type, VI, VariableKind, Vasm}, utils::Link, vasm};
use super::{ArgumentList, Identifier, VarPrefix};

#[parsable]
pub struct FieldOrMethodAccess {
    pub name: Identifier,
    pub arguments: Option<ArgumentList>
}

impl FieldOrMethodAccess {
    pub fn has_side_effects(&self) -> bool {
        self.arguments.is_some()
    }

    pub fn process(&self, parent_type: &Type, field_kind: FieldKind, access_type: AccessType, context: &mut ProgramContext) -> Option<Vasm> {
        match &self.arguments {
            Some(arguments) => process_method_call(parent_type, field_kind, &self.name, &[], arguments, access_type, context),
            None => process_field_access(parent_type, &self.name, access_type, context)
        }
    }
}

pub fn process_field_access(parent_type: &Type, field_name: &Identifier, access_type: AccessType, context: &mut ProgramContext) -> Option<Vasm> {
    let mut result = None;

    if let Some(field_details) = parent_type.get_field(field_name.as_str()) {
        let method_name = match access_type {
            AccessType::Get => GET_AS_PTR_METHOD_NAME,
            AccessType::Set(_) => SET_AS_PTR_METHOD_NAME,
        };

        result = Some(Vasm::new(field_details.ty.clone(), vec![], vec![
            VI::call_method(parent_type, parent_type.get_static_method(method_name).unwrap(), &[], vec![VI::int(field_details.offset)])
        ]));
    } else {
        context.errors.add(field_name, format!("type `{}` has no field `{}`", parent_type, field_name));
    }

    result
}

pub fn process_method_call(caller_type: &Type, field_kind: FieldKind, method_name: &Identifier, parameters: &[Type], arguments: &ArgumentList, access_type: AccessType, context: &mut ProgramContext) -> Option<Vasm> {
    let mut result = None;

    if let Some(function_blueprint) = caller_type.get_method(field_kind, method_name.as_str()) {
        result = process_function_call(Some(caller_type), function_blueprint, parameters, arguments, access_type, context);
    } else if !caller_type.is_undefined() {
        context.errors.add(method_name, format!("type `{}` has no {}method `{}`", caller_type, field_kind.get_qualifier(), method_name.as_str().bold()));
    }

    result
}

pub fn process_function_call(caller_type: Option<&Type>, function_wrapped: Link<FunctionBlueprint>, parameters: &[Type], arguments: &ArgumentList, access_type: AccessType, context: &mut ProgramContext) -> Option<Vasm> {
    if let AccessType::Set(set_location) = access_type  {
        context.errors.add(set_location, format!("cannot set result of a function call"));
        return None;
    }

    let mut result = Vasm::empty();

    let (function_name, return_type) = function_wrapped.with_ref(|function_unwrapped| {
        let expected_arg_count = function_unwrapped.arguments.len();

        if arguments.len() != expected_arg_count {
            let s = if expected_arg_count > 1 { "s" } else { "" };

            context.errors.add(arguments, format!("expected {} argument{}, got {}", expected_arg_count, s, arguments.as_vec().len()));
        } else {
            for (i, (expected_arg, arg_expr)) in function_unwrapped.arguments.iter().zip(arguments.as_vec().iter()).enumerate() {
                if let Some(arg_vasm) = arg_expr.process(context) {
                    let expected_type = expected_arg.ty.replace_generics(caller_type, parameters);

                    if arg_vasm.ty.is_assignable_to(&expected_type) {
                        result.extend(arg_vasm);
                    } else {
                        context.errors.add(arg_expr, format!("argument #{}: expected `{}`, got `{}`", i + 1, &expected_type, &arg_vasm.ty));
                    }
                }
            }
        }

        (
            function_unwrapped.name.to_string(),
            function_unwrapped.return_value.as_ref().and_then(|var_info| Some(var_info.ty.replace_generics(caller_type, parameters))).unwrap_or(Type::Undefined)
        )
    });

    let call_instruction = match caller_type {
        Some(ty) => VI::call_method(ty, function_wrapped.clone(), parameters, vasm![]),
        None => VI::call_function(function_wrapped.clone(), parameters, vasm![]),
    };

    result.extend(Vasm::new(return_type, vec![], vec![call_instruction]));

    Some(result)
}