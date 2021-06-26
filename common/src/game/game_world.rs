use crate::{server_api::ServerApi, traits::world::World};
use super::{game_player::GamePlayer, game_request::GameRequest, game_view::GameView};

pub struct GameWorld {

}

impl GameWorld {
    pub fn new() -> Self {
        Self {}
    }
}

impl World<GamePlayer, GameRequest, GameView> for GameWorld {
    fn on_player_connect(&mut self, player: &mut GamePlayer, api: &mut ServerApi<GamePlayer, GameView>) {
        println!("connected: {}", &player.username);

        let ui = GameView { };
        api.set_player_ui(player, ui);
    }

    fn on_player_disconnect(&mut self, player: &mut GamePlayer, _api: &mut ServerApi<GamePlayer, GameView>) {
        println!("disconnected: {}", &player.username);
    }

    fn on_player_request(&mut self, player: &mut GamePlayer, request: &GameRequest, _api: &mut ServerApi<GamePlayer, GameView>) {
        println!("request from: {}", &player.username);
        println!("{:?}", &request);
    }
}