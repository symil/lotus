use crate::{parsable::Parsable, string_reader::StringReader};

impl Parsable for () {
    fn parse_item(_reader: &mut StringReader) -> Option<Self> {
        Some(())
    }
}

impl<T : Parsable> Parsable for Box<T> {
    fn get_token_name() -> Option<String> {
        <T as Parsable>::get_token_name()
    }

    fn parse_item(reader: &mut StringReader) -> Option<Self> {
        match <T as Parsable>::parse_item(reader) {
            Some(value) => Some(Box::new(value)),
            None => None
        }
    }
}

impl<T : Parsable> Parsable for Option<T> {
    fn get_token_name() -> Option<String> {
        <T as Parsable>::get_token_name()
    }

    fn parse_item(reader: &mut StringReader) -> Option<Self> {
        match <T as Parsable>::parse_item(reader) {
            Some(value) => Some(Some(value)),
            None => {
                reader.set_expected_token(<T as Parsable>::get_token_name());
                Some(None)
            }
        }
    }
}

impl<T : Parsable> Parsable for Vec<T> {
    fn parse_item(reader: &mut StringReader) -> Option<Self> {
        let mut result = vec![];

        while let Some(value) = T::parse_item(reader) {
            result.push(value);
            reader.eat_spaces();
        }

        Some(result)
    }

    fn get_token_name() -> Option<String> {
        <T as Parsable>::get_token_name()
    }

    fn parse_item_with_separator(reader: &mut StringReader, separator: &'static str) -> Option<Self> {
        let mut result = vec![];

        while let Some(value) = T::parse_item(reader) {
            result.push(value);
            reader.eat_spaces();

            match reader.read_string(separator) {
                Some(_) => reader.eat_spaces(),
                None => {
                    reader.set_expected_token(Some(format!("{:?}", separator)));
                    break;
                }
            }
        }

        Some(result)
    }
}

impl<T : Parsable, U : Parsable> Parsable for (T, U) {
    fn get_token_name() -> Option<String> {
        let first = <T as Parsable>::get_token_name()?;
        let second = <U as Parsable>::get_token_name()?;

        Some(format!("({}, {})", first, second))
    }

    fn parse_item(reader: &mut StringReader) -> Option<Self> {
        let start_index = reader.get_index();
        let first = match T::parse_item(reader) {
            Some(value) => value,
            None => {
                reader.set_expected_token(T::get_token_name());
                return None;
            }
        };
        let second = match U::parse_item(reader) {
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