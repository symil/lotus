use crate::{parsable::Parsable, string_reader::StringReader, ItemLocation};

impl Parsable for () {
    fn parse_item(_reader: &mut StringReader) -> Option<Self> {
        Some(())
    }

    fn get_item_name() -> String {
        "()".to_string()
    }
}

impl<T : Parsable> Parsable for Box<T> {
    fn get_item_name() -> String {
        <T as Parsable>::get_item_name()
    }

    fn parse_item(reader: &mut StringReader) -> Option<Self> {
        match <T as Parsable>::parse_item(reader) {
            Some(value) => Some(Box::new(value)),
            None => None
        }
    }

    fn location(&self) -> &ItemLocation {
        Box::as_ref(self).location()
    }
}

impl<T : Parsable> Parsable for Option<T> {
    fn get_item_name() -> String {
        <T as Parsable>::get_item_name()
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

    fn get_item_name() -> String {
        <T as Parsable>::get_item_name()
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
    fn get_item_name() -> String {
        format!("({}, {})", <T as Parsable>::get_item_name(), <U as Parsable>::get_item_name())
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