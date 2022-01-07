use parsable::DataLocation;
use crate::{program::{Signature, FunctionBlueprint}, utils::Link};

pub struct SignatureHelpArea {
    pub location: DataLocation,
    pub function: Link<FunctionBlueprint>,
    pub argument_locations: Vec<DataLocation>
}

impl SignatureHelpArea {
    pub fn new(location: &DataLocation, function: &Link<FunctionBlueprint>) -> Self {
        Self {
            location: location.clone(),
            function: function.clone(),
            argument_locations: vec![],
        }
    }

    pub fn set_argument_location(&mut self, index: usize, location: &DataLocation) {
        if index == self.argument_locations.len() {
            self.argument_locations.push(location.clone());
        } else if index > self.argument_locations.len() {
            panic!("attempting to assign argument with index {}, but only {} arguments where assigned before", index, self.argument_locations.len());
        }
    }

    pub fn contains_cursor(&self, file_path: &str, cursor_index: usize) -> bool {
        self.location.contains_cursor(file_path, cursor_index)
    }

    pub fn get_active_argument_index(&self, file_path: &str, cursor_index: usize) -> Option<usize> {
        for (i, location) in self.argument_locations.iter().enumerate() {
            if location.contains_cursor(file_path, cursor_index) {
                return Some(i);
            }
        }

        None
    }

    pub fn get_function(&self) -> &Link<FunctionBlueprint> {
        &self.function
    }
}