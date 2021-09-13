use std::{cell::Ref, collections::HashMap};

use parsable::parsable;
use crate::{program::{AccessType, FunctionBlueprint, GET_AS_PTR_METHOD_NAME, ProgramContext, SET_AS_PTR_METHOD_NAME, Type, VI, VariableKind, Vasm}, utils::Link};
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

    pub fn process(&self, parent_type: &Type, access_type: AccessType, context: &mut ProgramContext) -> Option<Vasm> {
        match &self.arguments {
            Some(arguments) => process_method_call(parent_type, &self.name, arguments, access_type, context),
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
            VI::call_function(parent_type.get_static_method(method_name).unwrap(), vec![VI::int(field_details.offset)])
        ]));
    } else {
        context.errors.add(field_name, format!("type `{}` has no field `{}`", parent_type, field_name));
    }

    result
}

pub fn process_method_call(parent_type: &Type, method_name: &Identifier, arguments: &ArgumentList, access_type: AccessType, context: &mut ProgramContext) -> Option<Vasm> {
    let mut result = None;

    if let Some(function_blueprint) = parent_type.get_method(method_name.as_str()) {
        result = process_function_call(function_blueprint, arguments, access_type, context);
    } else {
        context.errors.add(method_name, format!("type `{}` has no method `{}`", parent_type, method_name));
    }

    result
}

pub fn process_function_call(function_blueprint: &Link<FunctionBlueprint>, arguments: &ArgumentList, access_type: AccessType, context: &mut ProgramContext) -> Option<Vasm> {
    if let AccessType::Set(set_location) = access_type  {
        context.errors.add(set_location, format!("cannot set result of a function call"));
        return None;
    }

    let function_content = function_blueprint.borrow();
    let expected_arg_count = function_content.arguments.len();
    let mut result = Vasm::empty();

    if arguments.len() != expected_arg_count {
        let s = if expected_arg_count > 1 { "s" } else { "" };

        context.errors.add(arguments, format!("expected {} argument{}, got {}", expected_arg_count, s, arguments.as_vec().len()));
    }

    for (i, arg_expr) in arguments.as_vec().iter().enumerate() {
        if let Some(arg_vasm) = arg_expr.process(context) {
            if i < expected_arg_count {
                let expected_type = &function_content.arguments[0].ty;

                if expected_type.is_assignable_to(&arg_vasm.ty) {
                    result.extend(arg_vasm);
                } else {
                    context.errors.add(arg_expr, format!("argument #{}: expected `{}`, got `{}`", i + 1, expected_type, &arg_vasm.ty));
                }
            }
        }
    }

    let return_type = function_content.return_value.and_then(|var_info| Some(var_info.ty.clone())).unwrap_or(Type::Void);

    result.extend(Vasm::new(return_type, vec![], vec![VI::call_function_from_stack(function_blueprint)]));

    Some(result)
}