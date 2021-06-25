use crate::{server_api::ServerApi, traits::world::World};
use super::{game_entity::GameEntity, game_player::GamePlayer, game_request::GameRequest};

pub struct GameWorld {

}

impl GameWorld {
    pub fn new() -> Self {
        Self {}
    }
}

impl World<GamePlayer, GameRequest, GameEntity> for GameWorld {
    fn on_player_connect(&mut self, player: &mut GamePlayer, api: &mut ServerApi<GamePlayer, GameEntity>) {
        println!("connected: {}", &player.username);

        let ui = GameEntity { };
        api.set_player_ui(player, ui);
    }

    fn on_player_disconnect(&mut self, player: &mut GamePlayer, _api: &mut ServerApi<GamePlayer, GameEntity>) {
        println!("disconnected: {}", &player.username);
    }

    fn on_player_request(&mut self, player: &mut GamePlayer, request: &GameRequest, _api: &mut ServerApi<GamePlayer, GameEntity>) {
        println!("request from: {}", &player.username);
        println!("{:?}", &request);
    }
}