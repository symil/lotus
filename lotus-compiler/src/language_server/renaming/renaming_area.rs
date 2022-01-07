use std::{collections::HashSet, iter::FromIterator};
use parsable::DataLocation;

#[derive(Debug)]
pub struct RenamingArea {
    pub definition: DataLocation,
    pub occurences: HashSet<DataLocation>
}

impl RenamingArea {
    pub fn new(definition: &DataLocation) -> Self {
        Self {
            definition: definition.clone(),
            occurences: HashSet::from_iter(vec![definition.clone()]),
        }
    }

    pub fn add_occurence(&mut self, occurence: &DataLocation) {
        self.occurences.insert(occurence.clone());
    }

    pub fn get_occurence_under_cursor(&self, root_directory_path: &str, file_path: &str, cursor_index: usize) -> Option<DataLocation> {
        if self.definition.file.package_root_path != root_directory_path {
            return None;
        }

        for location in &self.occurences {
            if location.contains_cursor(file_path, cursor_index) {
                return Some(location.clone());
            }
        }

        None
    }

    pub fn get_all_occurences(&self) -> Vec<DataLocation> {
        self.occurences.iter().map(|location| location.clone()).collect()
    }
}