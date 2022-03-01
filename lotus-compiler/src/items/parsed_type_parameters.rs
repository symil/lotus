use std::{collections::HashSet, rc::Rc};
use indexmap::{IndexMap, IndexSet};
use parsable::parsable;
use crate::{program::{self, ParameterTypeInfo, InterfaceList, ProgramContext}, utils::Link};
use super::{Identifier, ParsedOpeningAngleBracket, ParsedClosingAngleBracket};

#[parsable]
pub struct ParsedTypeParameters {
    pub opening_bracket: ParsedOpeningAngleBracket,
    #[parsable(separator=",")]
    pub list: Vec<ParsedTypeParameter>,
    pub closing_bracket: Option<ParsedClosingAngleBracket>,
}

#[parsable]
pub struct ParsedTypeParameter {
    pub name: Identifier,
    #[parsable(prefix=":", separator="+", optional=true)]
    pub required_interfaces: Vec<Identifier>
}

impl ParsedTypeParameters {
    pub fn process(&self, context: &mut ProgramContext) -> IndexMap<String, Rc<ParameterTypeInfo>> {
        let mut result = IndexMap::new();

        for parameter in &self.list {
            let name = parameter.name.clone(); 
            let mut required_interfaces = vec![];

            for interface_name in &parameter.required_interfaces {
                context.add_interface_completion_area(interface_name);

                if let Some(interface) = context.interfaces.get_by_identifier(interface_name) {
                    required_interfaces.push(interface.clone());
                } else {
                    context.errors.generic(&parameter.name, format!("undefined interface `{}`", interface_name));
                }
            }

            let index = result.len();
            let wasm_pattern = format!("<{}>", name.as_str());
            let item = Rc::new(ParameterTypeInfo {
                name,
                index,
                required_interfaces: InterfaceList::new(required_interfaces),
                wasm_pattern
            });

            if result.insert(parameter.name.to_string(), item).is_some() {
                context.errors.generic(&parameter.name, format!("duplicate type parameter `{}`", &parameter.name));
            }
        }

        if self.closing_bracket.is_none() {
            context.errors.expected_token(self, ">");
        }

        result
    }
}