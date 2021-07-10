use wasm_bindgen::prelude::*;
use as_js_string_macro::*;

#[as_js_string(lowercase)]
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum WindowEvent {
    Resize
}