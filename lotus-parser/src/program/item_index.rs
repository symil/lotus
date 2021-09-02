use std::{collections::HashMap, hash::Hash};
use crate::items::{Identifier, VisibilityToken};
use super::{Id, ItemMetadata};

#[derive(Debug)]
pub struct ItemIndex<V> {
    pub id_to_item: HashMap<Id, V>,
    pub name_to_ids: HashMap<String, Vec<Id>>
}

pub trait WithMetadata {
    fn get_metadata(&self) -> &ItemMetadata;
}

impl<V : WithMetadata> ItemIndex<V> {
    pub fn len(&self) -> usize {
        self.id_to_item.len()
    }

    pub fn insert(&mut self, value: V) {
        let metadata = value.get_metadata();
        let id = metadata.id;
        let name = &metadata.name;

        if let Some(vec) = self.name_to_ids.get_mut(name.as_str()) {
            vec.push(id);
        } else {
            self.name_to_ids.insert(name.to_string(), vec![id]);
        }

        self.id_to_item.insert(id, value);
    }

    pub fn get_by_name(&self, name: &Identifier, from_file_name: &str, from_namespace: &str) -> Option<&V> {
        let candidates = self.name_to_ids.get(name.as_str())?;

        for id in candidates.iter() {
            let value = self.id_to_item.get(id).unwrap();
            let metadata = value.get_metadata();
            let ok = match &metadata.visibility {
                VisibilityToken::Private => metadata.namespace == from_namespace && metadata.file_name == from_file_name,
                VisibilityToken::Public => metadata.namespace == from_namespace,
                VisibilityToken::Export => true,
                VisibilityToken::System => metadata.namespace == from_namespace,
            };

            if ok {
                return Some(value);
            }
        }

        None
    }

    pub fn get_by_id(&self, id: Id) -> Option<&V> {
        self.id_to_item.get(&id)
    }

    pub fn get_mut_by_id(&mut self, id: Id) -> Option<&mut V> {
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