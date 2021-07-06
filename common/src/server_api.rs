use std::{cell::RefCell, collections::HashMap, mem::take};

pub struct ServerApi<E> {
    outgoing_messages: HashMap<usize, Vec<E>>
}

impl<E> ServerApi<E> {
    pub fn new() -> Self {
        Self {
            outgoing_messages: HashMap::new(),
        }
    }

    fn retrieve_player_events<P>(&mut self, player: &RefCell<P>) -> &mut Vec<E> {
        let id = player.as_ptr() as usize;

        if !self.outgoing_messages.contains_key(&id) {
            self.outgoing_messages.insert(id, vec![]);
        }

        self.outgoing_messages.get_mut(&id).unwrap()
    }

    pub fn notify_state_update<P>(&mut self, player: &RefCell<P>) {
        self.retrieve_player_events(player);
    }

    pub fn notify_event<P>(&mut self, player: &RefCell<P>, event: E) {
        self.retrieve_player_events(player).push(event);
    }

    pub fn poll_outgoing_messages(&mut self) -> HashMap<usize, Vec<E>> {
        take(&mut self.outgoing_messages)
    }
}