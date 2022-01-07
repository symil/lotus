use std::{collections::hash_map::DefaultHasher, hash::{Hash, Hasher}, rc::Rc};
use crate::{line_col_lookup::{lookup_line_col}, file_info::FileInfo};

#[derive(Debug, Clone, Default)]
pub struct DataLocation {
    pub file: Rc<FileInfo>,
    pub start: usize,
    pub end: usize,
}

impl Hash for DataLocation {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.file.path.hash(state);
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

    pub fn get_end(&self) -> Self {
        self.clone().set_bounds(self.end)
    }

    pub fn offset(&self, offset: usize) -> Self {
        self.clone().set_bounds(self.start + offset)
    }

    fn _set_start(mut self, start: usize) -> Self {
        self.start = start;
        self
    }

    fn _set_end(mut self, end: usize) -> Self {
        self.end = end;
        self
    }

    fn set_bounds(mut self, offset: usize) -> Self {
        self.start = offset;
        self.end = offset;
        self
    }

    pub fn contains_cursor(&self, file_path: &str, index: usize) -> bool {
        self.file.path.as_str() == file_path && self.start <= index && self.end >= index
    }

    pub fn as_str(&self) -> &str {
        &self.file.content[self.start..self.end]
    }

    pub fn get_line_col(&self) -> (usize, usize) {
        lookup_line_col(self.file.path.as_str(), self.file.content.as_str(), self.start)
    }

    pub fn length(&self) -> usize {
        self.end - self.start
    }
}

impl PartialEq for DataLocation {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start &&
        self.end == other.end &&
        Rc::ptr_eq(&self.file, &other.file)
    }
}

impl Eq for DataLocation {

}