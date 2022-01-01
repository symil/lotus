use parsable::DataLocation;

use super::Type;

#[derive(Debug)]
pub struct SharedIdentifier {
    pub definition: DataLocation,
    pub usages: Vec<DataLocation>,
    pub type_info: Option<Type>
}

impl SharedIdentifier {
    pub fn new(definition: &DataLocation, type_info: Option<&Type>) -> Self {
        Self {
            definition: definition.clone(),
            usages: vec![],
            type_info: type_info.cloned()
        }
    }

    pub fn get_all_occurences(&self) -> Vec<DataLocation> {
        let mut result = vec![];

        result.push(self.definition.clone());
        result.extend_from_slice(&self.usages);

        result
    }

    pub fn match_cursor(&self, cursor_file_path: &str, cursor_index: usize) -> Option<&DataLocation> {
        if self.definition.contains_cursor(cursor_file_path, cursor_index) {
            return Some(&self.definition);
        }

        for usage in &self.usages {
            if usage.contains_cursor(cursor_file_path, cursor_index) {
                return Some(&usage);
            }
        }

        None
    }
}