use std::collections::HashMap;
use parsable::DataLocation;
use crate::{program::CursorInfo, utils::{is_valid_identifier, is_blank_string, contains_valid_identifier_character}};
use super::{CompletionArea, CompletionContent};

#[derive(Debug)]
pub struct CompletionProvider {
    pub cursor: Option<CursorInfo>,
    pub areas_under_cursor: Vec<CompletionArea>
}

impl CompletionProvider {
    pub fn new(cursor: &Option<CursorInfo>) -> Self {
        Self {
            cursor: cursor.clone(),
            areas_under_cursor: vec![],
        }
    }

    pub fn insert<F : FnOnce() -> CompletionContent>(&mut self, area_location: &DataLocation, make_content: F) {
        let (under_cursor, is_valid_identifier) = match &self.cursor {
            Some(cursor) => match cursor.file_path.as_str() == area_location.file.path.as_str() {
                true => match contains_valid_identifier_character(area_location.as_str()) || is_blank_string(area_location.as_str()) {
                    true => (cursor.index >= area_location.start && cursor.index <= area_location.end, false),
                    false => (cursor.index == area_location.end, true),
                },
                false => (false, false),
            },
            None => (false, false),
        };

        if under_cursor {
            let content = make_content();
            let location = match is_valid_identifier {
                true => area_location.get_end(),
                false => area_location.clone(),
            };

            self.areas_under_cursor.push(CompletionArea {
                location,
                content,
            });
        }
    }

    pub fn get(&self, file_path: &str, cursor_index: usize) -> Option<&CompletionArea> {
        if let Some(cursor) = &self.cursor {
            if file_path == cursor.file_path && cursor_index == cursor.index {
                return self.areas_under_cursor.last();
            }
        }

        None
    }
}