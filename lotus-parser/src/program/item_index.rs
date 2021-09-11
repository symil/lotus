use std::{borrow::Borrow, cell::Ref, collections::HashMap, hash::Hash, mem::take};
use indexmap::IndexMap;
use parsable::DataLocation;
use crate::{items::{Identifier, Visibility}, utils::Link};

#[derive(Debug)]
pub struct ItemIndex<V> {
    pub items_by_id: IndexMap<u64, Link<V>>,
    pub items_by_name: HashMap<String, Vec<Link<V>>>
}

pub trait GlobalItem {
    fn get_name(&self) -> &Identifier;
    fn get_visibility(&self) -> Visibility;
}

impl<V : GlobalItem> ItemIndex<V> {
    pub fn len(&self) -> usize {
        self.items_by_name.len()
    }

    pub fn insert(&mut self, value: V) {
        let name = value.get_name();
        let id = name.location.get_hash();
        let item = Link::new(value);

        self.items_by_id.insert(id, item.clone());

        if let Some(vec) = self.items_by_name.get_mut(name.as_str()) {
            vec.push(item);
        } else {
            self.items_by_name.insert(name.to_string(), vec![item]);
        }
    }

    pub fn get_by_name(&self, getter_name: &Identifier) -> Option<&Link<V>> {
        let candidates = self.items_by_name.get(getter_name.as_str())?;
        let getter_location : &DataLocation = &getter_name.location;

        for item in candidates.iter() {
            let item_name = item.borrow().get_name();
            let item_location = &item_name.location;
            let ok = match item.borrow().get_visibility() {
                Visibility::Private => item_location.file_namespace == getter_location.file_namespace && item_location.file_name == getter_location.file_name,
                Visibility::Public => item_location.file_namespace == getter_location.file_namespace,
                Visibility::Export => true,
                Visibility::System => item_location.file_namespace == getter_location.file_namespace,
                Visibility::Member => false,
            };

            if ok {
                return Some(&item);
            }
        }

        None
    }

    pub fn ref_by_name(&self, getter_name: &Identifier) -> Option<Ref<V>> {
        self.get_by_name(getter_name).and_then(|link| Some(link.borrow()))
    }

    pub fn get_by_location(&self, value_name: &Identifier) -> &Link<V> {
        self.items_by_id.get(&value_name.location.get_hash()).unwrap()
    }
}

impl<V> Default for ItemIndex<V> {
    fn default() -> Self {
        Self {
            items_by_id: IndexMap::new(),
            items_by_name: HashMap::new(),
        }
    }
}