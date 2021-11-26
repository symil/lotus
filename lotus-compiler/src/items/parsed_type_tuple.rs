use parsable::parsable;
use crate::program::{BuiltinType, ProgramContext, Type};
use super::{Identifier, ParsedType};

#[parsable]
pub struct ParsedTypeTuple {
    #[parsable(brackets="()",separator=",")]
    pub type_list: Vec<ParsedType>
}

impl ParsedTypeTuple {
    pub fn as_single_identifier(&self) -> Option<&Identifier> {
        match self.type_list.len() {
            1 => self.type_list.first().unwrap().as_single_identifier(),
            _ => None
        }
    }

    pub fn collected_instancied_type_names(&self, list: &mut Vec<Identifier>) {
        for parsed_type in &self.type_list {
            parsed_type.collected_instancied_type_names(list);
        }
    }

    pub fn process(&self, check_interfaces: bool, context: &mut ProgramContext) -> Option<Type> {
        let mut types = vec![];

        for parsed_type in &self.type_list {
            if let Some(ty) = parsed_type.process(check_interfaces, context) {
                types.push(ty);
            }
        }

        if types.len() != self.type_list.len() {
            return None;
        }

        match types.len() {
            0 => context.errors.add_generic_and_none(self, format!("invalid empty type")),
            1 => Some(types.remove(0)),
            2 => Some(context.get_builtin_type(BuiltinType::Pair, types)),
            _ => context.errors.add_generic_and_none(self, format!("tuples can contain 2 values for now")),
        }
    }
}