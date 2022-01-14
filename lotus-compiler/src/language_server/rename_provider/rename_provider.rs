use std::collections::HashMap;
use parsable::DataLocation;
use crate::{language_server::is_invalid_location, program::{CursorLocation, Cursor}};
use super::SharedName;

#[derive(Debug)]
pub struct RenameProvider {
    pub cursor: Cursor,
    pub shared_names: HashMap<DataLocation, SharedName>
}

impl RenameProvider {
    pub fn new(cursor: &Cursor) -> Self {
        Self {
            cursor: cursor.clone(),
            shared_names: HashMap::new(),
        }
    }

    pub fn add_occurence(&mut self, occurence: &DataLocation, definition: &DataLocation) {
        if !self.cursor.exists() || is_invalid_location(definition) || is_invalid_location(occurence) {
            return;
        }

        self.shared_names.entry(definition.clone())
            .or_insert(SharedName::new(definition))
            .add_occurence(occurence);
    }

    pub fn get_shared_name(&self) -> Option<(&SharedName, &DataLocation)> {
        let cursor_location = self.cursor.location.as_ref()?;

        for shared_name in self.shared_names.values() {
            if shared_name.definition.file.package_root_path != cursor_location.file.package_root_path {
                continue;
            }

            for occurence in &shared_name.occurences {
                if self.cursor.is_on_location(occurence) {
                    return Some((shared_name, occurence));
                }
            }
        }

        None
    }
}