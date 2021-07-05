#![allow(unused_unsafe)]

use lotus_common::{events::Event, graphics::graphics::Cursor, serialization::Serializable};
use wasm_bindgen::prelude::*;

use crate::draw_primitive::{DrawPrimitive, StringId};

#[wasm_bindgen]
extern {
    pub fn log(message: &str);
    pub fn poll_event() -> Option<Event>;
    pub fn send_message(bytes: &[u8]);
    pub fn poll_message() -> Option<Vec<u8>>;
    pub fn set_window_aspect_ratio(aspect_ratio: f64);
    pub fn get_window_width() -> f64;
    pub fn get_window_height() -> f64;
    pub fn get_string_id(string: &str) -> StringId;

    pub fn clear_canvas();
    pub fn draw(primitive: DrawPrimitive);
    pub fn set_cursor(cursor: Cursor);
    pub fn clear_renderer_cache();
}

pub struct Js;

impl Js {
    pub fn log(message: &str) {
        unsafe { log(message) };
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

    pub fn set_window_aspect_ratio(aspect_ratio: f64) {
        unsafe { set_window_aspect_ratio(aspect_ratio) };
    }

    pub fn get_window_size() -> (f64, f64) {
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

    pub fn set_cursor(cursor: Cursor) {
        unsafe { set_cursor(cursor) };
    }

    pub fn clear_renderer_cache() {
        unsafe { clear_renderer_cache() };
    }
}