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

    pub fn insert(&mut self, value: V) -> Link<V> {
        let id = value.get_name().location.get_hash();
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

    pub fn get_by_name(&self, getter_name: &Identifier) -> Option<Link<V>> {
        let candidates = self.items_by_name.get(getter_name.as_str())?;
        let getter_location : &DataLocation = &getter_name.location;

        for item_wrapped in candidates.iter() {
            let ok = item_wrapped.with_ref(|item| {
                let item_name = item.get_name();
                let item_location = &item_name.location;
                match item.get_visibility() {
                    Visibility::Private => item_location.file_namespace == getter_location.file_namespace && item_location.file_name == getter_location.file_name,
                    Visibility::Public => item_location.file_namespace == getter_location.file_namespace,
                    Visibility::Export => true,
                    Visibility::System => item_location.file_namespace == getter_location.file_namespace,
                    Visibility::Member => false,
                }
            });

            if ok {
                return Some(item_wrapped.clone());
            }
        }

        None
    }

    pub fn get_by_location(&self, value_name: &Identifier) -> Link<V> {
        self.items_by_id.get(&value_name.location.get_hash()).unwrap().clone()
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