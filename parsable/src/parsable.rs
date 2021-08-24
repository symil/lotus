use crate::{ParseError, string_reader::{ParseOptions, StringReader}, utils::get_type_name};

pub trait Parsable : Sized {
    fn parse_item(reader: &mut StringReader) -> Option<Self>;

    fn parse_item_all(reader: &mut StringReader) -> Vec<(Self, usize)> {
        match Self::parse_item(reader) {
            Some(item) => vec![(item, reader.get_index())],
            None => vec![]
        }
    }

    #[allow(unused_variables)]
    fn parse_item_with_separator(reader: &mut StringReader, separator: &'static str) -> Option<Self> {
        unimplemented!()
    }

    fn get_token_name() -> Option<String> {
        None
    }

    fn parse(string: String, options: ParseOptions) -> Result<Self, ParseError> {
        let mut reader = StringReader::new(string, options);

        reader.eat_spaces();

        match Self::parse_item(&mut reader) {
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