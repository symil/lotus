use std::{collections::HashSet, iter::FromIterator};
use parsable::ItemLocation;

#[derive(Debug)]
pub struct SharedName {
    pub definition: ItemLocation,
    pub occurences: HashSet<ItemLocation>
}

impl SharedName {
    pub fn new(definition: &ItemLocation) -> Self {
        Self {
            definition: definition.clone(),
            occurences: HashSet::new(),
        }
    }

    pub fn add_occurence(&mut self, occurence: &ItemLocation) {
        self.occurences.insert(occurence.clone());
    }
}