use wasm_bindgen::prelude::*;
use enum_as_string_macro::*;

#[enum_as_string(lowercase)]
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum WindowEvent {
    Resize
}