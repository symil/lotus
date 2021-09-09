use std::collections::HashSet;

use indexmap::{IndexMap, IndexSet};
use parsable::parsable;
use crate::program::{self, ProgramContext};
use super::Identifier;

#[parsable]
pub struct TypeParameters {
    #[parsable(brackets="<>", separator=",", optional=true)]
    pub list: Vec<TypeParameter>
}

#[parsable]
pub struct TypeParameter {
    pub name: Identifier,
    #[parsable(prefix=":", separator="+", optional=true)]
    pub required_interfaces: Vec<Identifier>
}

impl TypeParameters {
    pub fn process(&self, context: &mut ProgramContext) -> IndexMap<String, program::TypeParameter> {
        let mut result = IndexMap::new();

        for parameter in &self.list {
            let mut interface_ids = vec![];

            for interface_name in &parameter.required_interfaces {
                if let Some(interface) = context.interfaces.get_by_name(interface_name) {
                    interface_ids.push(interface.interface_id);
                } else {
                    context.errors.add(&parameter.name, format!("undefined interface `{}`", interface_name));
                }
            }

            let item = program::TypeParameter {
                name: parameter.name.clone(),
                required_interfaces: interface_ids,
            };

            if result.insert(parameter.name.to_string(), item).is_some() {
                context.errors.add(&parameter.name, format!("duplicate type parameter `{}`", &parameter.name));
            }
        }

        result
    }
}