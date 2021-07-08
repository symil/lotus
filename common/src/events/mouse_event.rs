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
            action: MouseAction::Move,
            button: MouseButton::Left,
            x: 0.,
            y: 0.,
        }
    }
}

impl MouseEvent {
    pub fn is_click(&self) -> bool {
        self.action == MouseAction::Click
    }
}

#[as_js_string(lowercase)]
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MouseAction {
    Move,
    Down,
    Click,
    Up
}

#[as_js_string(lowercase)]
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MouseButton {
    Left,
    Middle,
    Right
}