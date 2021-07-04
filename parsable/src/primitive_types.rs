use std::str::FromStr;

use crate::{parsable::Parsable, string_reader::StringReader, utils::get_type_name};

const NUMBER_PATTERN : &'static str = r"\d+(\.\d*)?";

impl Parsable for f64 {
    fn parse(reader: &mut StringReader) -> Option<Self> {
        match reader.read_regex(NUMBER_PATTERN) {
            Some(string) => Some(f64::from_str(string).unwrap()),
            None => None
        }
    }
}

impl Parsable for bool {
    fn parse(reader: &mut StringReader) -> Option<Self> {
        if let Some(_) = reader.read_string("true") {
            Some(true)
        } else if let Some(_) = reader.read_string("false") {
            Some(false)
        } else {
            None
        }
    }
}

impl<T : Parsable> Parsable for Box<T> {
    fn get_token_name() -> String {
        get_type_name::<T>()
    }

    fn parse(reader: &mut StringReader) -> Option<Self> {
        match <T as Parsable>::parse(reader) {
            Some(value) => Some(Box::new(value)),
            None => None
        }
    }
}

impl<T : Parsable> Parsable for Option<T> {
    fn get_token_name() -> String {
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

    fn get_token_name() -> String {
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
                    reader.set_expected_token(format!("{:?}", separator));
                    break;
                }
            }
        }

        Some(result)
    }
}

impl<T : Parsable, U : Parsable> Parsable for (T, U) {
    fn get_token_name() -> String {
        format!("({}, {})", get_type_name::<T>(), get_type_name::<U>())
    }

    fn parse(reader: &mut StringReader) -> Option<Self> {
        let start_index = reader.get_index();
        let first = match T::parse(reader) {
            Some(value) => value,
            None => {
                reader.set_expected_token(T::get_token_name());
                return None;
            }
        };
        let second = match U::parse(reader) {
            Some(value) => value,
            None => {
                reader.set_expected_token(U::get_token_name());
                reader.set_index(start_index);
                return None;
            }
        };

        Some((first, second))
    }
}