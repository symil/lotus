use std::{collections::HashMap, hash::Hash};
use crate::items::VisibilityToken;
use super::{Id, ItemMetadata};

#[derive(Debug)]
pub struct VecHashMap<K, V> {
    pub hashmap: HashMap<K, Vec<V>>
}

pub trait WithMetadata {
    fn get_metadata(&self) -> &ItemMetadata;
}

impl<K : Eq + Hash, V : WithMetadata> VecHashMap<K, V> {
    pub fn len(&self) -> usize {
        self.hashmap.len()
    }

    pub fn get(&self, key: &K, from_package_name: &str, from_file_name: &str) -> Option<&V> {
        let candidates = self.hashmap.get(key)?;

        for value in candidates.iter() {
            let metadata = value.get_metadata();
            let ok = match &metadata.visibility {
                VisibilityToken::Private => metadata.package_name == from_package_name && metadata.file_name == from_file_name,
                VisibilityToken::Public => metadata.package_name == from_package_name,
                VisibilityToken::Export => true,
                VisibilityToken::System => false,
            };

            if ok {
                return Some(value);
            }
        }

        None
    }

    pub fn contains_key(&self, key: &K, from_package_name: &str, from_file_name: &str) -> bool {
        self.get(key, from_package_name, from_file_name).is_some()
    }

    pub fn insert(&mut self, key: K, value: V) {
        if let Some(vec) = self.hashmap.get_mut(&key) {
            vec.push(value);
        } else {
            self.hashmap.insert(key, vec![value]);
        }
    }

    pub fn get_by_id(&self, key: &K, id: Id) -> Option<&V> {
        match self.hashmap.get(key) {
            Some(vec) => vec.iter().find(|value| value.get_metadata().id == id),
            None => None,
        }
    }

    pub fn get_mut_by_id(&mut self, key: &K, id: Id) -> Option<&mut V> {
        match self.hashmap.get_mut(key) {
            Some(vec) => vec.iter_mut().find(|value| value.get_metadata().id == id),
            None => None,
        }
    }
}

impl<K, V> Default for VecHashMap<K, V> {
    fn default() -> Self {
        Self {
            hashmap: HashMap::new()
        }
    }
}