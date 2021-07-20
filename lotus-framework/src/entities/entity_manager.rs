use std::collections::HashMap;
use crate::{Entity};

#[derive(Clone, Copy)]
pub enum EventStep {
    Hook = 0,
    Before = 1,
    After = 2
}

const EVENT_STEP_COUNT : usize = 3;

pub type EventCallback = fn(&Entity, &Entity);

static DEFAULT_CALLBACK : EventCallback = |_entity: &Entity, _event: &Entity| { };

pub struct EntityManager {
    pub event_type_to_callbacks: HashMap<u128, EventCallback>,
}

// TODO: `entity_count` and `event_count` can be known at compile time
pub struct EventCallbackTable {
    pub entity_count: usize,
    pub event_count: usize,
    pub callback_table: Vec<EventCallback>,
}

impl EventCallbackTable {
    pub fn new(entity_count: usize, event_count: usize) -> Self {
        let size = entity_count * event_count * EVENT_STEP_COUNT;
        let mut callback_table = Vec::with_capacity(size);

        callback_table.resize(size, DEFAULT_CALLBACK);

        Self { entity_count, event_count, callback_table }
    }

    fn get_index(&self, entity_type_id: usize, event_type_id: usize, step: EventStep) -> usize {
        (entity_type_id * EVENT_STEP_COUNT * self.event_count) + (event_type_id * EVENT_STEP_COUNT) + (step as usize)
    }

    pub fn get(&self, entity_type_id: usize, event_type_id: usize, step: EventStep) -> EventCallback {
        let index = self.get_index(entity_type_id, event_type_id, step);

        self.callback_table[index]
    }

    pub fn set(&mut self, entity_type_id: usize, event_type_id: usize, step: EventStep, callback: EventCallback) {
        let index = self.get_index(entity_type_id, event_type_id, step);

        self.callback_table[index] = callback;
    }
}

// pub struct EventCallback {
//     pub entity: Entity,
//     pub callback: fn(&Entity)
// }