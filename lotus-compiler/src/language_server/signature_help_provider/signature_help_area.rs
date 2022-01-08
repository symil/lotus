use parsable::DataLocation;
use crate::{program::{Signature, FunctionBlueprint, FunctionCall}, utils::Link};

pub struct SignatureHelpArea {
    pub location: DataLocation,
    pub function_name: String,
    pub function_call: FunctionCall,
    pub argument_locations: Vec<DataLocation>
}

impl SignatureHelpArea {
    pub fn new(location: &DataLocation, name: &str, function_call: &FunctionCall) -> Self {
        Self {
            location: location.offset(1, -1),
            function_name: name.to_string(),
            function_call: function_call.clone(),
            argument_locations: vec![],
        }
    }

    pub fn set_argument_location(&mut self, index: usize, next_arg_location: Option<&DataLocation>) {
        if index == self.argument_locations.len() {
            let file = self.location.file.clone();
            let start = match &self.argument_locations.last() {
                Some(location) => location.end + 1,
                None => self.location.start,
            };
            let end = match next_arg_location {
                Some(location) => location.start,
                None => self.location.end,
            };

            self.argument_locations.push(DataLocation { file, start, end });
        } else if index > self.argument_locations.len() {
            panic!("attempting to assign argument with index {}, but only {} arguments where assigned before", index, self.argument_locations.len());
        }
    }

    pub fn contains_cursor(&self, file_path: &str, cursor_index: usize) -> bool {
        self.location.contains_cursor(file_path, cursor_index)
    }

    pub fn get_active_argument_index(&self, file_path: &str, cursor_index: usize) -> Option<i32> {
        for (i, location) in self.argument_locations.iter().enumerate().rev() {
            if location.contains_cursor(file_path, cursor_index) {
                return Some(i as i32);
            }
        }

        None
    }
}