use std::{collections::HashMap, fmt, hash::Hash};

pub type Id = usize;

pub fn display_join<T : fmt::Display>(values: &[T], separator: &str) -> String {
    values.iter().map(|value| format!("{}", value)).collect::<Vec<String>>().join(separator)
}

pub fn insert_in_vec_hashmap<K : Clone + Hash + Eq, V>(hashmap: &mut HashMap<K, Vec<V>>, key: &K, value: V) -> Option<V> {
    if let Some(vec) = hashmap.get_mut(key) {
        vec.push(value);
    } else {
        hashmap.insert(key.clone(), vec![value]);
    }

    None
}