use parsable::DataLocation;
use crate::program::CursorInfo;

pub fn is_invalid_location(location: &DataLocation) -> bool {
    location.is_empty() || location.as_str().as_bytes()[0] == b'#'
}

pub fn location_contains_cursor(location: &DataLocation, cursor: &Option<CursorInfo>) -> bool {
    match cursor.as_ref() {
        Some(cursor) => location.contains_cursor(&cursor.file_path, cursor.index),
        None => false,
    }
}