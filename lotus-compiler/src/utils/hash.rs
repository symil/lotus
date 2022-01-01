use std::{collections::hash_map::DefaultHasher, hash::{Hash, Hasher}};

pub fn compute_hash<K : Hash>(key: &K) -> u64 {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    hasher.finish()
}