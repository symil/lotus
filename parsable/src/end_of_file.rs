use crate::Parsable;

pub struct EndOfFile;

impl Parsable for EndOfFile {
    fn parse_item(reader: &mut crate::StringReader) -> Option<Self> {
        match reader.is_finished() {
            true => Some(EndOfFile),
            false => None,
        }
    }

    fn token_name() -> &'static str {
        "EOF"
    }

    fn token_name_wrapper() -> &'static str {
        "<>"
    }
}