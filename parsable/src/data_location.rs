use std::{collections::hash_map::DefaultHasher, hash::{Hash, Hasher}, rc::Rc, fmt::Debug};
use crate::{line_col_lookup::{lookup_line_col}, file_info::FileInfo};

#[derive(Clone, Default)]
pub struct DataLocation {
    pub file: Rc<FileInfo>,
    pub start: usize,
    pub end: usize,
}

impl DataLocation {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.file.path.is_empty()
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

    pub fn contains_cursor(&self, file_path: &str, cursor_index: usize) -> bool {
        self.file.path.as_str() == file_path && self.start <= cursor_index && self.end >= cursor_index
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

impl Hash for DataLocation {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.file.path.hash(state);
        self.start.hash(state);
        self.end.hash(state);
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

impl Debug for DataLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (line, col) = self.get_line_col();

        f.debug_struct("DataLocation")
            .field("file_path", &self.file.path)
            .field("line", &line)
            .field("col", &col)
            .finish()
    }
}