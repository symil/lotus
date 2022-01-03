use std::{fs, rc::Rc, collections::HashMap, fs::File, time::{SystemTime, UNIX_EPOCH}};

use crate::items::LotusFile;

struct CachedItem<T, E> {
    value: Result<Rc<T>, Rc<E>>,
    last_read: SystemTime
}

pub struct FileSystemCache<T, E> {
    files: HashMap<String, CachedItem<T, E>>
}

impl<T : Default, E> FileSystemCache<T, E> {
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
        }
    }

    pub fn read_file<F : Fn(String) -> Result<T, E>>(&mut self, file_path: &str, process_function: F) -> Result<Rc<T>, Rc<E>> {
        let metadata = fs::metadata(file_path).unwrap();
        let modified = metadata.modified().unwrap();
        let item = self.files.entry(file_path.to_string())
            .or_insert_with(|| CachedItem {
                value: Ok(Rc::new(T::default())),
                last_read: UNIX_EPOCH
            });
        
        match modified > item.last_read {
            true => {
                let file_content = fs::read_to_string(file_path).unwrap();
                item.value = match process_function(file_content) {
                    Ok(value) => Ok(Rc::new(value)),
                    Err(error) => Err(Rc::new(error)),
                };
                item.last_read = SystemTime::now();
                item.value.clone()
            },
            false => {
                item.value.clone()
            }
        }
    }
}

impl<T : Default, E> Default for FileSystemCache<T, E> {
    fn default() -> Self {
        Self::new()
    }
}