use std::rc::Rc;
use parsable::{ItemLocation, FileInfo};
use super::CursorLocation;

#[derive(Debug, Clone)]
pub struct Cursor {
    pub location: Option<ItemLocation>
}

impl Cursor {
    pub fn new(cursor_location: &Option<CursorLocation>) -> Self {
        let location = match cursor_location {
            Some(cursor_location) => {
                let package_root_path = cursor_location.root_directory_path.clone();
                let path = cursor_location.file_path.clone();
                let content = String::new();

                Some(ItemLocation {
                    file: Rc::new(FileInfo::new(content, path, package_root_path)),
                    start: cursor_location.index,
                    end: cursor_location.index,
                })
            },
            None => None,
        };

        Self {
            location,
        }
    }

    pub fn get_location(&self) -> Option<&ItemLocation> {
        self.location.as_ref()
    }

    pub fn exists(&self) -> bool {
        self.location.is_some()
    }

    pub fn get_hovered_location<'a>(&'a self, location: Option<&'a ItemLocation>) -> Option<&'a ItemLocation> {
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

    pub fn is_on_location(&self, location: &ItemLocation) -> bool {
        self.get_hovered_location(Some(location)).is_some()
    }
}