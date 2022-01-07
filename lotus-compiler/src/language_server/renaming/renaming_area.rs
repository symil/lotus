use std::{collections::HashSet, iter::FromIterator};
use parsable::DataLocation;

#[derive(Debug)]
pub struct RenamingArea {
    pub occurences: HashSet<DataLocation>
}

impl RenamingArea {
    pub fn new(definition: &DataLocation) -> Self {
        Self {
            occurences: HashSet::from_iter(vec![definition.clone()]),
        }
    }

    pub fn add_occurence(&mut self, occurence: &DataLocation) {
        self.occurences.insert(occurence.clone());
    }

    pub fn contains_cursor(&self, file_path: &str, cursor_index: usize) -> bool {
        for location in &self.occurences {
            if location.contains_cursor(file_path, cursor_index) {
                return true;
            }
        }

        false
    }

    pub fn get_all_occurences(&self) -> Vec<DataLocation> {
        self.occurences.iter().map(|location| location.clone()).collect()
    }
}