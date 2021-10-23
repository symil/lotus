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
    pub fn collect_type_identifiers(&self, list: &mut Vec<Identifier>) {
        for ty in &self.list {
            ty.collect_type_identifiers(list);
        }
    }

    pub fn process(&self, check_interfaces: bool, context: &mut ProgramContext) -> Vec<Type> {
        let mut type_list = vec![];

        for arg in &self.list {
            let arg_type = match arg.process(check_interfaces, context) {
                Some(ty) => ty,
                None => Type::Undefined,
            };

            type_list.push(arg_type);
        }

        type_list
    }
}