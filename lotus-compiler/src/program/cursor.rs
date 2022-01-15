use std::rc::Rc;
use parsable::{DataLocation, FileInfo};
use super::CursorLocation;

#[derive(Debug, Clone)]
pub struct Cursor {
    pub location: Option<DataLocation>
}

impl Cursor {
    pub fn new(cursor_location: &Option<CursorLocation>) -> Self {
        let location = match cursor_location {
            Some(cursor_location) => Some(DataLocation {
                file: Rc::new(FileInfo {
                    package_root_path: cursor_location.root_directory_path.clone(),
                    path: cursor_location.file_path.clone(),
                    content: String::new(),
                }),
                start: cursor_location.index,
                end: cursor_location.index,
            }),
            None => None,
        };

        Self {
            location,
        }
    }

    pub fn get_location(&self) -> Option<&DataLocation> {
        self.location.as_ref()
    }

    pub fn exists(&self) -> bool {
        self.location.is_some()
    }

    pub fn get_hovered_location<'a>(&'a self, location: Option<&'a DataLocation>) -> Option<&'a DataLocation> {
        match &self.location {
            Some(cursor_location) => match location {
                Some(loc) => match loc.contains(cursor_location) {
                    true => Some(loc),
                    false => None,
                },
                None => Some(cursor_location),
            },
            None => None,
        }
    }

    pub fn is_on_location(&self, location: &DataLocation) -> bool {
        self.get_hovered_location(Some(location)).is_some()
    }
}