#![allow(unused_unsafe)]
use lotus_common::{game::{game_view::GameView, game_player::GamePlayer, game_request::GameRequest}, graphics::rect::Rect, serialization::serializable::Serializable, traits::view::View, view_context::ViewContext};
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
    let player : GamePlayer = match receive() {
        None => return,
        Some(player) => player
    };

    let context = ViewContext {
        rect: Rect::default(),
        pov: &player,
        hovered: None
    };

    let ui = GameView::root();
    let graphics = ui.render(&context);
    let string = format!("UI: {:?}", graphics);

    unsafe { log(&string) };
}