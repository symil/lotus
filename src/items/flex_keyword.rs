use std::ops::Deref;
use parsable::{ItemLocation, Parsable};
use crate::program::ProgramContext;
use super::Word;

#[derive(Debug)]
pub struct FlexKeyword<const KEYWORD: &'static str> {
    pub keyword: Word,
}

impl<const KEYWORD: &'static str> Parsable for FlexKeyword<KEYWORD> {
    fn parse_item(reader: &mut parsable::StringReader) -> Option<Self> {
        match <Word as Parsable>::parse_item(reader) {
            Some(keyword) => Some(Self { keyword }),
            None => None,
        }
    }

    fn get_item_name() -> String {
        format!("\"{}\"", KEYWORD)
    }
}

impl<const KEYWORD: &'static str> Deref for FlexKeyword<KEYWORD> {
    type Target = ItemLocation;

    fn deref(&self) -> &Self::Target {
        &self.keyword.location
    }
}

impl<const KEYWORD: &'static str> FlexKeyword<KEYWORD> {
    pub fn process(&self, context: &mut ProgramContext) -> Option<&'static str> {
        context.completion_provider.add_keyword_completion(self, &[KEYWORD]);

        match self.keyword.as_str() == KEYWORD {
            true => Some(KEYWORD),
            false => {
                context.errors.keyword_mismatch(&self.keyword, &[KEYWORD]);
                None
            },
        }
    }
}