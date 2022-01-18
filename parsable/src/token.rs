use std::ops::Deref;
use crate::{ItemLocation, Parsable};

#[derive(Debug)]
pub struct Token<const TOKEN: &'static str> {
    pub token: &'static str,
    pub location: ItemLocation
}

impl<const TOKEN: &'static str> Parsable for Token<TOKEN> {
    fn parse_item(reader: &mut crate::StringReader) -> Option<Self> {
        let start = reader.get_index();

        match reader.read_string(TOKEN) {
            Some(_) => Some(Self {
                token: TOKEN,
                location: reader.get_item_location(start),
            }),
            None => None,
        }
    }

    fn item_name() -> &'static str {
        TOKEN
    }

    fn item_name_wrapper() -> &'static str {
        "\""
    }
}

impl<const TOKEN: &'static str> Deref for Token<TOKEN> {
    type Target = ItemLocation;

    fn deref(&self) -> &Self::Target {
        &self.location
    }
}