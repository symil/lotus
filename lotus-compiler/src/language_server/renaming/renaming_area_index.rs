use std::collections::HashMap;
use parsable::DataLocation;
use crate::language_server::is_invalid_location;
use super::RenamingArea;

#[derive(Debug)]
pub struct RenamingAreaIndex {
    pub areas: HashMap<DataLocation, RenamingArea>
}

impl RenamingAreaIndex {
    pub fn new() -> Self {
        Self {
            areas: HashMap::new(),
        }
    }

    pub fn create_area(&mut self, definition: &DataLocation) {
        if is_invalid_location(definition) || self.areas.get(definition).is_some() {
            // panic!("duplicate renaming area creation");
            return;
        }

        self.areas.insert(definition.clone(), RenamingArea::new(definition));
    }

    pub fn add_occurence(&mut self, definition: &DataLocation, occurence: &DataLocation) {
        if is_invalid_location(definition) || is_invalid_location(occurence) {
            return;
        }

        let area = self.areas.get_mut(definition).expect("undefined renaming area");

        area.add_occurence(occurence);
    }

    pub fn get_occurences(&self, file_path: &str, cursor_index: usize) -> Option<Vec<DataLocation>> {
        for area in self.areas.values() {
            if area.contains_cursor(file_path, cursor_index) {
                return Some(area.get_all_occurences());
            }
        }

        None
    }
}