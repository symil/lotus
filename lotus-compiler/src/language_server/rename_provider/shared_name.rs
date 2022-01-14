use std::{collections::HashSet, iter::FromIterator};
use parsable::DataLocation;

#[derive(Debug)]
pub struct SharedName {
    pub definition: DataLocation,
    pub occurences: HashSet<DataLocation>
}

impl SharedName {
    pub fn new(definition: &DataLocation) -> Self {
        Self {
            definition: definition.clone(),
            occurences: HashSet::new(),
        }
    }

    pub fn add_occurence(&mut self, occurence: &DataLocation) {
        self.occurences.insert(occurence.clone());
    }
}