#![allow(unused_unsafe)]
use lotus_common::{game::{game_entity::GameEntity, game_request::GameRequest}, serialization::serializable::Serializable};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn send_message(bytes: &[u8]);
    pub fn read_message() -> Option<Vec<u8>>;
    pub fn log(message: &str);
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
    let mut new_ui = None;

    loop {
        match receive::<GameEntity>() {
            None => break,
            Some(ui) => new_ui = Some(ui)
        }
    }

    if let Some(ui) = new_ui {
        let string = format!("UI: {:?}", ui);

        unsafe { log(&string) };
    }
}