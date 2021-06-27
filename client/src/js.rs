use lotus_common::{events::Event, serialization::Serializable};
use wasm_bindgen::prelude::*;
use std::fmt::Debug;

#[wasm_bindgen]
extern {
    pub fn log(message: &str);
    pub fn poll_event() -> Option<Event>;
    pub fn send_message(bytes: &[u8]);
    pub fn poll_message() -> Option<Vec<u8>>;
}

pub struct Js;

impl Js {
    pub fn log<T : Debug>(value: &T) {
        unsafe { log(&format!("{:?}", value)) };
    }

    pub fn poll_event() -> Option<Event> {
        unsafe { poll_event() }
    }

    pub fn send_message<T : Serializable>(value: &T) {
        let bytes = value.serialize();

        unsafe {
            send_message(&bytes);
        }
    }

    pub fn poll_message<T : Serializable>() -> Option<T> {
        match unsafe { poll_message() } {
            None => None,
            Some(bytes) => T::deserialize(&bytes)
        }
    }
}