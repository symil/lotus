use crate::{Entity, Link};

pub struct EntityField {
    pub entities: Vec<Link<Entity>>
}

impl EntityField {
    pub fn new() -> Self {
        Self {
            entities: vec![]
        }
    }

    pub fn remove(&mut self, value: Link<Entity>) {
        let index = self.entities.iter().enumerate().find_map(|(i, entity)| {
            if entity.get_addr() == value.get_addr() {
                Some(i)
            } else {
                None
            }
        });

        if let Some(index) = index {
            self.entities.remove(index);
        }
    }

    pub fn add(&mut self, value: Link<Entity>) {
        self.entities.push(value);
    }

    pub fn is_null(&self) -> bool {
        self.entities.is_empty()
    }
}