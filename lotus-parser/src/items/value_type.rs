use std::fmt::format;

use parsable::parsable;
use crate::program::{GenericTypeInfo, ProgramContext, THIS_VAR_NAME, Type, ActualTypeInfo};
use super::{TypeArguments, Identifier, TypeSuffix};

#[parsable]
pub struct ValueType {
    pub name: Identifier,
    pub arguments: TypeArguments
}

impl ValueType {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Type> {
        if (self.name.as_str() == THIS_VAR_NAME) {
            if let Some(generic_list) = self.arguments.process(context) {
                context.errors.add(&self.arguments, format!("`{}` type cannot have parameters", THIS_VAR_NAME));
            }

            if context.current_type.is_none() && context.current_interface.is_none() {
                context.errors.add(&self.arguments, format!("`{}` does not refer to anything in this context", THIS_VAR_NAME));
                None
            } else {
                Some(Type::This)
            }
        } else if let Some(type_id) = context.check_generic_name(self.name.as_str()) {
            if let Some(generic_list) = self.arguments.process(context) {
                context.errors.add(&self.arguments, format!("generic types cannot have parameters"));
            }

            Some(Type::Generic(GenericTypeInfo {
                name: self.name.to_string(),
                type_context: type_id
            }))
        } else {
            let parameter_list = self.arguments.process(context).unwrap_or_default();

            if let Some(type_blueprint) = context.types.get_by_name(&self.name) {
                if parameter_list.len() != type_blueprint.parameters.len() {
                    context.errors.add(&self.name, format!("type `{}`: expected {} parameters, got {}", &self.name, type_blueprint.parameters.len(), parameter_list.len()));
                    None
                } else {
                    for (i, (parameter, argument)) in type_blueprint.parameters.values().zip(parameter_list.iter()).enumerate() {
                        for interface_id in &parameter.required_interfaces {
                            if !argument.match_interface(*interface_id, context) {
                                let interface_name = &context.interfaces.get_by_id(*interface_id).unwrap().name;

                                context.errors.add(&self.arguments.list[i], format!("type `{}` does not implement interface `{}`", argument, interface_name));
                            }
                        }
                    }

                    Some(Type::Actual(ActualTypeInfo {
                        name: type_blueprint.name.clone(),
                        type_id: type_blueprint.type_id,
                        parameters: parameter_list,
                    }))
                }
            } else {
                context.errors.add(&self.name, format!("undefined type `{}`", &self.name));
                None
            }
        }
    }
}