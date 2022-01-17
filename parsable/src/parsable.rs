use crate::{ParseError, string_reader::{ParseOptions, StringReader}};

pub trait Parsable : Sized {
    fn parse_item(reader: &mut StringReader) -> Option<Self>;

    #[allow(unused_variables)]
    fn parse_item_without_consuming_spaces(reader: &mut StringReader) -> Option<Self> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    fn parse_item_with_separator(reader: &mut StringReader, separator: &'static str) -> Option<Self> {
        unimplemented!()
    }

    fn token_name() -> &'static str;

    fn parse(string: String, options: ParseOptions) -> Result<Self, ParseError> {
        let mut reader = StringReader::new(string, options);

        reader.eat_spaces();

        match Self::parse_item(&mut reader) {
            Some(value) => match reader.is_finished() {
                true => Ok(value),
                false => {
                    reader.set_expected_token("<EOF>");
                    Err(reader.get_error())
                }
            },
            None => {
                reader.set_expected_token(<Self as Parsable>::token_name());
                Err(reader.get_error())
            }
        }
    }
}