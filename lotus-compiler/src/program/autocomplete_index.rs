use std::collections::HashMap;
use parsable::DataLocation;
use super::{AutoCompleteZone, AutoCompleteDetails};

pub struct AutoCompleteIndex {
    pub files: HashMap<&'static str, Vec<AutoCompleteZone>>
}

impl AutoCompleteIndex {
    pub fn insert(&mut self, location: DataLocation, details: AutoCompleteDetails) {
        
    }
}