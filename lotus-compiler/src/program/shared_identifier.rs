use parsable::DataLocation;

use super::Type;

#[derive(Debug)]
pub struct SharedIdentifier {
    pub definition: Option<DataLocation>,
    pub type_info: Option<Type>,
    pub usages: Vec<DataLocation>,
}

impl SharedIdentifier {
    pub fn new(definition: Option<&DataLocation>, type_info: Option<&Type>) -> Self {
        Self {
            definition: definition.cloned(),
            type_info: type_info.cloned(),
            usages: vec![],
        }
    }

    pub fn get_all_occurences(&self) -> Vec<DataLocation> {
        let mut result = vec![];

        if let Some(location) = &self.definition {
            result.push(location.clone());
        }
        result.extend_from_slice(&self.usages);

        result
    }

    pub fn match_cursor(&self, cursor_file_path: &str, cursor_index: usize) -> Option<&DataLocation> {
        if let Some(definition) = &self.definition {
            if definition.contains_cursor(cursor_file_path, cursor_index) {
                return Some(definition);
            }
        }

        for usage in &self.usages {
            if usage.contains_cursor(cursor_file_path, cursor_index) {
                return Some(&usage);
            }
        }

        None
    }
}