use std::{cell::RefCell, collections::HashMap, mem::take, time::Instant};

pub struct ServerState<E> {
    clock: Instant,
    outgoing_messages: HashMap<usize, Vec<E>>,
}

impl<E> ServerState<E> {
    pub fn new() -> Self {
        Self {
            clock: Instant::now(),
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

    pub fn get_current_time(&self) -> u128 {
        self.clock.elapsed().as_millis()
    }
}