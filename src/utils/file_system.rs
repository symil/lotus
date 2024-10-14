use std::path::{PathBuf, Path};

pub fn read_directory_recursively(directory_path: &Path) -> Vec<PathBuf> {
    let mut result = vec![];
    let path = Path::new(directory_path);

    if path.is_file() {
        result.push(path.to_path_buf());
    } else if path.is_dir() {
        if let Ok(entries) = path.read_dir() {
            for entry in entries {
                if let Ok(entry) = entry {
                    result.append(&mut read_directory_recursively(&entry.path()));
                }
            }
        }
    }

    result
}