use std::{collections::HashMap, hash::Hash};

#[derive(Default)]
pub struct VecHashMap<K, V> {
    hashmap: HashMap<K, Vec<V>>
}

impl<K : Eq + Hash + Clone, V> VecHashMap<K, V> {
    pub fn len(&self) -> usize {
        self.hashmap.len()
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        match self.hashmap.get(key) {
            Some(vec) => vec.first(),
            None => None,
        }
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        match self.hashmap.get_mut(key) {
            Some(vec) => vec.first_mut(),
            None => None,
        }
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    pub fn insert(&mut self, key: &K, value: V) {
        if let Some(vec) = self.hashmap.get_mut(key) {
            vec.push(value);
        } else {
            self.hashmap.insert(key.clone(), vec![value]);
        }
    }

    pub fn get_with_predicate<F : FnMut(&V) -> bool>(&self, key: &K, predicate: F) -> Option<&V> {
        match self.hashmap.get(key) {
            Some(vec) => vec.iter().find(|value| predicate(value)),
            None => None,
        }
    }

    pub fn get_mut_with_predicate<F : FnMut(&V) -> bool>(&self, key: &K, predicate: F) -> Option<&mut V> {
        match self.hashmap.get(key) {
            Some(vec) => vec.iter_mut().find(|value| predicate(value)),
            None => None,
        }
    }
}