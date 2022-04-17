use std::hash::Hash;

#[derive(Debug, Default)]
pub struct FileInfo {
    pub path: String,
    pub content: String,
    pub package_root_path: String,
}

impl PartialEq for FileInfo {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl Hash for FileInfo {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.path.hash(state);
    }
}

impl Eq for FileInfo {
    
}