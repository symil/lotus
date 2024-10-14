use std::ops::Deref;
use parsable::{ItemLocation, Parsable};
use crate::program::ProgramContext;
use super::Word;

// Rust is not quite ready for a `&'static[&'static str]` as type parameter

// #[derive(Debug)]
// pub struct FlexKeywordAmong<const KEYWORD_LIST: &'static[&'static str]> {
//     pub keyword: Word,
// }

// impl<const KEYWORD_LIST: &'static[&'static str]> Parsable for FlexKeywordAmong<KEYWORD_LIST> {
//     fn parse_item(reader: &mut parsable::StringReader) -> Option<Self> {
//         match <Word as Parsable>::parse_item(reader) {
//             Some(keyword) => Some(Self { keyword }),
//             None => None,
//         }
//     }

//     fn get_item_name() -> String {
//         KEYWORD_LIST.iter().map(|keyword| format!("\"{}\"", keyword)).collect::<Vec<String>>().join(" | ")
//     }
// }

// impl<const KEYWORD_LIST: &'static[&'static str]> Deref for FlexKeywordAmong<KEYWORD_LIST> {
//     type Target = ItemLocation;

//     fn deref(&self) -> &Self::Target {
//         &self.keyword.location
//     }
// }

// impl<const KEYWORD_LIST: &'static[&'static str]> FlexKeywordAmong<KEYWORD_LIST> {
//     pub fn process(&self, context: &mut ProgramContext) -> Option<&'static str> {
//         context.completion_provider.add_keyword_completion(self, KEYWORD_LIST);

//         match KEYWORD_LIST.iter().find(|keyword| **keyword == self.keyword.as_str()) {
//             Some(keyword) => Some(*keyword),
//             None => {
//                 context.errors.keyword_mismatch(&self.keyword, KEYWORD_LIST);
//                 None
//             },
//         }
//     }
// }