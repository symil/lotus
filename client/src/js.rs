#![allow(unused_unsafe, dead_code)]

use lotus_common::{events::ui_event::UiEvent, graphics::graphics::Cursor, logger::Logger};
use wasm_bindgen::prelude::*;

use crate::draw_primitive::{DrawPrimitive, StringId};

#[wasm_bindgen]
extern {
    pub fn log(message: &str);
    pub fn log_time_start(message: &str);
    pub fn log_time_end(message: &str);

    pub fn poll_event() -> Option<UiEvent>;
    pub fn send_message(bytes: &[u8]);
    pub fn poll_message() -> Option<Vec<u8>>;
    pub fn get_window_width() -> f64;
    pub fn get_window_height() -> f64;
    pub fn set_window_aspect_ratio(aspect_ratio: f64);
    pub fn set_window_title(title: &str);
    pub fn get_string_id(string: &str) -> StringId;

    pub fn clear_canvas();
    pub fn draw(primitive: DrawPrimitive);
    pub fn set_cursor(cursor: Cursor);
    pub fn clear_renderer_cache();
}

pub struct Js;

impl Js {
    pub fn log(message: &str) { unsafe { log(message) } }
    pub fn log_time_start(message: &str) { unsafe { log_time_start(message) } }
    pub fn log_time_end(message: &str) { unsafe { log_time_end(message) } }

    pub fn poll_event() -> Option<UiEvent> { unsafe { poll_event() } }
    pub fn send_message(bytes: &[u8]) { unsafe { send_message(bytes) } }
    pub fn poll_message() -> Option<Vec<u8>> { unsafe { poll_message() } }
    pub fn get_window_width() -> f64 { unsafe { get_window_width() } }
    pub fn get_window_height() -> f64 { unsafe { get_window_height() } }
    pub fn set_window_aspect_ratio(aspect_ratio: f64) { unsafe { set_window_aspect_ratio(aspect_ratio) } }
    pub fn set_window_title(title: &str) { unsafe { set_window_title(title) } }
    pub fn get_string_id(string: &str) -> StringId { unsafe { get_string_id(string) } }
    pub fn clear_canvas() { unsafe { clear_canvas() } }
    pub fn draw(primitive: DrawPrimitive) { unsafe { draw(primitive) } }
    pub fn set_cursor(cursor: Cursor) { unsafe { set_cursor(cursor) } }
    pub fn clear_renderer_cache() { unsafe { clear_renderer_cache() } }
}

pub struct JsLogger;

impl Logger for JsLogger {
    fn log(&self, value: &str) {
        Js::log(value);
    }

    fn log_time_start(&self, label: &str) {
        Js::log_time_start(label);
    }

    fn log_time_end(&self, label: &str) {
        Js::log_time_end(label);
    }
}