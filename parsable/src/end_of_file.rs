use crate::Parsable;

pub struct EndOfFile;

impl Parsable for EndOfFile {
    fn parse_item(reader: &mut crate::StringReader) -> Option<Self> {
        match reader.is_finished() {
            true => Some(EndOfFile),
            false => None,
        }
    }

    fn item_name() -> &'static str {
        "EOF"
    }

    fn item_name_wrapper() -> &'static str {
        "<>"
    }
}