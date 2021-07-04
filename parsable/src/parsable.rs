use crate::{parse_error::ParseError, string_reader::StringReader, utils::get_type_name};

pub trait Parsable : Sized {
    fn parse(reader: &mut StringReader) -> Option<Self>;

    #[allow(unused_variables)]
    fn parse_with_separator(reader: &mut StringReader, separator: &'static str) -> Option<Self> {
        unimplemented!()
    }

    fn get_token_name() -> String {
        get_type_name::<Self>()
    }

    fn parse_string(string: &str) -> Result<Self, ParseError> {
        let mut reader = StringReader::new(string);

        reader.eat_spaces();

        match Self::parse(&mut reader) {
            Some(value) => match reader.is_finished() {
                true => Ok(value),
                false => {
                    reader.set_expected_token("<EOF>".to_string());
                    Err(reader.get_error())
                }
            },
            None => {
                reader.set_expected_token(get_type_name::<Self>());
                Err(reader.get_error())
            }
        }
    }
}