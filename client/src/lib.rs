#![allow(unused_unsafe, unused_imports)]
pub mod js;

use js::Js;
use lotus_common::{events::Event, game::{game_view::GameView, game_player::GamePlayer, game_request::GameRequest}, graphics::{graphics::Cursor, rect::Rect}, serialization::serializable::Serializable, traits::view::View, view_context::ViewContext};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn start() {
    // unsafe { log_enum(Cursor::Pointer) };
    // send(&GameRequest::Login(String::from("Adius")));
}

#[wasm_bindgen]
pub fn update() {
    while let Some(_player) = Js::poll_message::<GamePlayer>() {
        // let context = ViewContext {
        //     rect: Rect::default(),
        //     pov: &player,
        //     hovered: None
        // };

        // let ui = GameView::root();
        // let graphics = ui.render(&context);
        // let string = format!("UI: {:?}", graphics);

        // unsafe { log(&string) };
    }

    while let Some(event) = Js::poll_event() {
        Js::log(&event);
    }
}