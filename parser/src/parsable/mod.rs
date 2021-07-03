use self::{parse_error::ParseError, string_reader::StringReader};

pub mod string_reader;
pub mod primitive_types;
pub mod parse_error;

pub trait Parsable : Sized {
    fn parse(reader: &mut StringReader) -> Option<Self>;

    fn parse_string(string: &str) -> Result<Self, ParseError> {
        let mut reader = StringReader::new(string);

        reader.eat_spaces();

        match Self::parse(&mut reader) {
            Some(value) => Ok(value),
            None => {
                reader.set_error::<Self>();
                Err(reader.get_error().unwrap())
            }
        }
    }
}