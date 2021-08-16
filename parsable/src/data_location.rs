use std::{collections::hash_map::DefaultHasher, hash::{Hash, Hasher}};

#[derive(Debug, Default, Clone)]
pub struct DataLocation {
    pub start: usize,
    pub end: usize,
    pub package_name: &'static str,
    pub file_name: &'static str,
    pub line: usize,
    pub column: usize
}

impl Hash for DataLocation {
    fn hash<H: Hasher>(&self, _state: &mut H) {

    }
}

impl DataLocation {
    pub fn get_hash(&self) -> u64 {
        let mut state = DefaultHasher::new();

        self.package_name.hash(&mut state);
        self.file_name.hash(&mut state);
        self.line.hash(&mut state);
        self.column.hash(&mut state);

        state.finish()
    }
}