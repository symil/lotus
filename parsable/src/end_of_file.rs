use crate::Parsable;

pub struct EndOfFile;

impl Parsable for EndOfFile {
    fn parse_item(reader: &mut crate::StringReader) -> Option<Self> {
        match reader.is_finished() {
            true => Some(EndOfFile),
            false => None,
        }
    }

    fn get_item_name() -> String {
        "<EOF>".to_string()
    }
}