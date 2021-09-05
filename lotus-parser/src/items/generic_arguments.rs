use indexmap::IndexSet;
use parsable::parsable;
use crate::program::{ProgramContext, Type};
use super::{FullType, Identifier};

#[parsable]
pub struct GenericArguments {
    #[parsable(brackets="<>", separator=",", optional=true)]
    list: Vec<FullType>
}

impl GenericArguments {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Vec<Type>> {
        let mut type_list = vec![];

        for arg in &self.list {
            if let Some(ty) = arg.process(context) {
                type_list.push(ty);
            }
        }

        match type_list.len() == self.list.len() {
            true => Some(type_list),
            false => None
        }
    }
}