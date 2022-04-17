use std::{rc::Rc, collections::HashMap};
use parsable::{LineColLookup, ItemLocation, FileInfo};

pub struct LineColIndex {
    map: HashMap<Rc<FileInfo>, LineColLookup>
}

impl LineColIndex {
    pub fn new() -> Self {
        Self {
            map: HashMap::new()
        }
    }

    pub fn lookup(&mut self, location: &ItemLocation) -> (usize, usize) {
        let lookup = self.map
            .entry(location.file.clone())
            .or_insert_with(|| location.compute_lookup_index());
        
        lookup.get(location.start)
    }
}