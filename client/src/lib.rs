#![allow(unused_unsafe)]
use lotus_common::{game::game_request::GameRequest, serialization::serializable::Serializable};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn send_data(bytes: &[u8]);
}

#[wasm_bindgen]
pub fn main() {
    let request = GameRequest::Login(String::from("Adius"));
    let bytes = request.serialize();

    unsafe {
        send_data(&bytes);
    }
}