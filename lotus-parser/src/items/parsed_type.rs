use parsable::parsable;
use crate::program::{BuiltinType, ProgramContext, Type};
use super::{Identifier, ParsedTypeSingle, ParsedTypeWithoutSuffix, TypeSuffix, TypeSuffixWrapper};

#[parsable]
pub struct ParsedType {
    pub parsed_type: ParsedTypeWithoutSuffix,
    pub suffix: Vec<TypeSuffixWrapper>
}

impl ParsedType {
    pub fn as_var_name(&self) -> Option<&Identifier> {
        match self.suffix.is_empty() {
            true => self.parsed_type.as_var_name(),
            false => None,
        }
    }

    pub fn is_option(&self) -> bool {
        match self.suffix.last() {
            Some(s) => s.value == TypeSuffix::Option,
            None => false,
        }
    }

    pub fn collected_instancied_type_names(&self, list: &mut Vec<Identifier>) {
        match self.suffix.last() {
            Some(type_suffix) => type_suffix.collected_instancied_type_names(list),
            None => self.parsed_type.collected_instancied_type_names(list),
        }
    }

    pub fn process(&self, check_interfaces: bool, context: &mut ProgramContext) -> Option<Type> {
        if let Some(mut final_type) = self.parsed_type.process(check_interfaces, context) {
            for suffix in &self.suffix {
                final_type = suffix.process(final_type, context);
            }

            Some(final_type)
        } else {
            None
        }
    }
}