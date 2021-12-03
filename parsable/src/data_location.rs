use std::{collections::hash_map::DefaultHasher, hash::{Hash, Hasher}};

use crate::line_col_lookup::LineColLookup;

#[derive(Debug, Default, Clone)]
pub struct DataLocation {
    pub start: usize,
    pub end: usize,
    pub file_namespace: &'static str,
    pub file_name: &'static str,
    pub file_content: &'static str,
    pub line: usize,
    pub column: usize
}

impl Hash for DataLocation {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.file_namespace.hash(state);
        self.file_name.hash(state);
        self.line.hash(state);
        self.column.hash(state);
    }
}

impl DataLocation {
    pub fn get_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();

        self.hash(&mut hasher);

        hasher.finish()
    }

    pub fn get_end(&self) -> DataLocation {
        let line_col_lookup = LineColLookup::new(self.file_content);
        let (line, column) = line_col_lookup.get(self.end);

        DataLocation {
            start: self.end,
            end: self.end,
            file_namespace: self.file_namespace,
            file_name: self.file_name,
            file_content: self.file_content,
            line,
            column,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.file_content == ""
    }

    pub fn as_str(&self) -> &'static str {
        &self.file_content[self.start..self.end]
    }
}

impl PartialEq for DataLocation {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start &&
        self.end == other.end &&
        self.file_namespace == other.file_namespace &&
        self.file_name == other.file_name
    }
}