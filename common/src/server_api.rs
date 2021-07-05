use std::cell::RefCell;

pub struct ServerApi {
    players_to_notify: Vec<usize>
}

impl ServerApi {
    pub fn new() -> Self {
        Self {
            players_to_notify: vec![],
        }
    }

    pub fn notify_player_update<P>(&mut self, player: &RefCell<P>) {
        self.players_to_notify.push(player.as_ptr() as usize);
    }

    pub fn drain_players_to_notify(&mut self) -> Vec<usize> {
        self.players_to_notify.drain(..).collect()
    }
}