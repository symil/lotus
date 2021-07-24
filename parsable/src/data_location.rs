use std::hash::{Hash, Hasher};

#[derive(Debug, Default, Clone)]
pub struct DataLocation {
    pub start: usize,
    pub end: usize,
    pub file_name: &'static str,
    pub line: usize,
    pub column: usize
}

impl Hash for DataLocation {
    fn hash<H: Hasher>(&self, _state: &mut H) {

    }
}