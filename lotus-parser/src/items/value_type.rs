use std::fmt::format;

use parsable::parsable;
use crate::program::{GenericInfo, ProgramContext, Type, TypeRef};
use super::{GenericArguments, Identifier, TypeSuffix};

#[parsable]
pub struct ValueType {
    pub name: Identifier,
    pub parameters: GenericArguments
}

impl ValueType {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Type> {
        if let Some(type_id) = context.check_generic_name(self.name.as_str()) {
            if let Some(generic_list) = self.parameters.process(context) {
                context.errors.add(&self.parameters, format!("generic types cannot have parameters"));
            }

            Some(Type::Generic(GenericInfo {
                name: self.name.to_string(),
                type_context: type_id
            }))
        } else {
            let generic_list = self.parameters.process(context).unwrap_or_default();

            if let Some(type_blueprint) = context.types.get_by_name(&self.name) {
                if generic_list.len() != type_blueprint.generics.len() {
                    context.errors.add(&self.name, format!("type `{}`: expected {} parameters, got {}", &self.name, type_blueprint.generics.len(), generic_list.len()));
                    None
                } else {
                    Some(Type::Actual(TypeRef {
                        type_id: type_blueprint.type_id,
                        type_context: context.current_type,
                        generic_values: generic_list,
                    }))
                }
            } else {
                context.errors.add(&self.name, format!("undefined type `{}`", &self.name));
                None
            }
        }
    }
}