use std::{collections::HashMap, fmt, hash::Hash, ops::Deref};

use super::Type;

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

pub fn print_type_ref_list(types: &[&Type]) {
    let mut s = String::new();

    if let Some(ty) = types.first() {
        s.push_str(&format!("{}", types[0].to_string()));
    }

    for i in 1..types.len() {
        s.push_str(&format!(", {}", types[0].to_string()));
    }

    println!("[{}]", s);
}

pub fn print_type_list(types: &[Type]) {
    let v : Vec<&Type> = types.iter().map(|ty| ty).collect();

    print_type_ref_list(&v);
}

pub fn hashmap_get_or_insert_with<'a, K : Clone + Eq + Hash, V, F : Fn() -> V>(hashmap: &'a mut HashMap<K, V>, key: &K, f: F) -> &'a mut V {
    if !hashmap.contains_key(key) {
        hashmap.insert(key.clone(), f());
    }

    hashmap.get_mut(key).unwrap()
}