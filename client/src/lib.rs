#![allow(unused_unsafe)]
use lotus_common::{game::{game_view::GameView, game_player::GamePlayer, game_request::GameRequest}, graphics::rect::Rect, serialization::serializable::Serializable, state_message::StateMessage, traits::view::View, view_context::ViewContext};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn send_message(bytes: &[u8]);
    pub fn read_message() -> Option<Vec<u8>>;
    pub fn log(message: &str);
    // pub fn draw_graphics(g: &Graphics);
}

fn send<T : Serializable>(value: &T) {
    let bytes = value.serialize();

    unsafe {
        send_message(&bytes);
    }
}

fn receive<T : Serializable>() -> Option<T> {
    match unsafe { read_message() } {
        None => None,
        Some(bytes) => T::deserialize(&bytes)
    }
}

#[wasm_bindgen]
pub fn start() {
    send(&GameRequest::Login(String::from("Adius")));
}

#[wasm_bindgen]
pub fn update() {
    let message : StateMessage<GamePlayer, GameView> = match receive() {
        None => return,
        Some(message) => message
    };

    let player = message.player;
    let context = ViewContext {
        rect: Rect::default(),
        pov: &player,
        hovered: None
    };

    let graphics = message.ui.render(&context);
    let string = format!("UI: {:?}", graphics);

    unsafe { log(&string) };
}