#![allow(unused_unsafe, unused_imports)]
pub mod js;
pub mod draw_primitive;
pub mod client;
pub mod default_interaction;

use client::Client;
use draw_primitive::DrawPrimitive;
use js::Js;
use lotus_common::{events::Event, game::{game_view::GameView, game_player::GamePlayer, game_request::GameRequest}, graphics::{color::Color, graphics::{Cursor, Font, Shape, TextHorizontalAlign, TextVerticalAlign}, rect::Rect}, serialization::serializable::Serializable, traits::view::View, client_state::ClientState};
use wasm_bindgen::prelude::*;

static mut CLIENT : Option<Client<GamePlayer, GameRequest, GameView>> = None;

#[wasm_bindgen]
pub fn start() {
    unsafe {
        CLIENT = Some(Client::new(1600., 900.));
        CLIENT.as_mut().unwrap().start();
    }
    // send(&GameRequest::Login(String::from("Adius")));
}

// static mut DISPLAYED : bool = false;

#[wasm_bindgen]
pub fn update() {
    unsafe {
        CLIENT.as_mut().unwrap().update();
    }
}