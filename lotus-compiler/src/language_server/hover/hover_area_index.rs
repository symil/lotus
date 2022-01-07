use std::collections::HashMap;
use parsable::DataLocation;
use crate::{program::{CursorInfo, Type}, language_server::{is_invalid_location, location_contains_cursor}};
use super::HoverArea;

pub struct HoverAreaIndex {
    // TODO: option to disable the index
    pub cursor: Option<CursorInfo>,
    pub areas: HashMap<DataLocation, HoverArea>
}

impl HoverAreaIndex {
    pub fn new(cursor: Option<CursorInfo>) -> Self {
        Self {
            cursor,
            areas: HashMap::new(),
        }
    }

    fn area<F : FnOnce(&mut HoverArea)>(&mut self, location: &DataLocation, callback: F) {
        if is_invalid_location(location) || !location_contains_cursor(location, &self.cursor) {
            return;
        }

        let area = self.areas
            .entry(location.clone())
            .or_insert_with(|| HoverArea::new(location));
        
        callback(area);
    }

    pub fn set_type(&mut self, location: &DataLocation, ty: &Type) {
        self.area(location, |area| area.set_type(ty));
    }

    pub fn set_definition(&mut self, location: &DataLocation, definition: &DataLocation) {
        self.area(location, |area| area.set_definition(definition));
    }

    pub fn get_area_under_cursor(&self, file_path: &str, cursor_index: usize) -> Option<&HoverArea> {
        for area in self.areas.values() {
            if area.contains_cursor(file_path, cursor_index) {
                return Some(area);
            }
        }

        None
    }
}