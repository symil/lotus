use std::{collections::HashMap, hash::Hash};

#[derive(Default)]
pub struct VecHashMap<K, V> {
    pub hashmap: HashMap<K, Vec<V>>
}

pub trait WithId {
    fn get_id(&self) -> usize;
}

impl<K : Eq + Hash + Clone, V : WithId> VecHashMap<K, V> {
    pub fn len(&self) -> usize {
        self.hashmap.len()
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        match self.hashmap.get(key) {
            Some(vec) => vec.first(),
            None => None,
        }
    }

    // pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
    //     match self.hashmap.get_mut(key) {
    //         Some(vec) => vec.first_mut(),
    //         None => None,
    //     }
    // }

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

    pub fn get_by_id(&self, key: &K, id: usize) -> Option<&V> {
        match self.hashmap.get(key) {
            Some(vec) => vec.iter().find(|value| value.get_id() == id),
            None => None,
        }
    }

    pub fn get_mut_by_id(&mut self, key: &K, id: usize) -> Option<&mut V> {
        match self.hashmap.get_mut(key) {
            Some(vec) => vec.iter_mut().find(|value| value.get_id() == id),
            None => None,
        }
    }
}