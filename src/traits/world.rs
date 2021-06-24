use crate::server_api::ServerApi;

pub trait World<Player, Request> {
    fn set_api(&mut self, _server_api: ServerApi) {}
    fn on_player_connect(&mut self, _player: &mut Player) {}
    fn on_player_disconnect(&mut self, _player: &mut Player) {}
    fn on_player_request(&mut self, _player: &mut Player, _request: &Request) {}
    fn update(&mut self) {}
}