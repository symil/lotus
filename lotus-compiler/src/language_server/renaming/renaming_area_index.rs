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

    pub fn add_occurence(&mut self, occurence: &DataLocation, definition: &DataLocation) {
        if is_invalid_location(definition) || is_invalid_location(occurence) {
            return;
        }

        self.areas.entry(definition.clone())
            .or_insert(RenamingArea::new(definition))
            .add_occurence(occurence);
    }

    pub fn get_occurence_under_cursor(&self, root_directory_path: &str, file_path: &str, cursor_index: usize) -> Option<DataLocation> {
        for area in self.areas.values() {
            if let Some(location) = area.get_occurence_under_cursor(root_directory_path, file_path, cursor_index) {
                return Some(location);
            }
        }

        None
    }

    pub fn get_all_occurences(&self, root_directory_path: &str, file_path: &str, cursor_index: usize) -> Option<Vec<DataLocation>> {
        for area in self.areas.values() {
            if area.get_occurence_under_cursor(root_directory_path, file_path, cursor_index).is_some() {
                return Some(area.get_all_occurences());
            }
        }

        None
    }
}