use crate::traits::{player::Player};

pub struct ServerApi {
    players_to_notify: Vec<u128>
}

impl ServerApi {
    pub fn new() -> Self {
        Self {
            players_to_notify: vec![],
        }
    }

    pub fn notify_player_update<P : Player>(&mut self, player: &P) {
        self.players_to_notify.push(player.get_id());
    }

    pub fn drain_players_to_notify(&mut self) -> Vec<u128> {
        self.players_to_notify.drain(..).collect()
    }
}