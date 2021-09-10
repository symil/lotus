use std::fmt::format;

use parsable::parsable;
use crate::program::{ActualTypeInfo, AssociatedTypeInfo, GenericTypeInfo, ProgramContext, THIS_VAR_NAME, Type};
use super::{TypeArguments, Identifier, TypeSuffix};

#[parsable]
pub struct ValueType {
    pub name: Identifier,
    pub arguments: TypeArguments
}

impl ValueType {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Type> {
        let mut result = None;
        let mut associated = false;
        let mut parameter = false;
        let parameters = self.arguments.process(context);
        let has_parameters = parameters.is_some();

        if let Some(interface_id) = context.current_interface {
            let interface_blueprint = context.interfaces.get_by_id(interface_id).unwrap();
            
            if let Some(associated_type) = interface_blueprint.associated_types.get(self.name.as_str()) {
                associated = true;
                result = Some(Type::Associated(AssociatedTypeInfo {
                    name: self.name.to_string(),
                    interface_context: interface_id,
                }));

                if let Some(generic_list) = self.arguments.process(context) {
                    context.errors.add(&self.arguments, format!("associated types do not have parameters"));
                }
            }
        } else if let Some(type_id) = context.current_type {
            let type_blueprint = context.types.get_by_id(type_id).unwrap();

            if let Some(parameter_type) = type_blueprint.parameters.get(self.name.as_str()) {
                parameter = true;
                result = Some(Type::Generic(GenericTypeInfo {
                    name: self.name.to_string(),
                    type_context: type_id
                }));
            } else if let Some(associated_type) = type_blueprint.associated_types.get(self.name.as_str()) {
                associated = true;
                result = Some(associated_type.value.clone());
            }
        }

        if result.is_none() {
            let parameter_list = parameters.unwrap_or_default();

            if let Some(type_blueprint) = context.types.get_by_name(&self.name) {
                if parameter_list.len() != type_blueprint.parameters.len() {
                    context.errors.add(&self.name, format!("type `{}`: expected {} parameters, got {}", &self.name, type_blueprint.parameters.len(), parameter_list.len()));
                } else {
                    for (i, (parameter, argument)) in type_blueprint.parameters.values().zip(parameter_list.iter()).enumerate() {
                        for interface_id in &parameter.required_interfaces {
                            if !argument.match_interface(*interface_id, context) {
                                let interface_name = &context.interfaces.get_by_id(*interface_id).unwrap().name;

                                context.errors.add(&self.arguments.list[i], format!("type `{}` does not implement interface `{}`", argument, interface_name));
                            }
                        }
                    }

                    result = Some(Type::Actual(ActualTypeInfo {
                        name: type_blueprint.name.clone(),
                        type_id: type_blueprint.type_id,
                        parameters: parameter_list,
                    }))
                }
            }
        }

        if has_parameters {
            if associated {
                context.errors.add(&self.arguments, format!("associated types do not take parameters"));
            } else if parameter {
                context.errors.add(&self.arguments, format!("associated types do not take parameters"));
            }
        }

        if result.is_none() {
            context.errors.add(&self.name, format!("undefined type `{}`", &self.name));
        }

        result
    }
}