use std::collections::HashMap;
use parsable::ItemLocation;
use crate::{program::{CursorLocation, Cursor}, utils::{is_valid_identifier, is_blank_string, contains_valid_identifier_character}};
use super::{CompletionItemGenerator, CompletionItem};

#[derive(Debug)]
pub struct CompletionItemProvider {
    pub cursor: Cursor,
    pub completion_item_generators: Vec<CompletionItemGenerator>
}

impl CompletionItemProvider {
    pub fn new(cursor: &Cursor) -> Self {
        Self {
            cursor: cursor.clone(),
            completion_item_generators: vec![],
        }
    }

    pub fn add_completion_generator<F : FnOnce() -> CompletionItemGenerator>(&mut self, area_location: &ItemLocation, make_item_generator: F) {
        let is_under_cursor = self.cursor.is_on_location(area_location);

        if !is_under_cursor {
            return;
        }

        let insert_at_end_of_location = !contains_valid_identifier_character(area_location.as_str()) && !is_blank_string(area_location.as_str());
        let location = match insert_at_end_of_location {
            true => area_location.get_end(),
            false => area_location.clone(),
        };

        if self.cursor.is_on_location(&location) {
            self.completion_item_generators.push(make_item_generator());
        }
    }

    pub fn get_completion_items(&self) -> Vec<CompletionItem> {
        match self.completion_item_generators.last() {
            Some(generator) => generator.generate(&self.cursor.location.as_ref().unwrap()),
            None => vec![],
        }
    }
}