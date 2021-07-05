use wasm_bindgen::prelude::*;
use lotus_as_js_string_macro::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct MouseEvent {
    pub action: MouseAction,
    pub button: MouseButton,
    pub x: f64,
    pub y: f64
}

#[wasm_bindgen(constructor)]
impl MouseEvent {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            action: MouseAction::None,
            button: MouseButton::None,
            x: 0.,
            y: 0.,
        }
    }
}

#[as_js_string(lowercase)]
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum MouseAction {
    None,
    Move,
    Down,
    Up
}

#[as_js_string(lowercase)]
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum MouseButton {
    None,
    Left,
    Middle,
    Right
}