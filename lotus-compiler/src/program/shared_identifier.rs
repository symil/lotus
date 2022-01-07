use std::{collections::HashSet, hash::Hash};
use parsable::DataLocation;
use super::Type;

#[derive(Debug)]
pub struct SharedIdentifier {
    pub definition: Option<DataLocation>,
    pub type_info: Option<Type>,
    pub usages: HashSet<SharedIdentifierUsage>,
}

#[derive(Debug)]
pub struct SharedIdentifierUsage {
    pub location: DataLocation,
    pub shadow: bool,
}

impl SharedIdentifier {
    pub fn new(definition: Option<&DataLocation>, type_info: Option<&Type>) -> Self {
        Self {
            definition: definition.cloned(),
            type_info: type_info.cloned(),
            usages: HashSet::new(),
        }
    }

    pub fn add_usage(&mut self, location: &DataLocation) {
        self.usages.insert(SharedIdentifierUsage {
            location: location.clone(),
            shadow: false,
        });
    }

    pub fn get_all_occurences(&self) -> Vec<DataLocation> {
        let mut result = vec![];

        if let Some(location) = &self.definition {
            result.push(location.clone());
        }

        for usage in &self.usages {
            if !usage.shadow {
                result.push(usage.location.clone());
            }
        }

        result
    }

    pub fn match_cursor(&self, cursor_file_path: &str, cursor_index: usize) -> Option<&DataLocation> {
        if let Some(definition) = &self.definition {
            if definition.contains_cursor(cursor_file_path, cursor_index) {
                return Some(definition);
            }
        }

        for usage in &self.usages {
            if usage.location.contains_cursor(cursor_file_path, cursor_index) {
                return Some(&usage.location);
            }
        }

        None
    }
}

impl PartialEq for SharedIdentifierUsage {
    fn eq(&self, other: &Self) -> bool {
        self.location == other.location
    }
}

impl Hash for SharedIdentifierUsage {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.location.hash(state);
    }
}

impl Eq for SharedIdentifierUsage {

}