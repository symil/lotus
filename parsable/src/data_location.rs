use std::{collections::hash_map::DefaultHasher, hash::{Hash, Hasher}, rc::Rc};
use crate::line_col_lookup::{lookup_line_col};

#[derive(Debug, Clone, Default)]
pub struct DataLocation {
    pub package_root_path: Rc<String>,
    pub file_path: Rc<String>,
    pub file_content: Rc<String>,
    pub start: usize,
    pub end: usize,
}

impl Hash for DataLocation {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.file_path.hash(state);
        self.start.hash(state);
        self.end.hash(state);
    }
}
thread_local! {
    static EMPTY_STRING : Rc<String> = Rc::new(String::new());
}

impl DataLocation {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn get_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();

        self.hash(&mut hasher);

        hasher.finish()
    }

    pub fn get_end(&self) -> DataLocation {
        DataLocation {
            package_root_path: self.package_root_path.clone(),
            file_path: self.file_path.clone(),
            file_content: self.file_content.clone(),
            start: self.end,
            end: self.end,
        }
    }

    pub fn contains_cursor(&self, file_path: &str, index: usize) -> bool {
        self.file_path.as_str() == file_path && self.start <= index && self.end >= index
    }

    pub fn is_empty(&self) -> bool {
        self.file_content.is_empty()
    }

    pub fn as_str(&self) -> &str {
        &self.file_content.as_ref()[self.start..self.end]
    }

    pub fn get_line_col(&self) -> (usize, usize) {
        lookup_line_col(self.file_path.as_str(), self.file_content.as_str(), self.start)
    }

    pub fn length(&self) -> usize {
        self.end - self.start
    }
}

impl PartialEq for DataLocation {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start &&
        self.end == other.end &&
        self.file_path == other.file_path
    }
}

impl Eq for DataLocation {

}