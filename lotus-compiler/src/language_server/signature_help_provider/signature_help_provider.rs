use indexmap::IndexMap;
use parsable::DataLocation;
use crate::{program::{CursorInfo, Signature, FunctionBlueprint, FunctionCall}, language_server::location_contains_cursor, utils::Link};
use super::SignatureHelpArea;

pub struct SignatureHelpProvider {
    cursor: Option<CursorInfo>,
    areas: IndexMap<DataLocation, SignatureHelpArea>
}

impl SignatureHelpProvider {
    pub fn new(cursor: &Option<CursorInfo>) -> Self {
        Self {
            cursor: cursor.clone(),
            areas: IndexMap::new()
        }
    }

    pub fn declare_signature(&mut self, location: &DataLocation, name: &str, function_call: &FunctionCall) {
        if !location_contains_cursor(location, &self.cursor) {
            return;
        }

        self.areas
            .entry(location.clone())
            .or_insert_with(|| SignatureHelpArea::new(location, name, function_call));
    }

    pub fn add_argument_location(&mut self, signature_location: &DataLocation, argument_index: usize, next_arg_location: Option<&DataLocation>) {
        if !location_contains_cursor(signature_location, &self.cursor) {
            return;
        }

        let area = match self.areas.get_mut(signature_location) {
            Some(area) => area,
            None => {
                panic!("cannot find signature help area");
            },
        };

        area.set_argument_location(argument_index, next_arg_location);
    }

    pub fn get_area_under_cursor(&self, file_path: &str, cursor_index: usize) -> Option<&SignatureHelpArea> {
        for area in self.areas.values().rev() {
            if area.contains_cursor(file_path, cursor_index) {
                return Some(area);
            }
        }

        None
    }
}