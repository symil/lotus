#![allow(unused_unsafe)]
use lotus_common::{game::game_request::GameRequest, traits::request::Request};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn send_data(bytes: &[u8]);
}

#[wasm_bindgen]
pub fn main() {
    // let request = GameRequest::Login(String::from("Adius"));
    // let bytes = GameRequest::serialize(&request);

    // unsafe {
    //     send_data(&bytes);
    // }
}