use crate::server_api::ServerApi;

pub trait World<P, R, E> {
    fn on_player_connect(&mut self, _player: &mut P, _server_api: &mut ServerApi<P, E>) {}
    fn on_player_disconnect(&mut self, _player: &mut P, _server_api: &mut ServerApi<P, E>) {}
    fn on_player_request(&mut self, _player: &mut P, _request: &R, _server_api: &mut ServerApi<P, E>) {}
    fn update(&mut self) {}
}