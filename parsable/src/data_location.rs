use std::{collections::hash_map::DefaultHasher, hash::{Hash, Hasher}};

#[derive(Debug, Default, Clone)]
pub struct DataLocation {
    pub start: usize,
    pub end: usize,
    pub file_namespace: &'static str,
    pub file_name: &'static str,
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
}