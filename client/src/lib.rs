#![allow(unused_unsafe)]
use lotus_common::{client_api::ClientApi, game::{game_entity::GameEntity, game_player::GamePlayer, game_request::GameRequest}, serialization::serializable::Serializable, state_message::StateMessage, traits::entity::Entity};
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
    let message = match receive::<StateMessage<GamePlayer, GameEntity>>() {
        None => return,
        Some(message) => message
    };

    let client_api = ClientApi {
        pov: message.player
    };

    let graphics = message.ui.render(&client_api);
    let string = format!("UI: {:?}", graphics);

    unsafe { log(&string) };
}