use indexmap::IndexSet;
use parsable::parsable;

use crate::program::ProgramContext;

use super::Identifier;

#[parsable]
pub struct GenericList {
    #[parsable(brackets="<>", separator=",", optional=true)]
    list: Vec<Identifier>
}

impl GenericList {
    pub fn process_as_parameters(&self, context: &mut ProgramContext) -> IndexSet<String> {
        let mut set = IndexSet::new();

        for identifier in &self.list {
            if !set.insert(identifier.to_string()) {
                context.errors.add(identifier, format!("duplicate generic type parameter `{}`", identifier));
            }
        }

        set
    }
}