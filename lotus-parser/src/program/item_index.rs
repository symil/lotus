use std::{collections::HashMap, hash::Hash, mem::take};
use indexmap::IndexMap;
use parsable::DataLocation;
use crate::items::{Identifier, Visibility};

#[derive(Debug)]
pub struct ItemIndex<V> {
    pub id_to_item: IndexMap<u64, Option<V>>,
    pub name_to_ids: HashMap<String, Vec<u64>>
}

pub trait GlobalItem {
    fn get_id(&self) -> u64;
    fn get_name(&self) -> &str;
    fn get_location(&self) -> &DataLocation;
    fn get_visibility(&self) -> Visibility;
}

impl<V : GlobalItem> ItemIndex<V> {
    pub fn len(&self) -> usize {
        self.id_to_item.len()
    }

    pub fn insert(&mut self, value: V) {
        let id = value.get_id();
        let name = value.get_name();

        if let Some(vec) = self.name_to_ids.get_mut(name) {
            vec.push(id);
        } else {
            self.name_to_ids.insert(name.to_string(), vec![id]);
        }

        self.id_to_item.insert(id, Some(value));
    }

    pub fn get_by_name(&self, name: &Identifier) -> Option<&V> {
        let candidates = self.name_to_ids.get(name.as_str())?;
        let getter_location : &DataLocation = &name.location;

        for id in candidates.iter() {
            if let Some(value) = self.id_to_item.get(id).unwrap() {
                let location = value.get_location();
                let ok = match value.get_visibility() {
                    Visibility::Private => location.file_namespace == getter_location.file_namespace && location.file_name == getter_location.file_name,
                    Visibility::Public => location.file_namespace == getter_location.file_namespace,
                    Visibility::Export => true,
                    Visibility::System => location.file_namespace == getter_location.file_namespace,
                };

                if ok {
                    return Some(value);
                }
            }
        }

        None
    }

    pub fn get_by_id(&self, id: u64) -> Option<&V> {
        match self.id_to_item.get(&id) {
            Some(value) => value.as_ref(),
            None => None
        }
    }

    pub fn take_by_index(&mut self, index: usize) -> V {
        let (_key_ref, value_ref) = self.id_to_item.get_index_mut(index).unwrap();

        take(value_ref).unwrap()
    }

    pub fn set_by_index(&mut self, index: usize, value: V) {
        let (key_ref, value_ref) = self.id_to_item.get_index_mut(index).unwrap();

        *value_ref = Some(value)
    }

    pub fn count(&self) -> usize {
        self.id_to_item.len()
    }
}

impl<V> Default for ItemIndex<V> {
    fn default() -> Self {
        Self {
            id_to_item: IndexMap::new(),
            name_to_ids: HashMap::new()
        }
    }
}