#![allow(unused_variables)]
use crate::server_api::ServerApi;

pub trait World<P, R> {
    fn on_start(&mut self) {}
    fn on_player_connect(&mut self, player: &mut P, api: &mut ServerApi) {}
    fn on_player_disconnect(&mut self, player: &mut P, api: &mut ServerApi) {}
    fn on_player_request(&mut self, player: &mut P, request: &R, api: &mut ServerApi) {}
    fn update(&mut self) {}
}