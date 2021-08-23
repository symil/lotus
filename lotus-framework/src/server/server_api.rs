use std::{collections::HashMap, mem::take, time::Instant};

use crate::Id;

pub struct ServerApi<E> {
    clock: Instant,
    outgoing_messages: HashMap<Id, Vec<E>>,
}

impl<E> ServerApi<E> {
    pub fn new() -> Self {
        Self {
            clock: Instant::now(),
            outgoing_messages: HashMap::new(),
        }
    }

    fn retrieve_user_events(&mut self, id: Id) -> &mut Vec<E> {
        if !self.outgoing_messages.contains_key(&id) {
            self.outgoing_messages.insert(id, vec![]);
        }

        self.outgoing_messages.get_mut(&id).unwrap()
    }

    pub fn notify_update(&mut self, id: Id) {
        self.retrieve_user_events(id);
    }

    pub fn notify_event(&mut self, id: Id, event: E) {
        self.retrieve_user_events(id).push(event);
    }

    pub fn get_current_time(&self) -> f64 {
        self.clock.elapsed().as_millis() as f64 / 1000.
    }

    pub(crate) fn poll_outgoing_messages(&mut self) -> HashMap<Id, Vec<E>> {
        take(&mut self.outgoing_messages)
    }
}