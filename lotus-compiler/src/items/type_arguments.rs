use indexmap::IndexSet;
use parsable::parsable;
use crate::program::{ProgramContext, Type};
use super::{ParsedType, Identifier};

#[parsable]
#[derive(Default)]
pub struct TypeArguments {
    #[parsable(brackets="<>", separator=",", optional=true)]
    pub list: Vec<ParsedType>
}

impl TypeArguments {
    pub fn collected_instancied_type_names(&self, list: &mut Vec<Identifier>, context: &mut ProgramContext) {
        for ty in &self.list {
            ty.collected_instancied_type_names(list, context);
        }
    }

    pub fn process(&self, check_interfaces: bool, context: &mut ProgramContext) -> Vec<Type> {
        let mut type_list = vec![];

        for arg in &self.list {
            let arg_type = match arg.process(check_interfaces, context) {
                Some(ty) => ty,
                None => Type::undefined(),
            };

            type_list.push(arg_type);
        }

        type_list
    }
}