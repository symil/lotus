use crate::{parsable::Parsable, string_reader::StringReader};

impl Parsable for () {
    fn parse_item(_reader: &mut StringReader) -> Option<Self> {
        Some(())
    }

    fn item_name() -> &'static str {
        "()"
    }
}

impl<T : Parsable> Parsable for Box<T> {
    fn item_name() -> &'static str {
        <T as Parsable>::item_name()
    }

    fn parse_item(reader: &mut StringReader) -> Option<Self> {
        match <T as Parsable>::parse_item(reader) {
            Some(value) => Some(Box::new(value)),
            None => None
        }
    }
}

impl<T : Parsable> Parsable for Option<T> {
    fn item_name() -> &'static str {
        <T as Parsable>::item_name()
    }

    fn parse_item(reader: &mut StringReader) -> Option<Self> {
        match <T as Parsable>::parse_item(reader) {
            Some(value) => Some(Some(value)),
            None => {
                reader.set_expected_item::<T>();
                None
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

    fn parse_item_without_consuming_spaces(reader: &mut StringReader) -> Option<Self> {
        let mut result = vec![];

        while let Some(value) = T::parse_item(reader) {
            result.push(value);
        }

        Some(result)
    }

    fn item_name() -> &'static str {
        <T as Parsable>::item_name()
    }

    fn parse_item_with_separator(reader: &mut StringReader, separator: &'static str) -> Option<Self> {
        let mut result = vec![];

        while let Some(value) = T::parse_item(reader) {
            result.push(value);
            reader.eat_spaces();

            match reader.read_string(separator) {
                Some(_) => reader.eat_spaces(),
                None => {
                    reader.set_expected_string(separator);
                    break;
                }
            }
        }

        Some(result)
    }
}

impl<T : Parsable, U : Parsable> Parsable for (T, U) {
    fn item_name() -> &'static str {
        // TODO
        <T as Parsable>::item_name()
        // const_format_args!("({}, {})", <T as Parsable>::token_name(), <U as Parsable>::token_name()).as_str().unwrap()
    }

    fn parse_item(reader: &mut StringReader) -> Option<Self> {
        let start_index = reader.get_index();
        let first = match T::parse_item(reader) {
            Some(value) => value,
            None => {
                reader.set_expected_item::<T>();
                return None;
            }
        };
        let second = match U::parse_item(reader) {
            Some(value) => value,
            None => {
                reader.set_expected_item::<U>();
                reader.set_index(start_index);
                return None;
            }
        };

        Some((first, second))
    }
}