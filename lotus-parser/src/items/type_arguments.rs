use indexmap::IndexSet;
use parsable::parsable;
use crate::program::{ProgramContext, Type};
use super::{FullType, Identifier};

#[parsable]
pub struct TypeArguments {
    #[parsable(brackets="<>", separator=",", optional=true)]
    pub list: Vec<FullType>
}

impl TypeArguments {
    pub fn process(&self, context: &mut ProgramContext) -> Vec<Type> {
        let mut type_list = vec![];

        for arg in &self.list {
            let arg_type = match arg.process(context) {
                Some(ty) => ty,
                None => Type::Undefined,
            };

            type_list.push(arg_type);
        }

        type_list
    }
}