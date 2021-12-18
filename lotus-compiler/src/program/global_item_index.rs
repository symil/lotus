use std::{borrow::Borrow, cell::Ref, collections::{HashMap, hash_map::DefaultHasher}, hash::{Hash, Hasher}, mem::take};
use indexmap::IndexMap;
use parsable::DataLocation;
use crate::{items::{Identifier, Visibility}, utils::Link};

#[derive(Debug)]
pub struct GlobalItemIndex<V> {
    pub items_by_id: IndexMap<u64, Link<V>>,
    pub items_by_name: HashMap<String, Vec<Link<V>>>
}

pub trait GlobalItem {
    fn get_name(&self) -> &Identifier;
    fn get_visibility(&self) -> Visibility;
}

fn get_id_from_location(location: &DataLocation, marker: Option<u64>) -> u64 {
    let mut state = DefaultHasher::new();

    location.hash(&mut state);

    if let Some(n) = marker {
        n.hash(&mut state);
    }

    state.finish()
}

impl<V : GlobalItem> GlobalItemIndex<V> {
    pub fn len(&self) -> usize {
        self.items_by_name.len()
    }

    pub fn insert(&mut self, value: V, marker: Option<u64>) -> Link<V> {
        let id = get_id_from_location(&value.get_name().location, marker);
        let name = value.get_name().to_string();
        let item = Link::new(value);

        self.items_by_id.insert(id, item.clone());

        if let Some(vec) = self.items_by_name.get_mut(name.as_str()) {
            vec.push(item.clone());
        } else {
            self.items_by_name.insert(name, vec![item.clone()]);
        }

        item
    }

    pub fn get_by_identifier(&self, getter_name: &Identifier) -> Option<Link<V>> {
        let candidates = self.items_by_name.get(getter_name.as_str())?;
        let getter_location : &DataLocation = &getter_name.location;

        for item_wrapped in candidates.iter() {
            let ok = item_wrapped.with_ref(|item| {
                let item_name = item.get_name();
                let item_location = &item_name.location;
                match item.get_visibility() {
                    Visibility::Private => item_location.file_path == getter_location.file_path,
                    Visibility::Public => item_location.package_root_path == getter_location.package_root_path,
                    Visibility::Export => true,
                    Visibility::System => item_location.package_root_path == getter_location.package_root_path,
                    Visibility::None => false,
                }
            });

            if ok {
                return Some(item_wrapped.clone());
            }
        }

        None
    }

    pub fn get_by_name(&self, name: &str) -> Option<Link<V>> {
        let candidates = self.items_by_name.get(name)?;

        for item_wrapped in candidates.iter() {
            let ok = item_wrapped.with_ref(|item| {
                let item_name = item.get_name();
                let item_location = &item_name.location;
                match item.get_visibility() {
                    Visibility::Private => false,
                    Visibility::Public => false,
                    Visibility::Export => true,
                    Visibility::System => true,
                    Visibility::None => false,
                }
            });

            if ok {
                return Some(item_wrapped.clone());
            }
        }

        None
    }

    pub fn get_by_name_private(&self, name: &str) -> Option<Link<V>> {
        let candidates = self.items_by_name.get(name)?;

        for item_wrapped in candidates.iter() {
            let ok = item_wrapped.with_ref(|item| {
                let item_name = item.get_name();
                let item_location = &item_name.location;
                match item.get_visibility() {
                    Visibility::Private => true,
                    Visibility::Public => true,
                    Visibility::Export => true,
                    Visibility::System => true,
                    Visibility::None => false,
                }
            });

            if ok {
                return Some(item_wrapped.clone());
            }
        }

        None
    }

    pub fn get_by_location(&self, value_name: &Identifier, marker: Option<u64>) -> Link<V> {
        let id = get_id_from_location(&value_name, marker);

        self.items_by_id.get(&id).unwrap().clone()
    }

    pub fn get_all(&self) -> Vec<Link<V>> {
        self.items_by_id.values().map(|v| v.clone()).collect()
    }
}

impl<V> Default for GlobalItemIndex<V> {
    fn default() -> Self {
        Self {
            items_by_id: IndexMap::new(),
            items_by_name: HashMap::new(),
        }
    }
}