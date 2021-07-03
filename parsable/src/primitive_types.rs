use std::str::FromStr;

use crate::{parsable::Parsable, string_reader::StringReader, utils::get_type_name};

const NUMBER_PATTERN : &'static str = r"\d+(\.\d*)?";

impl Parsable for f32 {
    fn parse(reader: &mut StringReader) -> Option<Self> {
        match reader.read_regex(NUMBER_PATTERN) {
            Some(string) => Some(f32::from_str(string).unwrap()),
            None => None
        }
    }
}

impl<T : Parsable> Parsable for Option<T> {
    fn get_token_name() -> &'static str {
        get_type_name::<T>()
    }

    fn parse(reader: &mut StringReader) -> Option<Self> {
        match <T as Parsable>::parse(reader) {
            Some(value) => Some(Some(value)),
            None => {
                reader.set_expected_token(<T as Parsable>::get_token_name());
                Some(None)
            }
        }
    }
}

impl<T : Parsable> Parsable for Vec<T> {
    fn parse(reader: &mut StringReader) -> Option<Self> {
        let mut result = vec![];

        while let Some(value) = T::parse(reader) {
            result.push(value);
            reader.eat_spaces();
        }

        Some(result)
    }

    fn get_token_name() -> &'static str {
        get_type_name::<T>()
    }

    fn parse_with_separator(reader: &mut StringReader, separator: &'static str) -> Option<Self> {
        let mut result = vec![];

        while let Some(value) = T::parse(reader) {
            result.push(value);
            reader.eat_spaces();

            match reader.read_string(separator) {
                Some(_) => reader.eat_spaces(),
                None => {
                    reader.set_expected_token(&format!("{:?}", separator));
                    break;
                }
            }
        }

        Some(result)
    }
}