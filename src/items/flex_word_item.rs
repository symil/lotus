use std::ops::Deref;
use parsable::{Parsable, ItemLocation};
use crate::program::ProgramContext;
use super::Word;

#[derive(Debug)]
pub struct FlexWordItem<T : Parsable> {
    content: FlexWordItemContent<T>,
    location: ItemLocation
}

#[derive(Debug)]
pub enum FlexWordItemContent<T : Parsable> {
    Item(T),
    Word(Word)
}

impl<T : Parsable> FlexWordItem<T> {
    pub fn process(&self, context: &mut ProgramContext) -> Option<&T> {
        context.completion_provider.add_keyword_completion(self, <T as Parsable>::get_completion_suggestions());

        match &self.content {
            FlexWordItemContent::Item(item) => Some(item),
            FlexWordItemContent::Word(_) => {
                context.errors.expected_item::<T>(self);
                None
            },
        }
    }
}

impl<T : Parsable> Parsable for FlexWordItem<T> {
    fn parse_item(reader: &mut parsable::StringReader) -> Option<Self> {
        let start = reader.get_index();

        if let Some(item) = <T as Parsable>::parse_item(reader) {
            Some(Self {
                content: FlexWordItemContent::Item(item),
                location: reader.get_item_location(start),
            })
        } else if let Some(word) = <Word as Parsable>::parse_item(reader) {
            Some(Self {
                content: FlexWordItemContent::Word(word),
                location: reader.get_item_location(start),
            })
        } else {
            None
        }
    }

    fn get_item_name() -> String {
        <T as Parsable>::get_item_name()
    }

    fn location(&self) -> &ItemLocation {
        &self.location
    }
}

impl<T : Parsable> Deref for FlexWordItem<T> {
    type Target = ItemLocation;

    fn deref(&self) -> &Self::Target {
        self.location()
    }
}