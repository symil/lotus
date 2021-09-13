use std::fmt::format;

use parsable::parsable;
use crate::program::{ActualTypeInfo, ProgramContext, THIS_VAR_NAME, Type};
use super::{TypeArguments, Identifier, TypeSuffix};

#[parsable]
pub struct ValueType {
    pub name: Identifier,
    pub arguments: TypeArguments
}

impl ValueType {
    pub fn as_single_name(&self) -> Option<&Identifier> {
        match self.arguments.list.is_empty() {
            true => Some(&self.name),
            false => None
        }
    }

    pub fn process(&self, context: &mut ProgramContext) -> Option<Type> {
        let mut result = None;
        let mut associated = false;
        let mut parameter = false;
        let parameters = self.arguments.process(context);
        let has_parameters = parameters.is_some();

        if let Some(current_function) = context.current_function {
            if let Some(parameter_type) = current_function.borrow().parameters.get(self.name.as_str()) {
                parameter = true;
                result = Some(Type::Parameter(parameter_type.clone()));
            }
        }

        if result.is_none() {
            if let Some(current_interface) = context.current_interface {
                if let Some(associated_type) = current_interface.borrow().associated_types.get(self.name.as_str()) {
                    associated = true;
                    result = Some(Type::Associated(associated_type.clone()));
                }
            } else if let Some(current_type) = context.current_type {
                if let Some(parameter_type) = current_type.borrow().parameters.get(self.name.as_str()) {
                    parameter = true;
                    result = Some(Type::Parameter(parameter_type.clone()));
                } else if let Some(associated_type) = current_type.borrow().associated_types.get(self.name.as_str()) {
                    associated = true;
                    result = Some(associated_type.value.clone());
                }
            }
        }

        if result.is_none() {
            let parameter_list = parameters.unwrap_or_default();

            if let Some(type_blueprint) = context.types.get_by_name(&self.name) {
                let parameters = &type_blueprint.borrow().parameters;

                if parameter_list.len() != parameters.len() {
                    context.errors.add(&self.name, format!("type `{}`: expected {} parameters, got {}", &self.name, parameters.len(), parameter_list.len()));
                } else {
                    for (i, (parameter, argument)) in parameters.values().zip(parameter_list.iter()).enumerate() {
                        for interface_blueprint in &parameter.borrow().required_interfaces {
                            if !argument.match_interface(interface_blueprint) {
                                let interface_name = &interface_blueprint.borrow().name;

                                context.errors.add(&self.arguments.list[i], format!("type `{}` does not implement interface `{}`", argument, interface_name));
                            }
                        }
                    }

                    result = Some(Type::Actual(ActualTypeInfo {
                        parameters: parameter_list,
                        type_blueprint: type_blueprint.clone(),
                    }))
                }
            }
        }

        if has_parameters {
            if associated {
                context.errors.add(&self.arguments, format!("associated types do not take parameters"));
            } else if parameter {
                context.errors.add(&self.arguments, format!("parameter types do not take parameters"));
            }
        }

        if result.is_none() {
            context.errors.add(&self.name, format!("undefined type `{}`", &self.name));
        }

        result
    }
}