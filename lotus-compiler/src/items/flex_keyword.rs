use std::ops::Deref;
use parsable::{ItemLocation, Parsable};
use crate::program::ProgramContext;

#[derive(Debug)]
pub struct FlexKeyword<const KEYWORD: &'static str> {
    pub keyword: String,
    pub location: ItemLocation
}

impl<const KEYWORD: &'static str> Parsable for FlexKeyword<KEYWORD> {
    fn parse_item(reader: &mut parsable::StringReader) -> Option<Self> {
        let start = reader.get_index();

        match reader.read_regex(r#"[a-zA-Z_]+"#) {
            Some(substring) => Some(Self {
                keyword: substring.to_string(),
                location: reader.get_item_location(start),
            }),
            None => None,
        }
    }

    fn item_name() -> &'static str {
        KEYWORD
    }

    fn item_name_wrapper() -> &'static str {
        "\""
    }
}

impl<const KEYWORD: &'static str> Deref for FlexKeyword<KEYWORD> {
    type Target = ItemLocation;

    fn deref(&self) -> &Self::Target {
        &self.location
    }
}

impl<const KEYWORD: &'static str> FlexKeyword<KEYWORD> {
    pub fn process(&self, context: &mut ProgramContext) -> Option<&'static str> {
        context.completion_provider.add_keyword_completion(self, &[KEYWORD]);

        match self.keyword.as_str() == KEYWORD {
            true => Some(KEYWORD),
            false => None,
        }
    }
}