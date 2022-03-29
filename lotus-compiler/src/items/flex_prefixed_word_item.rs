use std::ops::Deref;
use parsable::{parsable, Parsable, ItemLocation};
use crate::program::ProgramContext;
use super::{FlexWordItem, unwrap_item};

#[derive(Debug)]
pub struct FlexPrefixedWordItem<P : Parsable, W : Parsable> {
    prefix: P,
    word: Option<FlexWordItem<W>>,
    location: ItemLocation,
}

impl<P : Parsable, W : Parsable> FlexPrefixedWordItem<P, W> {
    pub fn process(&self, context: &mut ProgramContext) -> Option<&W> {
        context.completion_provider.add_keyword_completion(self.prefix.location(), W::get_completion_suggestions());

        let word = unwrap_item(&self.word, self, context)?;

        word.process(context)
    }
}

impl<P : Parsable, W : Parsable> Parsable for FlexPrefixedWordItem<P, W> {
    fn parse_item(reader: &mut parsable::StringReader) -> Option<Self> {
        let start = reader.get_index();
        let prefix = <P as Parsable>::parse_item(reader)?;
        let word = <FlexWordItem<W> as Parsable>::parse_item(reader);
        let location = reader.get_item_location(start);

        Some(Self {
            prefix,
            word,
            location,
        })
    }

    fn get_item_name() -> String {
        <P as Parsable>::get_item_name()
    }

    fn location(&self) -> &ItemLocation {
        &self.location
    }
}

impl<P : Parsable, W : Parsable> Deref for FlexPrefixedWordItem<P, W> {
    type Target = ItemLocation;

    fn deref(&self) -> &Self::Target {
        self.location()
    }
}