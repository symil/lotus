use wasm_bindgen::prelude::*;
use enum_as_string_macro::*;

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

    pub fn is_down(&self) -> bool {
        self.action == MouseAction::Down
    }

    pub fn is_up(&self) -> bool {
        self.action == MouseAction::Up
    }

    pub fn is_left(&self) -> bool {
        self.button == MouseButton::Left
    }

    pub fn is_right(&self) -> bool {
        self.button == MouseButton::Right
    }

    pub fn is_middle(&self) -> bool {
        self.button == MouseButton::Middle
    }

    pub fn xy(&self) -> (f64, f64) {
        (self.x, self.y)
    }
}

#[enum_as_string(discriminant, lowercase)]
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MouseAction {
    Move,
    Down,
    Click,
    Up
}

#[enum_as_string(discriminant, lowercase)]
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MouseButton {
    Left,
    Middle,
    Right
}