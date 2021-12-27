use parsable::DataLocation;

#[derive(Debug)]
pub struct SharedIdentifier {
    pub definition: DataLocation,
    pub usages: Vec<DataLocation>
}

impl SharedIdentifier {
    pub fn new(definition: &DataLocation) -> Self {
        Self {
            definition: definition.clone(),
            usages: vec![],
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