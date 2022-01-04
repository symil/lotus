use std::collections::HashMap;
use parsable::DataLocation;
use super::{CompletionArea, CompletionDetails};

#[derive(Debug, Default)]
pub struct CompletionAreaIndex {
    pub files: HashMap<String, Vec<CompletionArea>>
}

impl CompletionAreaIndex {
    pub fn insert(&mut self, location: DataLocation, details: CompletionDetails) {
        let mut areas = self.files.entry(location.file_path.to_string()).or_insert_with(|| vec![]);

        areas.push(CompletionArea {
            location,
            details,
        });
    }

    pub fn get(&self, file_path: &str, cursor_index: usize) -> Option<&CompletionArea> {
        match self.files.get(file_path) {
            Some(areas) => areas.iter().find(|area| area.contains_cursor(cursor_index)),
            None => None,
        }
    }
}