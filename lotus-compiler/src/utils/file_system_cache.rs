use std::{fs, rc::Rc, collections::HashMap, fs::File, time::{SystemTime, UNIX_EPOCH}, mem::take};
use crate::items::ParsedSourceFile;

struct CachedItem<T, E> {
    value: Result<Rc<T>, Rc<E>>,
    last_read: SystemTime
}

pub struct FileSystemCache<T, E> {
    files: HashMap<String, CachedItem<T, E>>,
    hook: Option<(String, String)>
}

impl<T : Default, E> FileSystemCache<T, E> {
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
            hook: None
        }
    }

    pub fn delete_hook(&mut self) {
        self.hook = None;
    }

    pub fn set_hook(&mut self, file_path: &str, file_content: String) {
        self.hook = Some((file_path.to_string(), file_content));
    }

    fn read_from_hook(&mut self, file_path: &str) -> Option<String> {
        let take_from_hook = self.hook.as_ref().map(|(path, _)| path).is_some_and(|path| path == &file_path);

        match take_from_hook {
            true => Some(self.hook.take().unwrap().1),
            false => None
        }
    }

    pub fn read_file<F : FnOnce(String) -> Result<T, E>>(&mut self, file_path: &str, process_function: F) -> Result<Rc<T>, Rc<E>> {
        let content_from_hook = self.read_from_hook(file_path);
        let item = self.files.entry(file_path.to_string())
            .or_insert_with(|| CachedItem {
                value: Ok(Rc::new(T::default())),
                last_read: UNIX_EPOCH
            });
        let metadata = fs::metadata(file_path).unwrap();
        let modified = metadata.modified().unwrap();
        let process_again = content_from_hook.is_some() || modified > item.last_read;

        if process_again {
            let file_content = content_from_hook.unwrap_or_else(|| fs::read_to_string(file_path).unwrap());

            item.value = match process_function(file_content) {
                Ok(value) => Ok(Rc::new(value)),
                Err(error) => Err(Rc::new(error)),
            };
            item.last_read = SystemTime::now();
        }

        item.value.clone()
    }
}

impl<T : Default, E> Default for FileSystemCache<T, E> {
    fn default() -> Self {
        Self::new()
    }
}