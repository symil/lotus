use crate::{DataLocation, parse_error::ParseError, string_reader::StringReader, utils::get_type_name};

pub trait Parsable : Sized {
    fn parse(reader: &mut StringReader) -> Option<Self>;

    #[allow(unused_variables)]
    fn parse_with_separator(reader: &mut StringReader, separator: &'static str) -> Option<Self> {
        unimplemented!()
    }

    fn get_token_name() -> Option<String> {
        // get_type_name::<Self>()
        None
    }

    fn get_location(&self) -> &DataLocation {
        unimplemented!()
    }

    fn parse_string(reader: &mut StringReader) -> Result<Self, ParseError> {
        reader.eat_spaces();

        match Self::parse(reader) {
            Some(value) => match reader.is_finished() {
                true => Ok(value),
                false => {
                    reader.set_expected_token(Some("<EOF>".to_string()));
                    Err(reader.get_error())
                }
            },
            None => {
                reader.set_expected_token(Some(get_type_name::<Self>()));
                Err(reader.get_error())
            }
        }
    }
}