use parsable::parsable;
use crate::program::{BuiltinType, ProgramContext, Type};
use super::{Identifier, ParsedTypeSingle, ParsedTypeWithoutSuffix, ParsedTypeSuffixToken, ParsedTypeSuffix};

#[parsable(name = "type")]
#[derive(Default)]
pub struct ParsedType {
    pub parsed_type: ParsedTypeWithoutSuffix,
    pub suffix: Vec<ParsedTypeSuffix>
}

impl ParsedType {
    pub fn as_single_identifier(&self) -> Option<&Identifier> {
        match self.suffix.is_empty() {
            true => self.parsed_type.as_single_identifier(),
            false => None,
        }
    }

    pub fn is_option(&self) -> bool {
        match self.suffix.last() {
            Some(s) => s.token == ParsedTypeSuffixToken::Option,
            None => false,
        }
    }

    pub fn collect_instancied_type_names(&self, list: &mut Vec<String>, context: &mut ProgramContext) {
        match self.suffix.last() {
            Some(type_suffix) => type_suffix.collect_instancied_type_names(list),
            None => self.parsed_type.collect_instancied_type_names(list, context),
        }
    }

    pub fn process(&self, check_interfaces: bool, type_hint: Option<&Type>, context: &mut ProgramContext) -> Option<Type> {
        if let Some(mut final_type) = self.parsed_type.process(check_interfaces, type_hint, context) {
            for suffix in &self.suffix {
                final_type = suffix.process(final_type, context);
            }

            Some(final_type)
        } else {
            None
        }
    }
}