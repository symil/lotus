use parsable::ItemLocation;
use crate::{program::{Signature, FunctionBlueprint, FunctionCall, Cursor}, utils::Link};

pub struct SignatureHelp {
    pub location: ItemLocation,
    pub function_name: String,
    pub function_call: FunctionCall,
    pub argument_locations: Vec<ItemLocation>,
    pub active_argument_index: Option<usize>
}

impl SignatureHelp {
    pub fn new(location: &ItemLocation, name: &str, function_call: &FunctionCall) -> Self {
        Self {
            location: location.offset(1, -1),
            function_name: name.to_string(),
            function_call: function_call.clone(),
            argument_locations: vec![],
            active_argument_index: None,
        }
    }

    pub fn set_argument_location(&mut self, index: usize, next_arg_location: Option<&ItemLocation>, cursor: &Cursor) {
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

            let location = ItemLocation { file, start, end };

            if cursor.is_on_location(&location) {
                self.active_argument_index = Some(index);
            }

            self.argument_locations.push(location);

        } else if index > self.argument_locations.len() {
            panic!("attempting to assign argument with index {}, but only {} arguments where assigned before", index, self.argument_locations.len());
        }
    }
}