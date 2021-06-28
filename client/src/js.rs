use lotus_common::{events::Event, serialization::Serializable};
use wasm_bindgen::prelude::*;
use std::fmt::Debug;

use crate::draw_primitive::{DrawPrimitive, StringId};

#[wasm_bindgen]
extern {
    pub fn log(message: &str);
    pub fn poll_event() -> Option<Event>;
    pub fn send_message(bytes: &[u8]);
    pub fn poll_message() -> Option<Vec<u8>>;
    pub fn set_window_aspect_ratio(aspect_ratio: f32);
    pub fn get_window_width() -> f32;
    pub fn get_window_height() -> f32;
    pub fn get_string_id(string: &str) -> StringId;

    pub fn clear_canvas();
    pub fn draw(primitive: DrawPrimitive);
    pub fn clear_renderer_cache();
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

    pub fn set_window_aspect_ratio(aspect_ratio: f32) {
        unsafe { set_window_aspect_ratio(aspect_ratio) };
    }

    pub fn get_window_size() -> (f32, f32) {
        (
            unsafe { get_window_width() },
            unsafe { get_window_height() },
        )
    }

    pub fn get_string_id(string: &str) -> StringId {
        unsafe { get_string_id(string) }
    }

    pub fn clear_canvas() {
        unsafe { clear_canvas() };
    }

    pub fn draw(primitive: DrawPrimitive) {
        unsafe { draw(primitive) };
    }

    pub fn clear_renderer_cache() {
        unsafe { clear_renderer_cache() };
    }
}