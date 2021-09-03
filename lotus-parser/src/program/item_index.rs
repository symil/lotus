use std::{collections::HashMap, hash::Hash};
use parsable::DataLocation;
use crate::items::Identifier;

use super::{Id, ItemVisibility};

#[derive(Debug)]
pub struct ItemIndex<V> {
    pub id_to_item: HashMap<u64, V>,
    pub name_to_ids: HashMap<String, Vec<u64>>
}

pub trait GlobalItem {
    fn get_id(&self) -> u64;
    fn get_name(&self) -> &str;
    fn get_location(&self) -> &DataLocation;
    fn get_visibility(&self) -> ItemVisibility;
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

        self.id_to_item.insert(id, value);
    }

    pub fn get_by_name(&self, name: &Identifier) -> Option<&V> {
        let candidates = self.name_to_ids.get(name.as_str())?;
        let getter_location : &DataLocation = &name.location;

        for id in candidates.iter() {
            let value = self.id_to_item.get(id).unwrap();
            let location = value.get_location();
            let ok = match value.get_visibility() {
                ItemVisibility::Private => location.file_namespace == getter_location.file_namespace && location.file_name == getter_location.file_name,
                ItemVisibility::Public => location.file_namespace == getter_location.file_namespace,
                ItemVisibility::Export => true,
                ItemVisibility::System => location.file_namespace == getter_location.file_namespace,
            };

            if ok {
                return Some(value);
            }
        }

        None
    }

    pub fn get_by_id(&self, id: u64) -> Option<&V> {
        self.id_to_item.get(&id)
    }

    pub fn get_mut_by_id(&mut self, id: u64) -> Option<&mut V> {
        self.id_to_item.get_mut(&id)
    }
}

impl<V> Default for ItemIndex<V> {
    fn default() -> Self {
        Self {
            id_to_item: HashMap::new(),
            name_to_ids: HashMap::new()
        }
    }
}