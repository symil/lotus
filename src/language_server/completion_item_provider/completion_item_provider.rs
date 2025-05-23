use std::collections::HashMap;
use parsable::ItemLocation;
use crate::{program::{CursorLocation, Cursor, Type}, utils::{is_valid_identifier, is_blank_string, contains_valid_identifier_character}};
use super::{CompletionItemGenerator, CompletionItem, FieldCompletionDetails, KeywordCompletionDetails, FieldCompletionOptions};

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

    pub fn add_completion<F : FnOnce() -> CompletionItemGenerator>(&mut self, area_location: &ItemLocation, make_item_generator: F) {
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

    pub fn add_keyword_completion(&mut self, location: &ItemLocation, keywords: &[&'static str]) {
        self.add_completion(location, || {
            CompletionItemGenerator::Keyword(KeywordCompletionDetails {
                available_keywords: keywords.to_vec(),
            })
        })
    }

    pub fn add_field_completion(&mut self, location: &ItemLocation, parent_type: &Type, type_hint: Option<&Type>, options: Option<&FieldCompletionOptions>) {
        self.add_completion(location, || {
            CompletionItemGenerator::FieldOrMethod(FieldCompletionDetails {
                parent_type: parent_type.clone(),
                options: options.cloned().unwrap_or_default(),
                expected_type: type_hint.cloned(),
            })
        })
    }

    pub fn add_static_field_completion(&mut self, location: &ItemLocation, parent_type: &Type, type_hint: Option<&Type>, options: Option<&FieldCompletionOptions>) {
        self.add_completion(location, || {
            CompletionItemGenerator::StaticFieldOrMethod(FieldCompletionDetails {
                parent_type: parent_type.clone(),
                options: options.cloned().unwrap_or_default(),
                expected_type: type_hint.cloned(),
            })
        })
    }

    pub fn add_enum_variant_completion(&mut self, location: &ItemLocation, enum_type: &Type) {
        self.add_completion(location, || {
            CompletionItemGenerator::Enum(enum_type.clone())
        })
    }

    pub fn get_completion_items(&self) -> Vec<CompletionItem> {
        match self.completion_item_generators.last() {
            Some(generator) => generator.generate(&self.cursor.location.as_ref().unwrap()),
            None => vec![],
        }
    }
}

impl Default for CompletionItemProvider {
    fn default() -> Self {
        Self {
            cursor: Cursor::new(&None),
            completion_item_generators: vec![]
        }
    }
}