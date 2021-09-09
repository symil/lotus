use parsable::parsable;
use crate::program::{ProgramContext, Type};
use super::{ItemType, TypeSuffix};

#[parsable]
pub struct FullType {
    pub item: ItemType,
    pub suffix: Vec<TypeSuffix>
}

impl FullType {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Type> {
        if let Some(mut final_type) = self.item.process(context) {
            for suffix in &self.suffix {
                final_type = match suffix {
                    TypeSuffix::Array => context.array_type(final_type)
                };
            }

            Some(final_type)
        } else {
            None
        }
    }
}