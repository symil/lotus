use parsable::parsable;
use crate::program::{BuiltinType, ProgramContext, Type};
use super::{Identifier, ItemType, TypeSuffix, TypeSuffixWrapper};

#[parsable]
pub struct FullType {
    pub item: ItemType,
    pub suffix: Vec<TypeSuffixWrapper>
}

impl FullType {
    pub fn as_var_name(&self) -> Option<&Identifier> {
        match self.suffix.iter().all(|suffix| suffix.value == TypeSuffix::Option) {
            true => self.item.as_var_name(),
            false => None
        }
    }

    pub fn collect_type_identifiers(&self, list: &mut Vec<Identifier>) {
        self.item.collect_type_identifiers(list);
    }

    pub fn process(&self, check_interfaces: bool, context: &mut ProgramContext) -> Option<Type> {
        if let Some(mut final_type) = self.item.process(check_interfaces, context) {
            for suffix in &self.suffix {
                final_type = suffix.process(final_type, context);
            }

            Some(final_type)
        } else {
            None
        }
    }
}