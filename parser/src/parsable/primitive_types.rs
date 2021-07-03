use std::str::FromStr;
use super::{Parsable, string_reader::StringReader};

const NUMBER_PATTERN : &'static str = r"\d+(\.\d*)?";

impl Parsable for f32 {
    fn parse(reader: &mut StringReader) -> Option<Self> {
        match reader.read_regex(NUMBER_PATTERN) {
            Some(string) => Some(f32::from_str(string).unwrap()),
            None => None
        }
    }
}