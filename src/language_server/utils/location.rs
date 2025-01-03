use parsable::ItemLocation;
use crate::program::CursorLocation;

pub fn is_invalid_location(location: &ItemLocation) -> bool {
    location.is_empty() || location.as_str().as_bytes()[0] == b'#'
}