use wasm_bindgen::prelude::*;
use super::{keyboard_event::KeyboardEvent, mouse_event::MouseEvent, wheel_event::WheelEvent, window_event::WindowEvent};

#[wasm_bindgen]
#[derive(Debug)]
pub struct UiEvent {
    pub window: Option<WindowEvent>,
    pub mouse: Option<MouseEvent>,
    pub keyboard: Option<KeyboardEvent>,
    pub wheel: Option<WheelEvent>
}

#[wasm_bindgen(constructor)]
impl UiEvent {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            window: None,
            mouse: None,
            keyboard: None,
            wheel: None
        }
    }
}