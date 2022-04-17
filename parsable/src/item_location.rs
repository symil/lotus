use std::{collections::hash_map::DefaultHasher, hash::{Hash, Hasher}, rc::Rc, fmt::Debug, path::{Path}};
use crate::{file_info::FileInfo, LineColLookup};

#[derive(Clone, Default)]
pub struct ItemLocation {
    pub file: Rc<FileInfo>,
    pub start: usize,
    pub end: usize,
}

impl ItemLocation {
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

    pub fn set_start_with_offset(&self, offset: usize) -> Self {
        self.clone().set_bounds(self.start + offset)
    }

    pub fn offset(&self, start: isize, end: isize) -> Self {
        Self {
            file: self.file.clone(),
            start: ((self.start as isize) + start) as usize,
            end: ((self.end as isize) + end) as usize,
        }
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

    pub fn until(&self, other: &Self) -> Self {
        Self {
            file: self.file.clone(),
            start: self.end,
            end: other.start,
        }
    }

    pub fn contains_cursor(&self, file_path: &str, cursor_index: usize) -> bool {
        self.file.path.as_str() == file_path && self.start <= cursor_index && self.end >= cursor_index
    }

    pub fn contains(&self, other: &Self) -> bool {
        other.file.path == self.file.path &&
        (other.start >= self.start && other.start <= self.end) &&
        (other.end >= self.start && other.end <= self.end)
    }

    pub fn as_str(&self) -> &str {
        &self.file.content[self.start..self.end]
    }

    pub fn compute_lookup_index(&self) -> LineColLookup {
        LineColLookup::new(&self.file.content)
    }

    pub fn length(&self) -> usize {
        self.end - self.start
    }

    pub fn get_root_directory_name(&self) -> &str {
        Path::new(&self.file.package_root_path).file_name().and_then(|os_str| os_str.to_str()).unwrap_or("")
    }
}

impl Hash for ItemLocation {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.file.path.hash(state);
        self.start.hash(state);
        self.end.hash(state);
    }
}

impl PartialEq for ItemLocation {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start &&
        self.end == other.end &&
        Rc::ptr_eq(&self.file, &other.file)
    }
}

impl Eq for ItemLocation {

}

impl Debug for ItemLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lookup = self.compute_lookup_index();
        let (start_line, start_col) = lookup.get(self.start);
        let (end_line, end_col) = lookup.get(self.end);

        write!(f, "{}: ({},{})->({}:{})", &self.file.path, start_line, start_col, end_line, end_col)
    }
}