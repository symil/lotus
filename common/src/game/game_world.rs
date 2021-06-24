use crate::traits::world::World;
use super::{game_player::GamePlayer, game_request::GameRequest};

pub struct GameWorld {

}

impl GameWorld {
    pub fn new() -> Self {
        Self {}
    }
}

impl World<GamePlayer, GameRequest> for GameWorld {
    fn on_player_connect(&mut self, player: &mut GamePlayer) {
        println!("connected: {}", &player.username);
    }

    fn on_player_disconnect(&mut self, player: &mut GamePlayer) {
        println!("disconnected: {}", &player.username);
    }

    fn on_player_request(&mut self, player: &mut GamePlayer, request: &GameRequest) {
        println!("request from: {}", &player.username);
        println!("{:?}", &request);
    }
}